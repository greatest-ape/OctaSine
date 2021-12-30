mod lfo;
pub mod simd;
mod voice_data;

use duplicate::duplicate;
use vst::buffer::AudioBuffer;

use crate::common::*;
use crate::constants::*;
use crate::parameters::processing::ProcessingParameter;
use crate::OctaSine;

use lfo::*;
use simd::*;
use voice_data::*;

pub trait AudioGen {
    unsafe fn process_f32(octasine: &mut OctaSine, lefts: &mut [f32], rights: &mut [f32]);
}

enum RemainingSamples {
    TwoOrMore,
    One,
    Zero,
}

impl RemainingSamples {
    fn new(remaining_samples: usize) -> Self {
        if remaining_samples >= 2 {
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
        unsafe {
            match RemainingSamples::new(num_samples - position) {
                #[cfg(all(feature = "simd", target_arch = "x86_64"))]
                RemainingSamples::TwoOrMore if is_x86_feature_detected!("avx") => {
                    let end_position = position + 2;

                    Avx::process_f32(
                        octasine,
                        &mut lefts[position..end_position],
                        &mut rights[position..end_position],
                    );

                    position = end_position;
                }
                RemainingSamples::One | RemainingSamples::TwoOrMore => {
                    let end_position = position + 1;

                    cfg_if::cfg_if!(
                        if #[cfg(all(feature = "simd", target_arch = "x86_64"))] {
                            // SSE2 is always supported on x86_64
                            Sse2::process_f32(
                                octasine,
                                &mut lefts[position..end_position],
                                &mut rights[position..end_position]
                            );
                        } else {
                            FallbackStd::process_f32(
                                octasine,
                                &mut lefts[position..end_position],
                                &mut rights[position..end_position]
                            );
                        }
                    );

                    position = end_position;
                }
                RemainingSamples::Zero => {
                    break;
                }
            }
        }
    }
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
    use once_cell::sync::OnceCell;

    #[feature_gate]
    use super::*;

    #[feature_gate]
    type VoiceData = super::VoiceData<{ S::PD_WIDTH }, { S::SAMPLES }>;

    #[feature_gate]
    impl AudioGen for S {
        #[target_feature_enable]
        unsafe fn process_f32(octasine: &mut OctaSine, lefts: &mut [f32], rights: &mut [f32]) {
            assert_eq!(lefts.len(), S::SAMPLES);
            assert_eq!(rights.len(), S::SAMPLES);

            octasine.processing.global_time.0 +=
                S::SAMPLES as f64 * octasine.processing.time_per_sample.0;

            static VOICE_DATA: OnceCell<SingleAccessLock<[VoiceData; 128]>> = OnceCell::new();

            let lock = VOICE_DATA.get_or_init(|| {
                SingleAccessLock::new(array_init::array_init(|_| VoiceData::default()))
            });

            let opt_guard = lock.get_mut();

            let mut voice_data = if let Some(voice_data) = opt_guard {
                voice_data
            } else {
                ::log::error!("audio gen concurrent access");

                return;
            };

            extract_voice_data(octasine, &mut voice_data);
            gen_audio(&mut octasine.processing.rng, &mut voice_data, lefts, rights);
        }
    }

    #[feature_gate]
    #[target_feature_enable]
    unsafe fn extract_voice_data(
        octasine: &mut OctaSine,
        voice_data: &mut SingleAccessLockGuard<[VoiceData; 128]>,
    ) {
        for voice_data in voice_data.iter_mut() {
            voice_data.active = false;
            // TODO: reset values if active?
        }

        let any_voice_active = octasine
            .processing
            .voices
            .iter()
            .filter(|voice| voice.active)
            .count()
            > 0;

        if !any_voice_active {
            return;
        }

        let time_per_sample = octasine.processing.time_per_sample;
        let bpm = octasine.get_bpm();

        for sample_index in 0..S::SAMPLES {
            octasine.processing.parameters.advance_one_sample();

            let operators = &mut octasine.processing.parameters.operators;

            for (voice, voice_data) in octasine
                .processing
                .voices
                .iter_mut()
                .zip(voice_data.iter_mut())
                .filter(|(voice, _)| voice.active)
            {
                voice_data.active = true;

                let sample_index_offset = sample_index * 2;

                let lfo_values = get_lfo_target_values(
                    &mut octasine.processing.parameters.lfos,
                    &mut voice.lfos,
                    time_per_sample.0,
                    bpm,
                );

                let voice_base_frequency = voice.midi_pitch.get_frequency(
                    octasine
                        .processing
                        .parameters
                        .master_frequency
                        .get_value_with_lfo_addition(lfo_values.get(LfoTargetParameter::Master(
                            LfoTargetMasterParameter::Frequency,
                        ))),
                );

                for (operator_index, operator) in operators.iter_mut().enumerate() {
                    voice_data.operator_wave_type[operator_index] = operator.wave_type.value;

                    if let Some(p) = &mut operator.output_operator {
                        voice_data.operator_modulation_targets[operator_index] = p.get_value();
                    }

                    let volume = operator.volume.get_value_with_lfo_addition(lfo_values.get(
                        LfoTargetParameter::Operator(
                            operator_index,
                            LfoTargetOperatorParameter::Volume,
                        ),
                    ));

                    voice_data.operator_volumes[operator_index][sample_index_offset] = volume;
                    voice_data.operator_volumes[operator_index][sample_index_offset + 1] = volume;

                    let modulation_index =
                        operator
                            .modulation_index
                            .get_value_with_lfo_addition(lfo_values.get(
                                LfoTargetParameter::Operator(
                                    operator_index,
                                    LfoTargetOperatorParameter::ModulationIndex,
                                ),
                            ));

                    voice_data.operator_modulation_indices[operator_index][sample_index_offset] =
                        modulation_index;
                    voice_data.operator_modulation_indices[operator_index]
                        [sample_index_offset + 1] = modulation_index;

                    let feedback = operator
                        .feedback
                        .get_value_with_lfo_addition(lfo_values.get(LfoTargetParameter::Operator(
                            operator_index,
                            LfoTargetOperatorParameter::Feedback,
                        )));

                    voice_data.operator_feedbacks[operator_index][sample_index_offset] = feedback;
                    voice_data.operator_feedbacks[operator_index][sample_index_offset + 1] =
                        feedback;

                    let panning = operator.panning.get_value_with_lfo_addition(lfo_values.get(
                        LfoTargetParameter::Operator(
                            operator_index,
                            LfoTargetOperatorParameter::Panning,
                        ),
                    ));

                    voice_data.operator_pannings[operator_index][sample_index] = panning;
                    let [l, r] = operator.panning.left_and_right;
                    voice_data.operator_constant_power_pannings[operator_index]
                        [sample_index_offset] = l;
                    voice_data.operator_constant_power_pannings[operator_index]
                        [sample_index_offset + 1] = r;

                    // Get additive factor; use 1.0 for operator 1
                    let additive = if operator_index == 0 {
                        1.0
                    } else {
                        operator
                            .additive_factor
                            .get_value_with_lfo_addition(lfo_values.get(
                                LfoTargetParameter::Operator(
                                    operator_index,
                                    LfoTargetOperatorParameter::Additive,
                                ),
                            ))
                    };

                    voice_data.operator_additives[operator_index][sample_index_offset] = additive;
                    voice_data.operator_additives[operator_index][sample_index_offset + 1] =
                        additive;

                    let frequency_ratio =
                        operator
                            .frequency_ratio
                            .get_value_with_lfo_addition(lfo_values.get(
                                LfoTargetParameter::Operator(
                                    operator_index,
                                    LfoTargetOperatorParameter::FrequencyRatio,
                                ),
                            ));
                    let frequency_free =
                        operator
                            .frequency_free
                            .get_value_with_lfo_addition(lfo_values.get(
                                LfoTargetParameter::Operator(
                                    operator_index,
                                    LfoTargetOperatorParameter::FrequencyFree,
                                ),
                            ));
                    let frequency_fine =
                        operator
                            .frequency_fine
                            .get_value_with_lfo_addition(lfo_values.get(
                                LfoTargetParameter::Operator(
                                    operator_index,
                                    LfoTargetOperatorParameter::FrequencyFine,
                                ),
                            ));

                    let frequency =
                        voice_base_frequency * frequency_ratio * frequency_free * frequency_fine;

                    voice_data.operator_frequencies[operator_index][sample_index_offset] =
                        frequency;
                    voice_data.operator_frequencies[operator_index][sample_index_offset + 1] =
                        frequency;
                }

                // Envelope
                for (operator_index, operator) in operators.iter_mut().enumerate() {
                    let v = voice.operators[operator_index].volume_envelope.get_volume(
                        &octasine.processing.log10_table,
                        &operator.volume_envelope,
                        voice.key_pressed,
                        voice.duration,
                    );

                    voice_data.operator_envelope_volumes[operator_index][sample_index_offset] = v;
                    voice_data.operator_envelope_volumes[operator_index][sample_index_offset + 1] =
                        v;
                }

                voice.duration.0 += time_per_sample.0;

                // Phase
                for operator_index in 0..4 {
                    let frequency =
                        voice_data.operator_frequencies[operator_index][sample_index * 2];

                    let last_phase = voice.operators[operator_index].last_phase.0;
                    let phase_addition = frequency * time_per_sample.0;

                    let new_phase = last_phase + phase_addition;

                    voice_data.operator_phases[operator_index][sample_index_offset] = new_phase;
                    voice_data.operator_phases[operator_index][sample_index_offset + 1] = new_phase;

                    // Save phase
                    voice.operators[operator_index].last_phase.0 = new_phase;
                }

                let voice_volume_factor = {
                    let lfo_parameter =
                        LfoTargetParameter::Master(LfoTargetMasterParameter::Volume);
                    let lfo_addition = lfo_values.get(lfo_parameter);

                    let master_volume = octasine
                        .processing
                        .parameters
                        .master_volume
                        .get_value_with_lfo_addition(lfo_addition);

                    let key_velocity = voice.key_velocity.0;

                    VOICE_VOLUME_FACTOR * master_volume * key_velocity
                };

                voice_data.volume_factors[sample_index_offset] = voice_volume_factor;
                voice_data.volume_factors[sample_index_offset + 1] = voice_volume_factor;

                voice.deactivate_if_envelopes_ended();
            }
        }
    }

    #[feature_gate]
    #[target_feature_enable]
    unsafe fn gen_audio(
        rng: &mut fastrand::Rng,
        voice_data: &mut SingleAccessLockGuard<[VoiceData; 128]>,
        audio_buffer_lefts: &mut [f32],
        audio_buffer_rights: &mut [f32],
    ) {
        // Maybe operator indexes should be inversed (3 - operator_index)
        // because that is how they will be accessed later.

        // S::SAMPLES * 2 because of two channels. Even index = left channel
        let mut summed_additive_outputs = [0.0f64; S::SAMPLES * 2];

        for voice_data in voice_data.iter().filter(|voice_data| voice_data.active) {
            // let operator_generate_audio = run_operator_dependency_analysis(voice_data);

            // --- Generate samples for all operators

            // Voice modulation input storage, indexed by operator
            let mut voice_modulation_inputs = [[0.0f64; S::SAMPLES * 2]; 4];

            // Go through operators downwards, starting with operator 4
            for operator_index in 0..4 {
                // FIXME: better iterator with 3, 2, 1, 0 possible?
                let operator_index = 3 - operator_index;

                // Disabled for now
                // Possibly skip generation based on previous dependency analysis
                // if !operator_generate_audio[operator_index] {
                //     continue;
                // }

                let envelope_volume =
                    S::pd_loadu(voice_data.operator_envelope_volumes[operator_index].as_ptr());

                // Skip generation when envelope volume is zero. Helps
                // performance when operator envelope lengths vary a lot.
                // Otherwise, the branching probably negatively impacts
                // performance.
                if !S::pd_over_zero_limit(envelope_volume) {
                    continue;
                }

                let modulation_target = voice_data.operator_modulation_targets[operator_index];

                let sample = if voice_data.operator_wave_type[operator_index]
                    == WaveType::WhiteNoise
                {
                    let mut random_numbers = [0.0f64; S::PD_WIDTH];

                    for sample_index in 0..S::SAMPLES {
                        let random = (rng.f64() - 0.5) * 2.0;

                        let sample_index_offset = sample_index * 2;

                        random_numbers[sample_index_offset] = random;
                        random_numbers[sample_index_offset + 1] = random;
                    }

                    S::pd_loadu(random_numbers.as_ptr())
                } else {
                    let mut pan_tendency = [0.0f64; S::PD_WIDTH];

                    for sample_index in 0..S::SAMPLES {
                        // Get panning as value between -1 and 1
                        let pan_transformed = 2.0
                            * (voice_data.operator_pannings[operator_index][sample_index] - 0.5);

                        let r = pan_transformed.max(0.0);
                        let l = (pan_transformed * -1.0).max(0.0);

                        let sample_index_offset = sample_index * 2;

                        pan_tendency[sample_index_offset] = l;
                        pan_tendency[sample_index_offset + 1] = r;
                    }

                    let pan_tendency = S::pd_loadu(pan_tendency.as_ptr());
                    let one_minus_pan_tendency = S::pd_sub(S::pd_set1(1.0), pan_tendency);

                    let modulation_in_for_channel =
                        S::pd_loadu(voice_modulation_inputs[operator_index].as_ptr());
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

                    let phase = S::pd_mul(
                        S::pd_loadu(voice_data.operator_phases[operator_index].as_ptr()),
                        S::pd_set1(TAU),
                    );
                    let feedback = S::pd_mul(
                        S::pd_loadu(voice_data.operator_feedbacks[operator_index].as_ptr()),
                        S::pd_fast_sin(phase),
                    );

                    let modulation_index = S::pd_loadu(
                        voice_data.operator_modulation_indices[operator_index].as_ptr(),
                    );
                    let modulation_phase_addition =
                        S::pd_mul(modulation_index, S::pd_add(feedback, modulation_in));

                    let sin_input = S::pd_add(phase, modulation_phase_addition);

                    S::pd_fast_sin(sin_input)
                };

                let operator_volume =
                    S::pd_loadu(voice_data.operator_volumes[operator_index].as_ptr());
                let constant_power_panning = S::pd_loadu(
                    voice_data.operator_constant_power_pannings[operator_index].as_ptr(),
                );
                let operator_additive =
                    S::pd_loadu(voice_data.operator_additives[operator_index].as_ptr());

                let volume_product = S::pd_mul(operator_volume, envelope_volume);

                let sample_adjusted =
                    S::pd_mul(S::pd_mul(sample, volume_product), constant_power_panning);
                let additive_out = S::pd_mul(sample_adjusted, operator_additive);
                let modulation_out = S::pd_sub(sample_adjusted, additive_out);

                // Add modulation output to target operator's modulation inputs
                let modulation_sum = S::pd_add(
                    S::pd_loadu(voice_modulation_inputs[modulation_target].as_ptr()),
                    modulation_out,
                );
                S::pd_storeu(
                    voice_modulation_inputs[modulation_target].as_mut_ptr(),
                    modulation_sum,
                );

                let addition = S::pd_mul(
                    additive_out,
                    S::pd_loadu(voice_data.volume_factors.as_ptr()),
                );

                // Add additive output to summed_additive_outputs
                let summed_plus_new =
                    S::pd_add(S::pd_loadu(summed_additive_outputs.as_ptr()), addition);
                S::pd_storeu(summed_additive_outputs.as_mut_ptr(), summed_plus_new);
            }
        }

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
    unsafe fn run_operator_dependency_analysis(voice_data: &VoiceData) -> [bool; 4] {
        let mut operator_generate_audio = [true; 4];
        let mut operator_additive_zero = [false; 4];
        let mut operator_modulation_index_zero = [false; 4];

        for operator_index in 0..4 {
            let operator_volume = S::pd_loadu(voice_data.operator_volumes[operator_index].as_ptr());

            if S::pd_over_zero_limit(operator_volume) {
                operator_additive_zero[operator_index] = !S::pd_over_zero_limit(S::pd_loadu(
                    voice_data.operator_additives[operator_index].as_ptr(),
                ));
                operator_modulation_index_zero[operator_index] = !S::pd_over_zero_limit(
                    S::pd_loadu(voice_data.operator_modulation_indices[operator_index].as_ptr()),
                );
            } else {
                // If volume is off, just set to skippable, don't even bother with lt calculations
                operator_generate_audio[operator_index] = false;
            }
        }

        for _ in 0..3 {
            for operator_index in 1..4 {
                let modulation_target = voice_data.operator_modulation_targets[operator_index];

                // Skip generation if operator was previously determined to be skippable OR
                #[rustfmt::skip]
                let skip_condition = !operator_generate_audio[operator_index] | (
                    // Additive factor for this operator is off AND
                    operator_additive_zero[operator_index] & (
                        // Modulation target was previously determined to be skippable OR
                        !operator_generate_audio[modulation_target] |
                        // Modulation target is white noise OR
                        (voice_data.operator_wave_type[modulation_target] == WaveType::WhiteNoise) |
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
