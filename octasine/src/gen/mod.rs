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

const MAX_PD_WIDTH: usize = 4;

pub trait AudioGen {
    unsafe fn process_f32(
        octasine: &mut OctaSine,
        lefts: &mut [f32],
        rights: &mut [f32],
        position: usize,
    );
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

#[derive(Debug, Default)]
pub struct VoiceData {
    pub active: bool,
    pub operator_volumes: [[f64; MAX_PD_WIDTH]; 4],
    pub operator_modulation_indices: [[f64; MAX_PD_WIDTH]; 4],
    pub operator_feedbacks: [[f64; MAX_PD_WIDTH]; 4],
    pub operator_additives: [[f64; MAX_PD_WIDTH]; 4],
    pub operator_pannings: [[f64; MAX_PD_WIDTH]; 4],
    pub operator_constant_power_pannings: [[f64; MAX_PD_WIDTH]; 4],
    pub operator_envelope_volumes: [[f64; MAX_PD_WIDTH]; 4],
    pub operator_phases: [[f64; MAX_PD_WIDTH]; 4],
    pub operator_wave_type: [crate::WaveType; 4],
    pub operator_modulation_targets: [usize; 4],
    pub volume_factors: [f64; 4],
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
                        position,
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
                                &mut rights[position..end_position],
                                position,
                            );
                        } else {
                            FallbackStd::process_f32(
                                octasine,
                                &mut lefts[position..end_position],
                                &mut rights[position..end_position],
                                position,
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
    use super::*;

    #[feature_gate]
    impl AudioGen for S {
        #[target_feature_enable]
        unsafe fn process_f32(
            octasine: &mut OctaSine,
            lefts: &mut [f32],
            rights: &mut [f32],
            position: usize,
        ) {
            assert_eq!(lefts.len(), S::SAMPLES);
            assert_eq!(rights.len(), S::SAMPLES);

            if octasine.processing.pending_midi_events.is_empty()
                && !octasine.processing.voices.iter().any(|v| v.active)
            {
                for (l, r) in lefts.iter_mut().zip(rights.iter_mut()) {
                    *l = 0.0;
                    *r = 0.0;
                }

                return;
            }

            extract_voice_data(octasine, position);
            gen_audio(
                &mut octasine.processing.rng,
                &octasine.processing.audio_gen_voice_data,
                lefts,
                rights,
            );
        }
    }

    #[feature_gate]
    #[target_feature_enable]
    unsafe fn extract_voice_data(octasine: &mut OctaSine, position: usize) {
        for voice_data in octasine.processing.audio_gen_voice_data.iter_mut() {
            voice_data.active = false;
        }

        for sample_index in 0..S::SAMPLES {
            let sample_index_offset = sample_index * 2;
            let time_per_sample = octasine.processing.time_per_sample;
            let bpm = octasine.get_bpm();

            octasine.processing.parameters.advance_one_sample();

            // Process events for position in buffer
            loop {
                match octasine
                    .processing
                    .pending_midi_events
                    .get(0)
                    .map(|e| e.delta_frames as usize)
                {
                    Some(event_delta_frames) if event_delta_frames == position + sample_index => {
                        let event = octasine.processing.pending_midi_events.pop_front().unwrap();

                        octasine.process_midi_event(event);
                    }
                    _ => break,
                }
            }

            let operators = &mut octasine.processing.parameters.operators;

            for (voice, voice_data) in octasine
                .processing
                .voices
                .iter_mut()
                .zip(octasine.processing.audio_gen_voice_data.iter_mut())
                .filter(|(voice, _)| voice.active)
            {
                for (operator_index, operator) in operators.iter_mut().enumerate() {
                    voice.operators[operator_index]
                        .volume_envelope
                        .advance_one_sample(
                            &operator.volume_envelope,
                            voice.key_pressed,
                            time_per_sample,
                        );
                }

                voice.deactivate_if_envelopes_ended();

                if voice.active {
                    voice_data.active = true;
                } else {
                    // If voice was deactivated during first sample in avx mode, ensure
                    // audio isn't generated for second sample (as long as voice isn't
                    // reactivated by midi events)
                    if (S::SAMPLES == 2) & (sample_index == 0) {
                        for volumes in voice_data.operator_envelope_volumes.iter_mut() {
                            volumes[2] = 0.0;
                            volumes[3] = 0.0;
                        }
                    }
                }

                let lfo_values = get_lfo_target_values(
                    &mut octasine.processing.parameters.lfos,
                    &mut voice.lfos,
                    time_per_sample,
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

                    voice_data.operator_pannings[operator_index][sample_index_offset] = panning;
                    voice_data.operator_pannings[operator_index][sample_index_offset + 1] = panning;
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

                    let last_phase = voice.operators[operator_index].last_phase.0;
                    let phase_addition = frequency * time_per_sample.0;

                    let new_phase = last_phase + phase_addition;

                    voice_data.operator_phases[operator_index][sample_index_offset] = new_phase;
                    voice_data.operator_phases[operator_index][sample_index_offset + 1] = new_phase;

                    // Save phase
                    voice.operators[operator_index].last_phase.0 = new_phase;
                }

                // Envelope
                for (operator_index, operator) in operators.iter_mut().enumerate() {
                    let v = voice.operators[operator_index]
                        .volume_envelope
                        .get_volume(&operator.volume_envelope);

                    voice_data.operator_envelope_volumes[operator_index][sample_index_offset] = v;
                    voice_data.operator_envelope_volumes[operator_index][sample_index_offset + 1] =
                        v;
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
            }
        }
    }

    #[feature_gate]
    #[target_feature_enable]
    unsafe fn gen_audio(
        rng: &mut fastrand::Rng,
        voice_data: &[VoiceData; 128],
        audio_buffer_lefts: &mut [f32],
        audio_buffer_rights: &mut [f32],
    ) {
        // Maybe operator indexes should be inversed (3 - operator_index)
        // because that is how they will be accessed later.

        // S::SAMPLES * 2 because of two channels. Even index = left channel
        let mut summed_additive_outputs = S::pd_setzero();

        for voice_data in voice_data.iter().filter(|voice_data| voice_data.active) {
            let operator_generate_audio = run_operator_dependency_analysis(voice_data);

            // --- Generate samples for all operators

            // Voice modulation input storage, indexed by operator
            let mut voice_modulation_inputs = [S::pd_setzero(); 4];

            // Go through operators downwards, starting with operator 4
            for operator_index in 0..4 {
                // FIXME: better iterator with 3, 2, 1, 0 possible?
                let operator_index = 3 - operator_index;

                // Possibly skip generation based on previous dependency analysis
                if !operator_generate_audio[operator_index] {
                    continue;
                }

                let modulation_target = voice_data.operator_modulation_targets[operator_index];

                let sample =
                    if voice_data.operator_wave_type[operator_index] == WaveType::WhiteNoise {
                        let mut random_numbers = [0.0f64; S::PD_WIDTH];

                        for sample_index in 0..S::SAMPLES {
                            let random = rng.f64();

                            let sample_index_offset = sample_index * 2;

                            random_numbers[sample_index_offset] = random;
                            random_numbers[sample_index_offset + 1] = random;
                        }

                        let random_numbers = S::pd_loadu(random_numbers.as_ptr());

                        // Convert random numbers to range -1.0 to 1.0
                        S::pd_mul(S::pd_set1(2.0), S::pd_sub(random_numbers, S::pd_set1(0.5)))
                    } else {
                        let pan_tendency = {
                            let pan =
                                S::pd_loadu(voice_data.operator_pannings[operator_index].as_ptr());

                            // Get panning as value between -1 and 1
                            let pan = S::pd_mul(S::pd_set1(2.0), S::pd_sub(pan, S::pd_set1(0.5)));

                            S::pd_max(
                                S::pd_mul(pan, S::pd_distribute_left_right(-1.0, 1.0)),
                                S::pd_setzero(),
                            )
                        };

                        let one_minus_pan_tendency = S::pd_sub(S::pd_set1(1.0), pan_tendency);

                        let modulation_in_for_channel = voice_modulation_inputs[operator_index];
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

                let envelope_volume =
                    S::pd_loadu(voice_data.operator_envelope_volumes[operator_index].as_ptr());
                let volume_product = S::pd_mul(operator_volume, envelope_volume);

                let sample_adjusted =
                    S::pd_mul(S::pd_mul(sample, volume_product), constant_power_panning);
                let additive_out = S::pd_mul(sample_adjusted, operator_additive);
                let modulation_out = S::pd_sub(sample_adjusted, additive_out);

                // Add modulation output to target operator's modulation inputs
                voice_modulation_inputs[modulation_target] =
                    S::pd_add(voice_modulation_inputs[modulation_target], modulation_out);

                let addition = S::pd_mul(
                    additive_out,
                    S::pd_loadu(voice_data.volume_factors.as_ptr()),
                );

                summed_additive_outputs = S::pd_add(summed_additive_outputs, addition);
            }
        }

        // --- Summed additive outputs: apply hard limit.

        summed_additive_outputs = S::pd_min(summed_additive_outputs, S::pd_set1(5.0));
        summed_additive_outputs = S::pd_max(summed_additive_outputs, S::pd_set1(-5.0));

        // --- Write additive outputs to audio buffer

        let mut out = [0.0f64; S::PD_WIDTH];

        S::pd_storeu(out.as_mut_ptr(), summed_additive_outputs);

        for sample_index in 0..S::SAMPLES {
            let sample_index_offset = sample_index * 2;

            audio_buffer_lefts[sample_index] = out[sample_index_offset] as f32;
            audio_buffer_rights[sample_index] = out[sample_index_offset + 1] as f32;
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
            let envelope_volume =
                S::pd_loadu(voice_data.operator_envelope_volumes[operator_index].as_ptr());

            if S::pd_any_over_zero(S::pd_mul(operator_volume, envelope_volume)) {
                operator_additive_zero[operator_index] = !S::pd_any_over_zero(S::pd_loadu(
                    voice_data.operator_additives[operator_index].as_ptr(),
                ));
                operator_modulation_index_zero[operator_index] = !S::pd_any_over_zero(S::pd_loadu(
                    voice_data.operator_modulation_indices[operator_index].as_ptr(),
                ));
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
