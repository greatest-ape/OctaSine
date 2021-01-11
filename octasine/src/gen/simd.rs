//! Audio generation using explicit simd
//! 
//! At least SSE2 is required. Simdeez scalar fallback will fail due to the
//! method used for stereo modulation input calculation.
//! 
//! TODO:
//!   - Interpolation for processing parameters every sample? Build long arrays here too?

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


pub struct Sse2;
pub struct Avx;


#[inline]
pub fn process_f32_runtime_select(
    octasine: &mut OctaSine,
    audio_buffer: &mut AudioBuffer<f32>,
){
    #[cfg(target_arch = "x86_64")]
    unsafe {
        if is_x86_feature_detected!("avx") {
            Avx::process_f32(octasine, audio_buffer);
        } else if is_x86_feature_detected!("sse2") {
            Sse2::process_f32(octasine, audio_buffer);
        }
    }
}


#[duplicate(
    [
        instruction_set [ Sse2 ]
        target_feature_enable [ "sse2" ]
        pd  [ __m128d ]
        pd_width [ 2 ] 
        pd_set1 [ _mm_set1_pd ]
        pd_loadu [ _mm_loadu_pd ]
        pd_storeu [ _mm_storeu_pd ]
        pd_add [ _mm_add_pd ]
        pd_sub [ _mm_sub_pd ]
        pd_mul [ _mm_mul_pd ]
        pd_min [ _mm_min_pd ]
        pd_fast_sin [ sleef_sys::Sleef_sind2_u35sse2 ]
        pd_gt [ (|a, b| _mm_cmp_pd(a, b, _CMP_GT_OQ))]
    ]
    [
        instruction_set [ Avx ]
        target_feature_enable [ "avx" ]
        pd  [ __m256d ]
        pd_width [ 4 ] 
        pd_set1 [ _mm256_set1_pd ]
        pd_loadu [ _mm256_loadu_pd ]
        pd_storeu [ _mm256_storeu_pd ]
        pd_add [ _mm256_add_pd ]
        pd_sub [ _mm256_sub_pd ]
        pd_mul [ _mm256_mul_pd ]
        pd_min [ _mm256_min_pd ]
        pd_fast_sin [ sleef_sys::Sleef_sind4_u35avx ]
        pd_gt [ (|a, b| _mm256_cmp_pd(a, b, _CMP_GT_OQ))]
    ]
)]
mod gen {
    use core::arch::x86_64::*;

    use super::*;

    impl AudioGen for instruction_set {
        #[target_feature(enable = target_feature_enable)]
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

    #[target_feature(enable = target_feature_enable)]
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

        // FIXME: needs to use get_value_with_lfo_addition, requiring doing
        // per-voice master volume application
        let master_volume_factor = VOICE_VOLUME_FACTOR * octasine.processing.parameters.master_volume.get_value(time);

        // Necessary for interpolation
        octasine.processing.global_time.0 += time_per_sample.0 * (SAMPLE_PASS_SIZE as f64);

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
                time_per_sample,
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

            let operator_generate_audio = get_operator_generate_audio(
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

            let key_velocity_splat = pd_set1(voice.key_velocity.0);

            // Go through operators downwards, starting with operator 4
            for operator_index in 0..4 { // FIXME: better iterator with 3, 2, 1, 0 possible?
                let operator_index = 3 - operator_index;

                // Possibly skip generation based on previous dependency analysis
                if !operator_generate_audio[operator_index]{
                    continue;
                }
                
                // --- White noise audio generation

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
                        let summed_plus_new = pd_add(pd_loadu(&summed_additive_outputs[i]), pd_mul(additive_out, key_velocity_splat));
                        pd_storeu(&mut summed_additive_outputs[i], summed_plus_new);
                    }
                } else {
                    // --- Sine frequency modulation audio generation: setup operator SIMD vars

                    let operator_volume_splat = pd_set1(operator_volume[operator_index]);
                    let operator_feedback_splat = pd_set1(operator_feedback[operator_index]);
                    let operator_additive_splat = pd_set1(operator_additive[operator_index]);
                    let operator_modulation_index_splat = pd_set1(operator_modulation_index[operator_index]);

                    let (pan_tendency, one_minus_pan_tendency) = {
                        // Get panning as value between -1 and 1
                        let pan_transformed = 2.0 * (operator_panning[operator_index] - 0.5);

                        let r = pan_transformed.max(0.0);
                        let l = (pan_transformed * -1.0).max(0.0);

                        // Width 8 in case of eventual avx512 support in simdeez
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

                            let mut volume_on_tmp = [0.0; pd_width];
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
                        // Note: breaks without VF64_WIDTH >= 2 (SSE2 or newer)
                        let modulation_in_channel_sum = {
                            // Replacing with SIMD: suitable instructions in avx:
                            //   _mm256_permute_pd with imm8 = [1, 0, 1, 0] followed by addition
                            //     Indices:
                            //       0 -> 1
                            //       1 -> 0
                            //       2 -> 3
                            //       3 -> 2
                            //   _mm256_hadd_pd (takes two variables which would need to be identical): pretty slow
                            // So the idea is to take modulation_in_for_channel and run any of the above on it.

                            let mut permuted = [0.0f64; 8]; // Width 8 in case of eventual avx512 support in simdeez

                            // Should be equivalent to simd instruction permute_pd with imm8 = [1, 0, 1, 0]
                            for (j, input) in (&voice_modulation_inputs[operator_index][i..i + pd_width]).iter().enumerate(){
                                let add = (j + 1) % 2;
                                let subtract = j % 2;

                                permuted[j + add - subtract] = *input;
                            }

                            pd_add(pd_loadu(&permuted[0]), modulation_in_for_channel)
                        };

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
                        let summed_plus_new = pd_add(pd_loadu(&summed_additive_outputs[i]), pd_mul(additive_out, key_velocity_splat));
                        pd_storeu(&mut summed_additive_outputs[i], summed_plus_new);
                    } // End of sample pass size *  2 iteration
                }
            } // End of operator iteration
        } // End of voice iteration

        // --- Summed additive outputs: apply master volume and hard limit.

        let master_volume_factor_splat = pd_set1(master_volume_factor);
        let max_volume_splat = pd_set1(5.0);

        for i in (0..SAMPLE_PASS_SIZE * 2).step_by(pd_width) {
            let additive_outputs = pd_loadu(&summed_additive_outputs[i]);
            let additive_outputs = pd_mul(additive_outputs, master_volume_factor_splat);
            let limited_outputs = pd_min(additive_outputs, max_volume_splat);

            pd_storeu(&mut summed_additive_outputs[i], limited_outputs);
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
    #[target_feature(enable = target_feature_enable)]
    unsafe fn get_operator_generate_audio(
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
