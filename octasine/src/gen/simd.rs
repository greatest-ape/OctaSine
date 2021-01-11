use duplicate::duplicate;
use vst::buffer::AudioBuffer;

use crate::OctaSine;
use crate::common::*;
use crate::constants::*;
use crate::parameters::processing::parameters::*;

use super::common::*;


/// Each SAMPLE_PASS_SIZE samples, load parameter changes and processing
/// parameter values (interpolated values where applicable)
const SAMPLE_PASS_SIZE: usize = 16;


pub trait AudioGen {
    unsafe fn process_f32(
        octasine: &mut OctaSine,
        audio_buffer: &mut AudioBuffer<f32>,
    );
}


pub struct Fallback;
pub struct Sse2;
pub struct Avx;


#[inline]
pub fn process_f32_runtime_select(
    octasine: &mut OctaSine,
    audio_buffer: &mut AudioBuffer<f32>,
){
    unsafe {
        #[cfg(target_arch = "x86_64")]
        if is_x86_feature_detected!("avx") {
            Avx::process_f32(octasine, audio_buffer);
        } else {
            // SSE2 is always supported on x86_64
            Sse2::process_f32(octasine, audio_buffer);
        }

        #[cfg(not(target_arch = "x86_64"))]
        Fallback::process_f32(octasine, audio_buffer);
    }
}


#[duplicate(
    [
        instruction_set [ Fallback ]
        target_feature_enable [ cfg(not(feature = "fake-feature")) ]
        pd  [ [f64; 2] ]
        pd_width [ 2 ] 
        pd_set1 [ (|v| [v, v]) ]
        pd_loadu [ (|source: *const f64| *(source as *const [f64; 2])) ]
        pd_storeu [ (|target: *mut f64, v: [f64; 2]| {
            ::std::ptr::write(target as *mut [f64; 2], v);
        }) ]
        pd_add [ (|[a1, a2]: [f64; 2], [b1, b2]: [f64; 2]|
            [a1 + b1, a2 + b2]
        ) ]
        pd_sub [ (|[a1, a2]: [f64; 2], [b1, b2]: [f64; 2]|
            [a1 - b1, a2 - b2]
        ) ]
        pd_mul [ (|[a1, a2]: [f64; 2], [b1, b2]: [f64; 2]|
            [a1 * b1, a2 * b2]
        ) ]
        pd_min [ (|[a1, a2]: [f64; 2], [b1, b2]: [f64; 2]|
            [a1.min(b1), a2.min(b2)]
        ) ]
        pd_max [ (|[a1, a2]: [f64; 2], [b1, b2]: [f64; 2]|
            [a1.max(b1), a2.max(b2)]
        ) ]
        pd_fast_sin [ (|[a1, a2]: [f64; 2]| [
            sleef_sys::Sleef_sin_u35(a1),
            sleef_sys::Sleef_sin_u35(a2),
        ]) ]
        pd_gt [ (|[a1, a2]: [f64; 2], [b1, b2]: [f64; 2]| [
            (a1 > b1) as u64 as f64,
            (a2 > b2) as u64 as f64
        ]) ]
        pd_mod_input_panning [ (|[l, r]: [f64; 2]| [l + r, l + r]) ]
    ]
    [
        instruction_set [ Sse2 ]
        target_feature_enable [ target_feature(enable = "sse2") ]
        pd  [ __m128d ]
        pd_width [ 2 ] 
        pd_set1 [ _mm_set1_pd ]
        pd_loadu [ _mm_loadu_pd ]
        pd_storeu [ _mm_storeu_pd ]
        pd_add [ _mm_add_pd ]
        pd_sub [ _mm_sub_pd ]
        pd_mul [ _mm_mul_pd ]
        pd_min [ _mm_min_pd ]
        pd_max [ _mm_max_pd ]
        pd_fast_sin [ sleef_sys::Sleef_sind2_u35sse2 ]
        pd_gt [ (|a, b| _mm_cmpgt_pd(a, b))]
        pd_mod_input_panning [ (|mod_in| {
            _mm_add_pd(mod_in, _mm_shuffle_pd(mod_in, mod_in, 0b01))
        }) ]
    ]
    [
        instruction_set [ Avx ]
        target_feature_enable [ target_feature(enable = "avx") ]
        pd  [ __m256d ]
        pd_width [ 4 ] 
        pd_set1 [ _mm256_set1_pd ]
        pd_loadu [ _mm256_loadu_pd ]
        pd_storeu [ _mm256_storeu_pd ]
        pd_add [ _mm256_add_pd ]
        pd_sub [ _mm256_sub_pd ]
        pd_mul [ _mm256_mul_pd ]
        pd_min [ _mm256_min_pd ]
        pd_max [ _mm256_max_pd ]
        pd_fast_sin [ sleef_sys::Sleef_sind4_u35avx ]
        pd_gt [ (|a, b| _mm256_cmp_pd(a, b, _CMP_GT_OQ))]
        pd_mod_input_panning [ (|mod_in| {
            _mm256_add_pd(mod_in, _mm256_permute_pd(mod_in, 0b0101))
        }) ]
    ]
)]
mod gen {
    #[allow(unused_imports)]
    use core::arch::x86_64::*;

    use super::*;

    impl AudioGen for instruction_set {
        #[target_feature_enable]
        unsafe fn process_f32(
            octasine: &mut OctaSine,
            audio_buffer: &mut AudioBuffer<f32>,
        ){
            let num_samples = audio_buffer.samples();

            let mut audio_buffer_outputs = audio_buffer.split().1;
            let mut audio_buffer_lefts = audio_buffer_outputs.get_mut(0);
            let mut audio_buffer_rights = audio_buffer_outputs.get_mut(1);
    
            let num_passes = num_samples / SAMPLE_PASS_SIZE;

            for pass_index in 0..num_passes { 
                run_pass(
                    octasine,
                    &mut audio_buffer_lefts,
                    &mut audio_buffer_rights,
                    pass_index,
                )
            }
        }
    }

    #[target_feature_enable]
    unsafe fn run_pass(
        octasine: &mut OctaSine,
        audio_buffer_lefts: &mut [f32],
        audio_buffer_rights: &mut [f32],
        pass_index: usize,
    ){
        // --- Update processing parameters from preset parameters

        let changed_preset_parameters = octasine.sync.presets
            .get_changed_parameters_from_processing();

        if let Some(indeces) = changed_preset_parameters {
            for (index, opt_new_value) in indeces.iter().enumerate(){
                if let Some(new_value) = opt_new_value {
                    octasine.processing.parameters.set_from_sync(index, *new_value);
                }
            }
        }

        // --- Set some generally useful variables

        let bpm = octasine.get_bpm();

        let operators = &mut octasine.processing.parameters.operators;

        let time_per_sample = octasine.processing.time_per_sample;
        let time = octasine.processing.global_time;
        let time_advancement = time_per_sample.0 * (SAMPLE_PASS_SIZE as f64);

        // Necessary for interpolation
        octasine.processing.global_time.0 += time_advancement;

        // --- Collect parameter data and do audio generation

        // FIXME: optimize section, possibly with simd. Maybe envelopes can be calculated less often

        // Maybe operator indexes should be inversed (3 - operator_index)
        // because that is how they will be accessed later.

        // Sample pass size * 2 because of two channels. Even index = left channel
        let mut summed_additive_outputs = [0.0f64; SAMPLE_PASS_SIZE * 2];

        let zero_value_limit_splat = pd_set1(ZERO_VALUE_LIMIT);

        for voice in octasine.processing.voices.iter_mut().filter(|voice| voice.active){
            // --- Get voice data

            let lfo_values = get_lfo_target_values(
                &mut octasine.processing.parameters.lfos,
                &mut voice.lfos,
                time,
                time_advancement,
                bpm
            );

            let voice_base_frequency = voice.midi_pitch.get_frequency(
                octasine.processing.parameters.master_frequency.get_value_with_lfo_addition(
                    (),
                    lfo_values.get(LfoTargetParameter::Master(LfoTargetMasterParameter::Frequency))
                )
            );

            // Interpolated
            let mut operator_volume: [f64; 4] = [0.0; 4];
            let mut operator_modulation_index = [0.0f64; 4];
            let mut operator_feedback: [f64; 4] = [0.0; 4];
            let mut operator_panning: [f64; 4] = [0.0; 4];
            let mut operator_additive: [f64; 4] = [0.0; 4];
            
            // Not interpolated
            let mut operator_wave_type = [WaveType::Sine; 4];
            let mut operator_frequency: [f64; 4] = [voice_base_frequency; 4]; 
            let mut operator_modulation_targets = [0usize; 4];

            let mut operator_envelope_volumes = [[0.0f64; SAMPLE_PASS_SIZE * 2]; 4];
            let mut operator_phases = [[0.0f64; SAMPLE_PASS_SIZE * 2]; 4];

            for (index, operator) in operators.iter_mut().enumerate(){
                operator_volume[index] = operator.volume.get_value_with_lfo_addition(
                    time,
                    lfo_values.get(LfoTargetParameter::Operator(index, LfoTargetOperatorParameter::Volume))
                );
                operator_modulation_index[index] = operator.modulation_index.get_value_with_lfo_addition(
                    time,
                    lfo_values.get(LfoTargetParameter::Operator(index, LfoTargetOperatorParameter::ModulationIndex))
                );
                operator_feedback[index] = operator.feedback.get_value_with_lfo_addition(
                    time,
                    lfo_values.get(LfoTargetParameter::Operator(index, LfoTargetOperatorParameter::Feedback))
                );
                operator_panning[index] = operator.panning.get_value_with_lfo_addition(
                    time,
                    lfo_values.get(LfoTargetParameter::Operator(index, LfoTargetOperatorParameter::Panning))
                );

                // Get additive factor; use 1.0 for operator 1
                operator_additive[index] = if index == 0 {
                    1.0
                } else {
                    operator.additive_factor.get_value_with_lfo_addition(
                        time,
                        lfo_values.get(LfoTargetParameter::Operator(index, LfoTargetOperatorParameter::Additive))
                    )
                };

                operator_wave_type[index] = operator.wave_type.value;

                let frequency_ratio = operator.frequency_ratio.get_value_with_lfo_addition(
                    (),
                    lfo_values.get(LfoTargetParameter::Operator(index, LfoTargetOperatorParameter::FrequencyRatio))
                );
                let frequency_free = operator.frequency_free.get_value_with_lfo_addition(
                    (),
                    lfo_values.get(LfoTargetParameter::Operator(index, LfoTargetOperatorParameter::FrequencyFree))
                );
                let frequency_fine = operator.frequency_fine.get_value_with_lfo_addition(
                    (),
                    lfo_values.get(LfoTargetParameter::Operator(index, LfoTargetOperatorParameter::FrequencyFine))
                );

                operator_frequency[index] *= frequency_ratio *
                    frequency_free *
                    frequency_fine;

                if let Some(p) = &mut operator.output_operator {
                    use ProcessingParameterOperatorModulationTarget::*;

                    let opt_value = match p {
                        OperatorIndex2(p) => Some(p.get_value(())),
                        OperatorIndex3(p) => Some(p.get_value(())),
                    };

                    if let Some(value) = opt_value {
                        operator_modulation_targets[index] = value;
                    }
                }
            }

            // Envelope
            for i in 0..SAMPLE_PASS_SIZE {
                for (operator_index, operator) in operators.iter_mut().enumerate(){
                    let v = voice.operators[operator_index].volume_envelope.get_volume(
                        &octasine.processing.log10_table,
                        &operator.volume_envelope,
                        voice.key_pressed,
                        voice.duration
                    );

                    let j = i * 2;

                    operator_envelope_volumes[operator_index][j] = v;
                    operator_envelope_volumes[operator_index][j + 1] = v;
                }

                voice.duration.0 += time_per_sample.0;
            }

            // Phase
            for operator_index in 0..4 {
                let last_phase = voice.operators[operator_index].last_phase.0;
                let frequency = operator_frequency[operator_index];
                let phase_addition = frequency * time_per_sample.0;

                let mut new_phase = 0.0;

                for i in 0..SAMPLE_PASS_SIZE {
                    // Do multiplication instead of successive addition for less precision loss (hopefully)
                    new_phase = last_phase + phase_addition * ((i + 1) as f64);

                    let j = i * 2;

                    operator_phases[operator_index][j] = new_phase;
                    operator_phases[operator_index][j + 1] = new_phase;
                }

                // Save phase
                voice.operators[operator_index].last_phase.0 = new_phase;
            }

            let voice_volume_factor_splat = {
                let lfo_parameter = LfoTargetParameter::Master(
                    LfoTargetMasterParameter::Volume
                );
                let lfo_addition = lfo_values.get(lfo_parameter);

                let master_volume = octasine.processing
                    .parameters
                    .master_volume
                    .get_value_with_lfo_addition(time, lfo_addition);

                let key_velocity = voice.key_velocity.0;

                pd_set1(VOICE_VOLUME_FACTOR * master_volume * key_velocity)
            };

            let operator_generate_audio = run_operator_dependency_analysis(
                operator_volume,
                operator_additive,
                operator_modulation_index,
                operator_wave_type,
                operator_modulation_targets,
            );

            voice.deactivate_if_envelopes_ended();

            // --- Generate samples for all operators

            // Voice modulation input storage, indexed by operator
            let mut voice_modulation_inputs = [[0.0f64; SAMPLE_PASS_SIZE * 2]; 4];

            // Go through operators downwards, starting with operator 4
            for operator_index in 0..4 { // FIXME: better iterator with 3, 2, 1, 0 possible?
                let operator_index = 3 - operator_index;

                // Possibly skip generation based on previous dependency analysis
                if !operator_generate_audio[operator_index]{
                    continue;
                }
                
                if operator_wave_type[operator_index] == WaveType::WhiteNoise {
                    let random_numbers = {
                        let mut random_numbers = [0.0f64; SAMPLE_PASS_SIZE * 2];

                        for i in 0..SAMPLE_PASS_SIZE {
                            let random = (octasine.processing.rng.f64() - 0.5) * 2.0;

                            let j = i * 2;

                            random_numbers[j] = random;
                            random_numbers[j + 1] = random;
                        }

                        random_numbers
                    };

                    let modulation_target = operator_modulation_targets[operator_index];

                    let constant_power_panning = {
                        let mut data = [0.0f64; 8];

                        let left_and_right = operators[operator_index].panning.left_and_right;
                        
                        for (i, v) in data.iter_mut().enumerate() {
                            *v = left_and_right[i % 2];
                        }

                        pd_loadu(&data[0])
                    };

                    let operator_volume_splat = pd_set1(operator_volume[operator_index]);
                    let operator_additive_splat = pd_set1(operator_additive[operator_index]);

                    for i in (0..SAMPLE_PASS_SIZE * 2).step_by(pd_width){
                        let envelope_volume = pd_loadu(&operator_envelope_volumes[operator_index][i]);
                        let volume_product = pd_mul(operator_volume_splat, envelope_volume);

                        let sample = pd_loadu(&random_numbers[i]);

                        let sample_adjusted = pd_mul(pd_mul(sample, volume_product), constant_power_panning);
                        let additive_out = pd_mul(sample_adjusted, operator_additive_splat);
                        let modulation_out = pd_sub(sample_adjusted, additive_out);

                        // Add modulation output to target operator's modulation inputs
                        let modulation_sum = pd_add(pd_loadu(&voice_modulation_inputs[modulation_target][i]), modulation_out);
                        pd_storeu(&mut voice_modulation_inputs[modulation_target][i], modulation_sum);

                        // Add additive output to summed_additive_outputs
                        let summed_plus_new = pd_add(
                            pd_loadu(&summed_additive_outputs[i]),
                            pd_mul(additive_out, voice_volume_factor_splat)
                        );
                        pd_storeu(
                            &mut summed_additive_outputs[i],
                            summed_plus_new
                        );
                    }
                } else {
                    // --- Setup operator SIMD vars

                    let operator_volume_splat = pd_set1(operator_volume[operator_index]);
                    let operator_feedback_splat = pd_set1(operator_feedback[operator_index]);
                    let operator_additive_splat = pd_set1(operator_additive[operator_index]);
                    let operator_modulation_index_splat = pd_set1(operator_modulation_index[operator_index]);

                    let (pan_tendency, one_minus_pan_tendency) = {
                        // Get panning as value between -1 and 1
                        let pan_transformed = 2.0 * (operator_panning[operator_index] - 0.5);

                        let r = pan_transformed.max(0.0);
                        let l = (pan_transformed * -1.0).max(0.0);

                        // Width 8 in case of eventual avx512 support
                        let data = [l, r, l, r, l, r, l, r];
                        
                        let tendency = pd_loadu(&data[0]);
                        let one_minus_tendency = pd_sub(pd_set1(1.0), tendency);

                        (tendency, one_minus_tendency)
                    };

                    let constant_power_panning = {
                        let mut data = [0.0f64; 8];

                        let left_and_right = operators[operator_index].panning.left_and_right;
                        
                        for (i, v) in data.iter_mut().enumerate() {
                            *v = left_and_right[i % 2];
                        }

                        pd_loadu(&data[0])
                    };

                    let modulation_target = operator_modulation_targets[operator_index];

                    // --- Create samples for both channels

                    let tau_splat = pd_set1(TAU);

                    for i in (0..SAMPLE_PASS_SIZE * 2).step_by(pd_width) {
                        let envelope_volume = pd_loadu(&operator_envelope_volumes[operator_index][i]);
                        let volume_product = pd_mul(operator_volume_splat, envelope_volume);

                        // Skip generation when envelope volume or operator volume is zero.
                        // Helps performance when operator envelope lengths vary a lot.
                        // Otherwise, the branching probably negatively impacts performance.
                        {
                            let volume_on = pd_gt(volume_product, zero_value_limit_splat);

                            let mut volume_on_tmp = [0.0f64; pd_width];
                            pd_storeu(&mut volume_on_tmp[0], volume_on);

                            // Higher indeces don't really matter: if previous sample has zero
                            // envelope volume, next one probably does too. Worst case scenario
                            // is that attacks are a tiny bit slower.
                            if volume_on_tmp[0].to_bits() == 0 {
                                continue;
                            }
                        }

                        let modulation_in_for_channel = pd_loadu(&voice_modulation_inputs[operator_index][i]);
                        let phase = pd_mul(pd_loadu(&operator_phases[operator_index][i]), tau_splat);

                        // Weird modulation input panning
                        // Note: breaks unless pd_width >= 2
                        let modulation_in_channel_sum = pd_mod_input_panning(
                            modulation_in_for_channel
                        );

                        let modulation_in = pd_add(pd_mul(pan_tendency, modulation_in_channel_sum),
                            pd_mul(one_minus_pan_tendency, modulation_in_for_channel));

                        let feedback = pd_mul(operator_feedback_splat, pd_fast_sin(phase));

                        let sin_input = pd_add(pd_mul(operator_modulation_index_splat, pd_add(feedback, modulation_in)), phase);

                        let sample = pd_fast_sin(sin_input);

                        let sample_adjusted = pd_mul(pd_mul(sample, volume_product), constant_power_panning);
                        let additive_out = pd_mul(sample_adjusted, operator_additive_splat);
                        let modulation_out = pd_sub(sample_adjusted, additive_out);

                        // Add modulation output to target operator's modulation inputs
                        let modulation_sum = pd_add(pd_loadu(&voice_modulation_inputs[modulation_target][i]), modulation_out);
                        pd_storeu(&mut voice_modulation_inputs[modulation_target][i], modulation_sum);

                        // Add additive output to summed_additive_outputs
                        let summed_plus_new = pd_add(
                            pd_loadu(&summed_additive_outputs[i]),
                            pd_mul(additive_out, voice_volume_factor_splat)
                        );
                        pd_storeu(
                            &mut summed_additive_outputs[i],
                            summed_plus_new
                        );
                    } // End of sample pass size *  2 iteration
                }
            } // End of operator iteration
        } // End of voice iteration

        // --- Summed additive outputs: apply hard limit.

        let max_value_splat = pd_set1(5.0);
        let min_value_splat = pd_set1(-5.0);

        for i in (0..SAMPLE_PASS_SIZE * 2).step_by(pd_width) {
            let additive = pd_loadu(&summed_additive_outputs[i]);

            let additive = pd_min(additive, max_value_splat);
            let additive = pd_max(additive, min_value_splat);

            pd_storeu(&mut summed_additive_outputs[i], additive);
        }

        // --- Write additive outputs to audio buffer

        let sample_offset = pass_index * SAMPLE_PASS_SIZE;

        for i in 0..SAMPLE_PASS_SIZE {
            let j = i * 2;
            audio_buffer_lefts[i + sample_offset] = summed_additive_outputs[j] as f32;
            audio_buffer_rights[i + sample_offset] = summed_additive_outputs[j + 1] as f32;
        }
    }

    /// Operator dependency analysis to allow skipping audio generation when possible
    #[target_feature_enable]
    unsafe fn run_operator_dependency_analysis(
        operator_volume: [f64; 4],
        operator_additive: [f64; 4],
        operator_modulation_index: [f64; 4],
        operator_wave_type: [WaveType; 4],
        operator_modulation_targets: [usize; 4],
    ) -> [bool; 4] {
        let mut operator_generate_audio = [true; 4];
        let mut operator_additive_zero = [false; 4];
        let mut operator_modulation_index_zero = [false; 4];
        
        for operator_index in 0..4 {
            // If volume is off, just set to skippable, don't even bother with lt calculations
            if operator_volume[operator_index].lt(&ZERO_VALUE_LIMIT){
                operator_generate_audio[operator_index] = false;
            } else {
                operator_additive_zero[operator_index] =
                    operator_additive[operator_index].lt(&ZERO_VALUE_LIMIT);

                operator_modulation_index_zero[operator_index] =
                    operator_modulation_index[operator_index].lt(&ZERO_VALUE_LIMIT);
            }
        }

        for _ in 0..3 {
            for operator_index in 1..4 {
                let modulation_target = operator_modulation_targets[operator_index];

                // Skip generation if operator was previously determined to be skippable OR
                let skip_condition = !operator_generate_audio[operator_index] || (
                    // Additive factor for this operator is off AND
                    operator_additive_zero[operator_index] && (
                        // Modulation target was previously determined to be skippable OR
                        !operator_generate_audio[modulation_target] ||
                        // Modulation target is white noise OR
                        operator_wave_type[modulation_target] == WaveType::WhiteNoise ||
                        // Modulation target doesn't do anything with its input modulation
                        operator_modulation_index_zero[modulation_target]
                    )
                );

                if skip_condition {
                    operator_generate_audio[operator_index] = false;
                }
            }
        }

        operator_generate_audio
    }
}
