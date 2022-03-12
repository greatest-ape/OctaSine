mod lfo;
pub mod simd;

use std::f64::consts::TAU;

use duplicate::duplicate;
use vst::buffer::AudioBuffer;

use crate::approximations::Log10Table;
use crate::common::*;
use crate::constants::*;
use crate::parameters::processing::{ProcessingParameter, ProcessingParameterOperator};
use crate::{OctaSine, ProcessingState};

use lfo::*;
use simd::*;

const MAX_PD_WIDTH: usize = 4;

pub trait AudioGen {
    #[allow(clippy::missing_safety_doc)]
    unsafe fn process_f32(
        octasine: &mut ProcessingState,
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
pub struct OperatorVoiceData {
    pub mixes: [f64; MAX_PD_WIDTH],
    pub modulation_indices: [f64; MAX_PD_WIDTH],
    pub feedbacks: [f64; MAX_PD_WIDTH],
    pub pannings: [f64; MAX_PD_WIDTH],
    pub constant_power_pannings: [f64; MAX_PD_WIDTH],
    pub envelope_volumes: [f64; MAX_PD_WIDTH],
    pub phases: [f64; MAX_PD_WIDTH],
    pub wave_type: crate::WaveType,
    pub modulation_target: usize,
    pub volume_factors: [f64; MAX_PD_WIDTH],
}

#[derive(Debug, Default)]
pub struct VoiceData {
    pub active: bool,
    pub volume_factors: [f64; MAX_PD_WIDTH],
    pub operators: [OperatorVoiceData; 4],
}

#[inline]
pub fn process_f32_runtime_select(octasine: &mut OctaSine, audio_buffer: &mut AudioBuffer<f32>) {
    octasine.update_processing_parameters();
    octasine.update_bpm();

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
                        &mut octasine.processing,
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
                                &mut octasine.processing,
                                &mut lefts[position..end_position],
                                &mut rights[position..end_position],
                                position,
                            );
                        } else {
                            FallbackStd::process_f32(
                                &mut octasine.processing,
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
            processing: &mut ProcessingState,
            lefts: &mut [f32],
            rights: &mut [f32],
            position: usize,
        ) {
            assert_eq!(lefts.len(), S::SAMPLES);
            assert_eq!(rights.len(), S::SAMPLES);

            if processing.pending_midi_events.is_empty()
                && !processing.voices.iter().any(|v| v.active)
            {
                for (l, r) in lefts.iter_mut().zip(rights.iter_mut()) {
                    *l = 0.0;
                    *r = 0.0;
                }

                return;
            }

            extract_voice_data(processing, position);
            gen_audio(
                &mut processing.rng,
                &processing.audio_gen_voice_data,
                lefts,
                rights,
            );
        }
    }

    #[feature_gate]
    #[target_feature_enable]
    unsafe fn extract_voice_data(processing: &mut ProcessingState, position: usize) {
        for voice_data in processing.audio_gen_voice_data.iter_mut() {
            voice_data.active = false;
        }

        for sample_index in 0..S::SAMPLES {
            let time_per_sample = processing.time_per_sample;

            processing.parameters.advance_one_sample();
            processing.process_events_for_sample(position + sample_index);

            let operators = &mut processing.parameters.operators;

            for (voice, voice_data) in processing
                .voices
                .iter_mut()
                .zip(processing.audio_gen_voice_data.iter_mut())
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
                        for operator in voice_data.operators.iter_mut() {
                            set_value_for_both_channels(&mut operator.envelope_volumes, 1, 0.0);
                        }
                    }
                }

                let lfo_values = get_lfo_target_values(
                    &mut processing.parameters.lfos,
                    &mut voice.lfos,
                    time_per_sample,
                    processing.bpm,
                );

                let voice_volume_factor = {
                    let lfo_parameter =
                        LfoTargetParameter::Master(LfoTargetMasterParameter::Volume);
                    let lfo_addition = lfo_values.get(lfo_parameter);

                    let master_volume = processing
                        .parameters
                        .master_volume
                        .get_value_with_lfo_addition(lfo_addition);

                    let key_velocity = voice.key_velocity.0;

                    VOICE_VOLUME_FACTOR * master_volume * key_velocity
                };

                set_value_for_both_channels(
                    &mut voice_data.volume_factors,
                    sample_index,
                    voice_volume_factor,
                );

                let voice_base_frequency = voice.midi_pitch.get_frequency(
                    processing
                        .parameters
                        .master_frequency
                        .get_value_with_lfo_addition(lfo_values.get(LfoTargetParameter::Master(
                            LfoTargetMasterParameter::Frequency,
                        ))),
                );

                for (operator_index, operator) in operators.iter_mut().enumerate() {
                    extract_voice_operator_data(
                        &processing.log10table,
                        sample_index,
                        operator_index,
                        operator,
                        &mut voice.operators[operator_index],
                        &mut voice_data.operators[operator_index],
                        &lfo_values,
                        time_per_sample,
                        voice_base_frequency,
                    )
                }
            }
        }
    }

    #[feature_gate]
    #[target_feature_enable]
    unsafe fn extract_voice_operator_data(
        log10table: &Log10Table,
        sample_index: usize,
        operator_index: usize,
        operator: &mut ProcessingParameterOperator,
        voice_operator: &mut crate::voices::VoiceOperator,
        voice_data: &mut OperatorVoiceData,
        lfo_values: &LfoTargetValues,
        time_per_sample: TimePerSample,
        voice_base_frequency: f64,
    ) {
        voice_data.wave_type = operator.wave_type.value;

        if let Some(p) = &mut operator.output_operator {
            voice_data.modulation_target = p.get_value();
        }

        let envelope_volume = voice_operator
            .volume_envelope
            .get_volume(log10table, &operator.volume_envelope);

        set_value_for_both_channels(
            &mut voice_data.envelope_volumes,
            sample_index,
            envelope_volume,
        );

        let volume =
            operator
                .mix
                .get_value_with_lfo_addition(lfo_values.get(LfoTargetParameter::Operator(
                    operator_index,
                    LfoTargetOperatorParameter::Volume,
                )));

        set_value_for_both_channels(&mut voice_data.mixes, sample_index, volume);

        let modulation_index = operator.modulation_index.as_mut().map_or(0.0, |p| {
            p.get_value_with_lfo_addition(lfo_values.get(LfoTargetParameter::Operator(
                operator_index,
                LfoTargetOperatorParameter::ModulationIndex,
            )))
        });

        set_value_for_both_channels(
            &mut voice_data.modulation_indices,
            sample_index,
            modulation_index,
        );

        let feedback = operator
            .feedback
            .get_value_with_lfo_addition(lfo_values.get(LfoTargetParameter::Operator(
                operator_index,
                LfoTargetOperatorParameter::Feedback,
            )));

        set_value_for_both_channels(&mut voice_data.feedbacks, sample_index, feedback);

        let panning = operator.panning.get_value_with_lfo_addition(lfo_values.get(
            LfoTargetParameter::Operator(operator_index, LfoTargetOperatorParameter::Panning),
        ));

        set_value_for_both_channels(&mut voice_data.pannings, sample_index, panning);

        {
            let [l, r] = operator.panning.left_and_right;

            let sample_index_offset = sample_index * 2;

            voice_data.constant_power_pannings[sample_index_offset] = l;
            voice_data.constant_power_pannings[sample_index_offset + 1] = r;
        }

        let frequency_ratio = operator
            .frequency_ratio
            .get_value_with_lfo_addition(lfo_values.get(LfoTargetParameter::Operator(
                operator_index,
                LfoTargetOperatorParameter::FrequencyRatio,
            )));
        let frequency_free = operator
            .frequency_free
            .get_value_with_lfo_addition(lfo_values.get(LfoTargetParameter::Operator(
                operator_index,
                LfoTargetOperatorParameter::FrequencyFree,
            )));
        let frequency_fine = operator
            .frequency_fine
            .get_value_with_lfo_addition(lfo_values.get(LfoTargetParameter::Operator(
                operator_index,
                LfoTargetOperatorParameter::FrequencyFine,
            )));

        let frequency = voice_base_frequency * frequency_ratio * frequency_free * frequency_fine;
        let new_phase = voice_operator.last_phase.0 + frequency * time_per_sample.0;

        set_value_for_both_channels(&mut voice_data.phases, sample_index, new_phase);

        // Save phase
        voice_operator.last_phase.0 = new_phase;
    }

    #[feature_gate]
    #[target_feature_enable]
    unsafe fn gen_audio(
        rng: &mut fastrand::Rng,
        voice_data: &[VoiceData; 128],
        audio_buffer_lefts: &mut [f32],
        audio_buffer_rights: &mut [f32],
    ) {
        // S::SAMPLES * 2 because of two channels. Even index = left channel
        let mut summed_additive_outputs = S::pd_setzero();

        for voice_data in voice_data.iter().filter(|voice_data| voice_data.active) {
            let operator_generate_audio = run_operator_dependency_analysis(voice_data);

            // Voice modulation input storage, indexed by operator
            let mut voice_modulation_inputs = [S::pd_setzero(); 4];

            let volume_factors = S::pd_loadu(voice_data.volume_factors.as_ptr());

            // Go through operators downwards, starting with operator 4
            for operator_index in (0..4).map(|i| 3 - i) {
                // Possibly skip generation based on previous dependency analysis
                if !operator_generate_audio[operator_index] {
                    continue;
                }

                let operator_voice_data = &voice_data.operators[operator_index];

                let (additive_out, modulation_out) = gen_voice_operator_audio(
                    rng,
                    operator_voice_data,
                    voice_modulation_inputs[operator_index],
                );

                summed_additive_outputs = S::pd_add(
                    summed_additive_outputs,
                    S::pd_mul(additive_out, volume_factors),
                );

                // Add modulation output to target operator's modulation inputs
                voice_modulation_inputs[operator_voice_data.modulation_target] = S::pd_add(
                    voice_modulation_inputs[operator_voice_data.modulation_target],
                    modulation_out,
                );
            }
        }

        // Apply hard limit

        summed_additive_outputs = S::pd_min(summed_additive_outputs, S::pd_set1(5.0));
        summed_additive_outputs = S::pd_max(summed_additive_outputs, S::pd_set1(-5.0));

        // Write additive outputs to audio buffer

        let mut out = [0.0f64; S::PD_WIDTH];

        S::pd_storeu(out.as_mut_ptr(), summed_additive_outputs);

        for sample_index in 0..S::SAMPLES {
            let sample_index_offset = sample_index * 2;

            audio_buffer_lefts[sample_index] = out[sample_index_offset] as f32;
            audio_buffer_rights[sample_index] = out[sample_index_offset + 1] as f32;
        }
    }

    #[feature_gate]
    #[target_feature_enable]
    unsafe fn gen_voice_operator_audio(
        rng: &mut fastrand::Rng,
        voice_data: &OperatorVoiceData,
        modulation_inputs: <S as Simd>::PackedDouble,
    ) -> (<S as Simd>::PackedDouble, <S as Simd>::PackedDouble) {
        let sample = if voice_data.wave_type == WaveType::WhiteNoise {
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
            // Weird modulation input panning
            // Mix modulator into current operator depending on
            // panning of current operator. If panned to the
            // middle, just pass through the stereo signals. If
            // panned to any side, mix out the original stereo
            // signals and mix in mono.
            // Note: breaks unless S::PD_WIDTH >= 2
            let modulation_in = {
                let pan = S::pd_loadu(voice_data.pannings.as_ptr());

                // Get panning as value between -1 and 1
                let pan = S::pd_mul(S::pd_set1(2.0), S::pd_sub(pan, S::pd_set1(0.5)));

                let pan_tendency = S::pd_max(
                    S::pd_mul(pan, S::pd_distribute_left_right(-1.0, 1.0)),
                    S::pd_setzero(),
                );
                let one_minus_pan_tendency = S::pd_sub(S::pd_set1(1.0), pan_tendency);

                let modulation_in_channel_sum = S::pd_pairwise_horizontal_sum(modulation_inputs);

                S::pd_add(
                    S::pd_mul(pan_tendency, modulation_in_channel_sum),
                    S::pd_mul(one_minus_pan_tendency, modulation_inputs),
                )
            };

            let phase = S::pd_mul(S::pd_loadu(voice_data.phases.as_ptr()), S::pd_set1(TAU));

            let feedback = S::pd_mul(
                S::pd_loadu(voice_data.feedbacks.as_ptr()),
                S::pd_fast_sin(phase),
            );

            S::pd_fast_sin(S::pd_add(phase, S::pd_add(feedback, modulation_in)))
        };

        let sample = {
            let envelope_volume = S::pd_loadu(voice_data.envelope_volumes.as_ptr());
            let constant_power_panning = S::pd_loadu(voice_data.constant_power_pannings.as_ptr());

            S::pd_mul(S::pd_mul(sample, envelope_volume), constant_power_panning)
        };

        let additive_out = S::pd_mul(sample, S::pd_loadu(voice_data.mixes.as_ptr()));
        let modulation_out = S::pd_mul(sample, S::pd_loadu(voice_data.modulation_indices.as_ptr()));

        (additive_out, modulation_out)
    }

    /// Operator dependency analysis to allow skipping audio generation when possible
    #[feature_gate]
    #[target_feature_enable]
    unsafe fn run_operator_dependency_analysis(voice_data: &VoiceData) -> [bool; 4] {
        let mut operator_generate_audio = [true; 4];
        let mut operator_mix_zero = [false; 4];
        let mut operator_modulation_index_zero = [false; 4];

        for operator_index in 0..4 {
            let operator_mix = S::pd_loadu(voice_data.operators[operator_index].mixes.as_ptr());
            let modulation_index = S::pd_loadu(
                voice_data.operators[operator_index]
                    .modulation_indices
                    .as_ptr(),
            );

            let mix_active = S::pd_any_over_zero(operator_mix);
            let modulation_active = S::pd_any_over_zero(modulation_index);

            operator_generate_audio[operator_index] = mix_active | modulation_active;

            operator_modulation_index_zero[operator_index] = modulation_active;
            operator_mix_zero[operator_index] = mix_active;
        }

        for _ in 0..3 {
            for operator_index in 1..4 {
                let modulation_target = voice_data.operators[operator_index].modulation_target;

                // Skip generation if operator was previously determined to be skippable OR
                #[rustfmt::skip]
                let skip_condition = !operator_generate_audio[operator_index] | (
                    // Operator mix is 0.0 AND
                    operator_mix_zero[operator_index] & (
                        // Modulation target was previously determined to be skippable OR
                        !operator_generate_audio[modulation_target] |
                        // Modulation target is white noise OR
                        (voice_data.operators[modulation_target].wave_type == WaveType::WhiteNoise)
                    )
                );

                if skip_condition {
                    operator_generate_audio[operator_index] = false;
                }
            }
        }

        operator_generate_audio
    }

    #[feature_gate]
    #[target_feature_enable]
    unsafe fn set_value_for_both_channels(
        target: &mut [f64; MAX_PD_WIDTH],
        sample_index: usize,
        value: f64,
    ) {
        let offset = sample_index * 2;

        target[offset] = value;
        target[offset + 1] = value;
    }
}
