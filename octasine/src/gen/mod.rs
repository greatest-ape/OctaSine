mod lfo;
pub mod simd;

use duplicate::duplicate;
use vst::buffer::AudioBuffer;

use crate::common::*;
use crate::constants::*;
use crate::parameters::processing::ProcessingParameter;
use crate::OctaSine;

use lfo::*;
use simd::*;

enum RemainingSamples {
    FourOrMore,
    TwoOrMore,
    One,
    Zero
}

impl RemainingSamples {
    fn new(remaining_samples: usize) -> Self {
        if remaining_samples >= 4 {
            Self::FourOrMore
        } else if remaining_samples >= 2 {
            Self::TwoOrMore
        } else if remaining_samples == 1 {
            Self::One
        } else {
            Self::Zero
        }
    }
}

#[inline]
pub fn process_f32_runtime_select(octasine: &mut OctaSine, audio_buffer: &mut AudioBuffer<f32>) {
    octasine.update_processing_parameters();

    let num_samples = audio_buffer.samples();

    let mut outputs = audio_buffer.split().1;
    let lefts = outputs.get_mut(0);
    let rights = outputs.get_mut(1);

    let mut position = 0;

    loop {
        use RemainingSamples::*;

        unsafe {
            match RemainingSamples::new(num_samples - position) {
                #[cfg(all(feature = "simd", target_arch = "x86_64"))]
                FourOrMore if is_x86_feature_detected!("avx") => {
                    let end_position = position + 4;

                    Avx::process_f32(octasine, &mut lefts[position..end_position], &mut rights[position..end_position]);

                    position = end_position;
                }
                #[cfg(all(feature = "simd", target_arch = "x86_64"))]
                TwoOrMore | FourOrMore => {
                    let end_position = position + 2;

                    // SSE2 is always supported on x86_64
                    Sse2::process_f32(octasine, &mut lefts[position..end_position], &mut rights[position..end_position]);

                    position = end_position;
                }
                One | TwoOrMore | FourOrMore => {
                    let end_position = position + 1;

                    cfg_if::cfg_if!(
                        if #[cfg(all(feature = "simd", target_arch = "x86_64"))] {
                            FallbackSleef::process_f32(octasine, &mut lefts[position..end_position], &mut rights[position..end_position]);
                        } else {
                            FallbackStd::process_f32(octasine, &mut lefts[position..end_position], &mut rights[position..end_position]);
                        }
                    );

                    position = end_position;
                }
                Zero => {
                    break;
                }
            }
        }
    }
}

pub trait AudioGen {
    unsafe fn process_f32(octasine: &mut OctaSine, lefts: &mut [f32], rights: &mut [f32]);
}

#[duplicate(
    [
        S [ FallbackStd ]
        target_feature_enable [ cfg(not(feature = "fake-feature")) ]
        feature_gate [ cfg(not(feature = "fake-feature")) ]
    ]
    [
        S [ FallbackSleef ]
        target_feature_enable [ cfg(not(feature = "fake-feature")) ]
        feature_gate [ cfg(all(feature = "simd")) ]
    ]
    [
        S [ Sse2 ]
        target_feature_enable [ target_feature(enable = "sse2") ]
        feature_gate [ cfg(all(feature = "simd", target_arch = "x86_64")) ]
    ]
    [
        S [ Avx ]
        target_feature_enable [ target_feature(enable = "avx") ]
        feature_gate [ cfg(all(feature = "simd", target_arch = "x86_64")) ]
    ]
)]
mod gen {
    #[feature_gate]
    use super::*;

    #[feature_gate]
    impl AudioGen for S {
        #[target_feature_enable]
        unsafe fn process_f32(octasine: &mut OctaSine, lefts: &mut [f32], rights: &mut [f32]) {
            process(octasine, lefts, rights)
        }
    }

    #[feature_gate]
    #[target_feature_enable]
    unsafe fn process(
        octasine: &mut OctaSine,
        audio_buffer_lefts: &mut [f32],
        audio_buffer_rights: &mut [f32],
    ) {
        assert_eq!(audio_buffer_lefts.len(), S::SAMPLES);
        assert_eq!(audio_buffer_rights.len(), S::SAMPLES);

        // --- Set some generally useful variables

        let time_per_sample = octasine.processing.time_per_sample;
        let bpm = octasine.get_bpm();

        let operators = &mut octasine.processing.parameters.operators;

        // --- Collect parameter data and do audio generation

        // Maybe operator indexes should be inversed (3 - operator_index)
        // because that is how they will be accessed later.

        // S::SAMPLES * 2 because of two channels. Even index = left channel
        let mut summed_additive_outputs = [0.0f64; S::SAMPLES * 2];

        for voice in octasine
            .processing
            .voices
            .iter_mut()
            .filter(|voice| voice.active)
        {
            // Can be changed per sample
            let mut operator_volumes = [[0.0f64; S::SAMPLES * 2]; 4];
            let mut operator_modulation_indices = [[0.0f64; S::SAMPLES * 2]; 4];
            let mut operator_feedbacks = [[0.0f64; S::SAMPLES * 2]; 4];
            let mut operator_additives = [[0.0f64; S::SAMPLES * 2]; 4];
            let mut operator_frequencies = [[0.0f64; S::SAMPLES * 2]; 4];

            let mut operator_pannings = [[0.0f64; S::SAMPLES]; 4];
            let mut operator_pannings_left_and_right = [[0.0f64; S::SAMPLES * 2]; 4];
            let mut operator_envelope_volumes = [[0.0f64; S::SAMPLES * 2]; 4];
            let mut operator_phases = [[0.0f64; S::SAMPLES * 2]; 4];

            // Can not be changed per sample
            let mut operator_wave_type = [WaveType::Sine; 4];
            let mut operator_modulation_targets = [0usize; 4];

            // Other
            let mut voice_volume_factors = [0.0f64; S::SAMPLES * 2];

            for sample_index in 0..S::SAMPLES {
                let sample_index_offset = sample_index * 2;

                // --- Get voice data

                let time = octasine.processing.global_time;
                let time_advancement = time_per_sample.0;

                // Necessary for interpolation
                octasine.processing.global_time.0 += time_advancement;

                let lfo_values = get_lfo_target_values(
                    &mut octasine.processing.parameters.lfos,
                    &mut voice.lfos,
                    time,
                    time_advancement,
                    bpm,
                );

                let voice_base_frequency = voice.midi_pitch.get_frequency(
                    octasine
                        .processing
                        .parameters
                        .master_frequency
                        .get_value_with_lfo_addition(
                            (),
                            lfo_values.get(LfoTargetParameter::Master(
                                LfoTargetMasterParameter::Frequency,
                            )),
                        ),
                );

                for (operator_index, operator) in operators.iter_mut().enumerate() {
                    operator_wave_type[operator_index] = operator.wave_type.value;

                    if let Some(p) = &mut operator.output_operator {
                        operator_modulation_targets[operator_index] = p.get_value();
                    }

                    let volume = operator.volume.get_value_with_lfo_addition(
                        time,
                        lfo_values.get(LfoTargetParameter::Operator(
                            operator_index,
                            LfoTargetOperatorParameter::Volume,
                        )),
                    );

                    operator_volumes[operator_index][sample_index_offset] = volume;
                    operator_volumes[operator_index][sample_index_offset + 1] = volume;

                    let modulation_index = operator.modulation_index.get_value_with_lfo_addition(
                        time,
                        lfo_values.get(LfoTargetParameter::Operator(
                            operator_index,
                            LfoTargetOperatorParameter::ModulationIndex,
                        )),
                    );

                    operator_modulation_indices[operator_index][sample_index_offset] = modulation_index;
                    operator_modulation_indices[operator_index][sample_index_offset + 1] = modulation_index;

                    let feedback = operator.feedback.get_value_with_lfo_addition(
                        time,
                        lfo_values.get(LfoTargetParameter::Operator(
                            operator_index,
                            LfoTargetOperatorParameter::Feedback,
                        )),
                    );

                    operator_feedbacks[operator_index][sample_index_offset] = feedback;
                    operator_feedbacks[operator_index][sample_index_offset + 1] = feedback;

                    let panning = operator.panning.get_value_with_lfo_addition(
                        time,
                        lfo_values.get(LfoTargetParameter::Operator(
                            operator_index,
                            LfoTargetOperatorParameter::Panning,
                        )),
                    );

                    operator_pannings[operator_index][sample_index] = panning;
                    let [l, r] = operator.panning.left_and_right;
                    operator_pannings_left_and_right[operator_index][sample_index_offset] = l;
                    operator_pannings_left_and_right[operator_index][sample_index_offset + 1] = r;

                    // Get additive factor; use 1.0 for operator 1
                    let additive = if operator_index == 0 {
                        1.0
                    } else {
                        operator.additive_factor.get_value_with_lfo_addition(
                            time,
                            lfo_values.get(LfoTargetParameter::Operator(
                                operator_index,
                                LfoTargetOperatorParameter::Additive,
                            )),
                        )
                    };

                    operator_additives[operator_index][sample_index_offset] = additive;
                    operator_additives[operator_index][sample_index_offset + 1] = additive;

                    let frequency_ratio = operator.frequency_ratio.get_value_with_lfo_addition(
                        (),
                        lfo_values.get(LfoTargetParameter::Operator(
                            operator_index,
                            LfoTargetOperatorParameter::FrequencyRatio,
                        )),
                    );
                    let frequency_free = operator.frequency_free.get_value_with_lfo_addition(
                        (),
                        lfo_values.get(LfoTargetParameter::Operator(
                            operator_index,
                            LfoTargetOperatorParameter::FrequencyFree,
                        )),
                    );
                    let frequency_fine = operator.frequency_fine.get_value_with_lfo_addition(
                        (),
                        lfo_values.get(LfoTargetParameter::Operator(
                            operator_index,
                            LfoTargetOperatorParameter::FrequencyFine,
                        )),
                    );

                    let frequency = voice_base_frequency * frequency_ratio * frequency_free * frequency_fine;

                    operator_frequencies[operator_index][sample_index_offset] = frequency;
                    operator_frequencies[operator_index][sample_index_offset + 1] = frequency;
                }

                // Envelope
                for (operator_index, operator) in operators.iter_mut().enumerate() {
                    let v = voice.operators[operator_index].volume_envelope.get_volume(
                        &octasine.processing.log10_table,
                        &operator.volume_envelope,
                        voice.key_pressed,
                        voice.duration,
                    );

                    operator_envelope_volumes[operator_index][sample_index_offset] = v;
                    operator_envelope_volumes[operator_index][sample_index_offset + 1] = v;
                }

                voice.duration.0 += time_per_sample.0;

                voice.deactivate_if_envelopes_ended();

                // Phase
                for operator_index in 0..4 {
                    let frequency = operator_frequencies[operator_index][sample_index * 2];

                    let last_phase = voice.operators[operator_index].last_phase.0;
                    let phase_addition = frequency * time_per_sample.0;

                    let new_phase = last_phase + phase_addition;

                    operator_phases[operator_index][sample_index_offset] = new_phase;
                    operator_phases[operator_index][sample_index_offset + 1] = new_phase;

                    // Save phase
                    voice.operators[operator_index].last_phase.0 = new_phase;
                }

                let voice_volume_factor = {
                    let lfo_parameter = LfoTargetParameter::Master(LfoTargetMasterParameter::Volume);
                    let lfo_addition = lfo_values.get(lfo_parameter);

                    let master_volume = octasine
                        .processing
                        .parameters
                        .master_volume
                        .get_value_with_lfo_addition(time, lfo_addition);

                    let key_velocity = voice.key_velocity.0;

                    VOICE_VOLUME_FACTOR * master_volume * key_velocity
                };

                voice_volume_factors[sample_index_offset] = voice_volume_factor;
                voice_volume_factors[sample_index_offset + 1] = voice_volume_factor;
            }

            /*
            let operator_generate_audio = run_operator_dependency_analysis(
                operator_volume,
                operator_additive,
                operator_modulation_index,
                operator_wave_type,
                operator_modulation_targets,
            );
            */

            // --- Generate samples for all operators

            // Voice modulation input storage, indexed by operator
            let mut voice_modulation_inputs = [[0.0f64; S::SAMPLES * 2]; 4];

            // Go through operators downwards, starting with operator 4
            for operator_index in 0..4 {
                // FIXME: better iterator with 3, 2, 1, 0 possible?
                let operator_index = 3 - operator_index;

                // Possibly skip generation based on previous dependency analysis
                // if !operator_generate_audio[operator_index] {
                //     continue;
                // }

                if operator_wave_type[operator_index] == WaveType::WhiteNoise {
                    let random_numbers = {
                        let mut random_numbers = [0.0f64; S::SAMPLES * 2];

                        for i in 0..S::SAMPLES {
                            let random = (octasine.processing.rng.f64() - 0.5) * 2.0;

                            let j = i * 2;

                            random_numbers[j] = random;
                            random_numbers[j + 1] = random;
                        }

                        random_numbers
                    };

                    let modulation_target = operator_modulation_targets[operator_index];

                    let constant_power_panning = S::pd_loadu(operator_pannings_left_and_right[operator_index].as_ptr());
                    let operator_volume = S::pd_loadu(operator_volumes[operator_index].as_ptr());
                    let operator_additive = S::pd_loadu(operator_additives[operator_index].as_ptr());

                    let envelope_volume =
                        S::pd_loadu(&operator_envelope_volumes[operator_index][0]);
                    let volume_product = S::pd_mul(operator_volume, envelope_volume);

                    let sample = S::pd_loadu(&random_numbers[0]);

                    let sample_adjusted =
                        S::pd_mul(S::pd_mul(sample, volume_product), constant_power_panning);
                    let additive_out = S::pd_mul(sample_adjusted, operator_additive);
                    let modulation_out = S::pd_sub(sample_adjusted, additive_out);

                    // Add modulation output to target operator's modulation inputs
                    let modulation_sum = S::pd_add(
                        S::pd_loadu(&voice_modulation_inputs[modulation_target][0]),
                        modulation_out,
                    );
                    S::pd_storeu(
                        &mut voice_modulation_inputs[modulation_target][0],
                        modulation_sum,
                    );

                    // Add additive output to summed_additive_outputs
                    let summed_plus_new = S::pd_add(
                        S::pd_loadu(&summed_additive_outputs[0]),
                        S::pd_mul(additive_out, S::pd_loadu(&voice_volume_factors[0])),
                    );
                    S::pd_storeu(&mut summed_additive_outputs[0], summed_plus_new);
                } else {
                    // --- Setup operator SIMD vars

                    let operator_volume = S::pd_loadu(operator_volumes[operator_index].as_ptr());
                    let operator_feedback = S::pd_loadu(operator_feedbacks[operator_index].as_ptr());
                    let operator_additive = S::pd_loadu(operator_additives[operator_index].as_ptr());
                    let operator_modulation_index =
                        S::pd_loadu(operator_modulation_indices[operator_index].as_ptr());
                    
                    let mut pan_tendency = [0.0f64; S::PD_WIDTH];
                    let mut one_minus_pan_tendency = [0.0f64; S::PD_WIDTH];

                    for sample_index in 0..S::SAMPLES {
                        // Get panning as value between -1 and 1
                        let pan_transformed = 2.0 * (operator_pannings[operator_index][sample_index] - 0.5);

                        let r = pan_transformed.max(0.0);
                        let l = (pan_transformed * -1.0).max(0.0);

                        let j = sample_index * 2;

                        pan_tendency[j] = l;
                        pan_tendency[j + 1] = r;

                        one_minus_pan_tendency[j] = 1.0 - l;
                        one_minus_pan_tendency[j + 1] = 1.0 - l;
                    }

                    let pan_tendency = S::pd_loadu(pan_tendency.as_ptr());
                    let one_minus_pan_tendency = S::pd_loadu(one_minus_pan_tendency.as_ptr());

                    let constant_power_panning = S::pd_loadu(operator_pannings_left_and_right[operator_index].as_ptr());

                    let modulation_target = operator_modulation_targets[operator_index];

                    // --- Create samples for both channels

                    let tau_splat = S::pd_set1(TAU);

                    let envelope_volume =
                        S::pd_loadu(operator_envelope_volumes[operator_index].as_ptr());
                    let volume_product = S::pd_mul(operator_volume, envelope_volume);

                    // Skip generation when envelope volume or operator
                    // volume is zero. Helps performance when operator
                    // envelope lengths vary a lot. Otherwise, the
                    // branching probably negatively impacts performance.
                    // Higher indeces don't really matter: if previous
                    // sample has zero envelope volume, next one probably
                    // does too. The worst case scenario is that attacks
                    // are a tiny bit slower.
                    if !S::pd_first_over_zero_limit(volume_product) {
                        continue;
                    }

                    let phase =
                        S::pd_mul(S::pd_loadu(&operator_phases[operator_index][0]), tau_splat);

                    let modulation_in_for_channel =
                        S::pd_loadu(&voice_modulation_inputs[operator_index][0]);
                    let modulation_in_channel_sum =
                        S::pd_pairwise_horizontal_sum(modulation_in_for_channel);
                    // Weird modulation input panning
                    // Mix modulator into current operator depending on
                    // panning of current operator. If panned to the
                    // middle, just pass through the stereo signals. If
                    // panned to any side, mix out the original stereo
                    // signals and mix in mono.
                    // Note: breaks unless S::PD_WIDTH >= 2
                    let modulation_in = S::pd_add(
                        S::pd_mul(pan_tendency, modulation_in_channel_sum),
                        S::pd_mul(one_minus_pan_tendency, modulation_in_for_channel),
                    );

                    let feedback = S::pd_mul(operator_feedback, S::pd_fast_sin(phase));

                    let sin_input = S::pd_add(
                        S::pd_mul(
                            operator_modulation_index,
                            S::pd_add(feedback, modulation_in),
                        ),
                        phase,
                    );

                    let sample = S::pd_fast_sin(sin_input);

                    let sample_adjusted =
                        S::pd_mul(S::pd_mul(sample, volume_product), constant_power_panning);
                    let additive_out = S::pd_mul(sample_adjusted, operator_additive);
                    let modulation_out = S::pd_sub(sample_adjusted, additive_out);

                    // Add modulation output to target operator's modulation inputs
                    let modulation_sum = S::pd_add(
                        S::pd_loadu(&voice_modulation_inputs[modulation_target][0]),
                        modulation_out,
                    );
                    S::pd_storeu(
                        &mut voice_modulation_inputs[modulation_target][0],
                        modulation_sum,
                    );

                    // Add additive output to summed_additive_outputs
                    let summed_plus_new = S::pd_add(
                        S::pd_loadu(&summed_additive_outputs[0]),
                        S::pd_mul(additive_out, S::pd_loadu(&voice_volume_factors[0])),
                    );
                    S::pd_storeu(&mut summed_additive_outputs[0], summed_plus_new);
                }
            } // End of operator iteration
        } // End of voice iteration

        // --- Summed additive outputs: apply hard limit.

        for out in summed_additive_outputs.iter_mut() {
            *out = out.min(5.0).max(-5.0);
        }

        // --- Write additive outputs to audio buffer

        for i in 0..S::SAMPLES {
            let j = i * 2;
            audio_buffer_lefts[i] = summed_additive_outputs[j] as f32;
            audio_buffer_rights[i] = summed_additive_outputs[j + 1] as f32;
        }
    }

    /// Operator dependency analysis to allow skipping audio generation when possible
    #[feature_gate]
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
            if operator_volume[operator_index].lt(&ZERO_VALUE_LIMIT) {
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
                let skip_condition = !operator_generate_audio[operator_index]
                    || (
                        // Additive factor for this operator is off AND
                        operator_additive_zero[operator_index]
                            && (
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
