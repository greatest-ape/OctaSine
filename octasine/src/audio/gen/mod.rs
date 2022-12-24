pub mod lfo;

use std::f64::consts::TAU;

use duplicate::duplicate_item;

use crate::audio::parameters::{common::AudioParameter, OperatorAudioParameters};
use crate::audio::voices::log10_table::Log10Table;
use crate::audio::AudioState;
use crate::common::*;
use crate::parameters::operator_wave_type::WaveType;
use crate::parameters::{MasterParameter, ModTargetStorage, OperatorParameter, Parameter};
use crate::simd::*;

use lfo::*;

const MASTER_VOLUME_FACTOR: f64 = 0.2;
const LIMIT: f64 = 10.0;

pub trait AudioGen {
    #[allow(clippy::missing_safety_doc)]
    unsafe fn process_f32(
        octasine: &mut AudioState,
        lefts: &mut [f32],
        rights: &mut [f32],
        position: usize,
    );
}

/// Audio gen data cache.
///
/// Data is only valid for the duration of the processing of one or two
/// (stereo) samples, depending on the SIMD instruction width.
pub struct AudioGenData<const W: usize> {
    lfo_target_values: LfoTargetValues,
    voices: [VoiceData<W>; 128],
}

impl<const W: usize> Default for AudioGenData<W> {
    fn default() -> Self {
        Self {
            lfo_target_values: Default::default(),
            voices: array_init::array_init(|_| Default::default()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct VoiceData<const W: usize> {
    voice_index: u8,
    key_velocity: [f64; W],
    /// Master volume is calculated per-voice, since it can be an LFO target
    master_volume: [f64; W],
    operators: [VoiceOperatorData<W>; 4],
}

impl<const W: usize> Default for VoiceData<W> {
    fn default() -> Self {
        Self {
            voice_index: 0,
            key_velocity: [0.0; W],
            /// Master volume is calculated per-voice, since it can be an LFO target
            master_volume: [0.0; W],
            operators: Default::default(),
        }
    }
}

impl<const W: usize> VoiceData<W> {
    /// Set envelope volumes to zero to prevent audio from being generated due to
    /// invalid data from previous passes
    #[inline]
    fn reset_envelope_volumes(&mut self) {
        self.operators[0].envelope_volume.copy_from_slice(&[0.0; W]);
        self.operators[1].envelope_volume.copy_from_slice(&[0.0; W]);
        self.operators[2].envelope_volume.copy_from_slice(&[0.0; W]);
        self.operators[3].envelope_volume.copy_from_slice(&[0.0; W]);
    }
}

#[derive(Debug, Clone, Copy)]
struct VoiceOperatorData<const W: usize> {
    volume: [f64; W],
    mix_out: [f64; W],
    mod_out: [f64; W],
    feedback: [f64; W],
    panning: [f64; W],
    constant_power_panning: [f64; W],
    envelope_volume: [f64; W],
    phase: [f64; W],
    wave_type: WaveType,
    modulation_targets: ModTargetStorage,
}

impl<const W: usize> Default for VoiceOperatorData<W> {
    fn default() -> Self {
        Self {
            volume: [0.0; W],
            mix_out: [0.0; W],
            mod_out: [0.0; W],
            feedback: [0.0; W],
            panning: [0.0; W],
            constant_power_panning: [0.0; W],
            envelope_volume: [0.0; W],
            phase: [0.0; W],
            wave_type: Default::default(),
            modulation_targets: Default::default(),
        }
    }
}

#[inline]
pub fn process_f32_runtime_select(
    audio_state: &mut AudioState,
    lefts: &mut [f32],
    rights: &mut [f32],
    frame_offset: usize,
) {
    let num_samples = lefts.len();

    let mut position = 0;

    loop {
        let num_remaining_samples = (num_samples - position) as u64;

        unsafe {
            match num_remaining_samples {
                #[cfg(target_arch = "x86_64")]
                (2..) if is_x86_feature_detected!("avx") => {
                    let new_position = position + 2;

                    Avx::process_f32(
                        audio_state,
                        &mut lefts[position..new_position],
                        &mut rights[position..new_position],
                        frame_offset + position,
                    );

                    position = new_position;
                }
                #[cfg(target_arch = "x86_64")]
                1.. => {
                    let new_position = position + 1;

                    Sse2::process_f32(
                        audio_state,
                        &mut lefts[position..new_position],
                        &mut rights[position..new_position],
                        frame_offset + position,
                    );

                    position = new_position;
                }
                #[cfg(not(target_arch = "x86_64"))]
                1.. => {
                    let new_position = position + 1;

                    Fallback::process_f32(
                        audio_state,
                        &mut lefts[position..new_position],
                        &mut rights[position..new_position],
                        frame_offset + position,
                    );

                    position = new_position;
                }
                0 => {
                    break;
                }
            }
        }
    }
}

#[duplicate_item(
    [
        S [ Fallback ]
        target_feature_enable [ cfg(not(feature = "fake-feature")) ]
        feature_gate [ cfg(not(feature = "fake-feature")) ]
        test_feature_gate [ cfg(not(feature = "fake-feature")) ]
        audio_gen_data_field [ audio_gen_data_w2 ]
    ]
    [
        S [ Sse2 ]
        target_feature_enable [ cfg(not(feature = "fake-feature")) ]
        feature_gate [ cfg(target_arch = "x86_64") ]
        test_feature_gate [ cfg(all(target_arch = "x86_64")) ]
        audio_gen_data_field [ audio_gen_data_w2 ]
    ]
    [
        S [ Avx ]
        target_feature_enable [ target_feature(enable = "avx") ]
        feature_gate [ cfg(target_arch = "x86_64") ]
        test_feature_gate [ cfg(all(target_arch = "x86_64", target_feature = "avx")) ]
        audio_gen_data_field [ audio_gen_data_w4 ]
    ]
)]
mod gen {
    #[feature_gate]
    use super::*;

    #[feature_gate]
    type Pd = <S as Simd>::Pd;

    #[feature_gate]
    impl AudioGen for S {
        #[target_feature_enable]
        unsafe fn process_f32(
            audio_state: &mut AudioState,
            lefts: &mut [f32],
            rights: &mut [f32],
            position: usize,
        ) {
            assert_eq!(lefts.len(), Pd::SAMPLES);
            assert_eq!(rights.len(), Pd::SAMPLES);

            if audio_state.pending_note_events.is_empty()
                && !audio_state.voices.iter().any(|v| v.active)
            {
                for (l, r) in lefts.iter_mut().zip(rights.iter_mut()) {
                    *l = 0.0;
                    *r = 0.0;
                }

                return;
            }

            let num_valid_voice_datas = extract_voice_data(audio_state, position);

            gen_audio(
                &mut audio_state.rng,
                &audio_state.audio_gen_data_field.voices[..num_valid_voice_datas],
                lefts,
                rights,
            );
        }
    }

    #[feature_gate]
    #[target_feature_enable]
    unsafe fn extract_voice_data(audio_state: &mut AudioState, position: usize) -> usize {
        let mut num_valid_voice_datas = 0;

        for sample_index in 0..Pd::SAMPLES {
            let time_per_sample = audio_state.time_per_sample;

            audio_state
                .parameters
                .advance_one_sample(audio_state.sample_rate);
            audio_state.process_events_for_sample(position + sample_index);

            let operators = &mut audio_state.parameters.operators;
            let lfo_values = &mut audio_state.audio_gen_data_field.lfo_target_values;

            for (voice_index, voice) in audio_state
                .voices
                .iter_mut()
                .enumerate()
                .filter(|(_, voice)| voice.active)
                .map(|(voice_index, voice)| (voice_index as u8, voice))
            {
                // Select an appropriate VoiceData item to fill with data
                let voice_data = if sample_index == 0 {
                    let voice_data =
                        &mut audio_state.audio_gen_data_field.voices[num_valid_voice_datas];

                    voice_data.voice_index = voice_index;

                    voice_data.reset_envelope_volumes();

                    num_valid_voice_datas += 1;

                    voice_data
                } else {
                    // During second sample in AVX mode, look for the relevant voice data cache
                    // among the ones filled while processing sample 1. If it is not found because
                    // the voice was activated this sample, use a new one.
                    if let Some(voice_data) = audio_state.audio_gen_data_field.voices
                        [..num_valid_voice_datas]
                        .iter_mut()
                        .find(|voice_data| voice_data.voice_index == voice_index)
                    {
                        voice_data
                    } else {
                        let voice_data =
                            &mut audio_state.audio_gen_data_field.voices[num_valid_voice_datas];

                        voice_data.voice_index = voice_index;

                        voice_data.reset_envelope_volumes();

                        num_valid_voice_datas += 1;

                        voice_data
                    }
                };

                voice.advance_velocity_interpolator_one_sample(audio_state.sample_rate);

                for (operator_index, operator) in operators.iter_mut().enumerate() {
                    voice.operators[operator_index]
                        .volume_envelope
                        .advance_one_sample(
                            &operator.volume_envelope,
                            &mut voice.operators[operator_index].last_phase,
                            voice.key_pressed | audio_state.sustain_pedal_on,
                            time_per_sample,
                        );
                }

                update_lfo_target_values(
                    lfo_values,
                    &mut audio_state.parameters.lfos,
                    &mut voice.lfos,
                    audio_state.sample_rate,
                    time_per_sample,
                    audio_state.bpm_lfo_multiplier,
                );

                set_value_for_both_channels(
                    &mut voice_data.key_velocity,
                    sample_index,
                    voice.get_key_velocity().0 as f64,
                );

                const MASTER_VOLUME_INDEX: u8 =
                    Parameter::Master(MasterParameter::Volume).to_index();

                let master_volume = audio_state
                    .parameters
                    .master_volume
                    .get_value_with_lfo_addition(lfo_values.get(MASTER_VOLUME_INDEX));

                set_value_for_both_channels(
                    &mut voice_data.master_volume,
                    sample_index,
                    master_volume as f64,
                );

                const MASTER_FREQUENCY_INDEX: u8 =
                    Parameter::Master(MasterParameter::Frequency).to_index();

                let master_frequency = audio_state
                    .parameters
                    .master_frequency
                    .get_value_with_lfo_addition(lfo_values.get(MASTER_FREQUENCY_INDEX));

                let voice_base_frequency = voice.midi_pitch.get_frequency(master_frequency);

                for (operator_index, operator) in operators.iter_mut().enumerate() {
                    if voice.operators[operator_index].volume_envelope.is_ended() {
                        continue;
                    }

                    extract_voice_operator_data(
                        &audio_state.log10table,
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

                voice.deactivate_if_envelopes_ended(position + sample_index);
            }
        }

        num_valid_voice_datas
    }

    #[feature_gate]
    #[target_feature_enable]
    unsafe fn extract_voice_operator_data(
        log10table: &Log10Table,
        sample_index: usize,
        operator_index: usize,
        operator_parameters: &mut OperatorAudioParameters,
        voice_operator: &mut crate::audio::voices::VoiceOperator,
        operator_data: &mut VoiceOperatorData<{ Pd::WIDTH }>,
        lfo_values: &LfoTargetValues,
        time_per_sample: TimePerSample,
        voice_base_frequency: f64,
    ) {
        const VOLUME_INDICES: [u8; NUM_OPERATORS] = OperatorParameter::Volume.index_array();
        const MIX_INDICES: [u8; NUM_OPERATORS] = OperatorParameter::MixOut.index_array();
        /// Note: MOD_INDICES index 0 is invalid (0) and must never be used
        const MOD_INDICES: [u8; NUM_OPERATORS] = OperatorParameter::ModOut.index_array();
        const FEEDBACK_INDICES: [u8; NUM_OPERATORS] = OperatorParameter::Feedback.index_array();
        const PANNING_INDICES: [u8; NUM_OPERATORS] = OperatorParameter::Panning.index_array();
        const RATIO_INDICES: [u8; NUM_OPERATORS] = OperatorParameter::FrequencyRatio.index_array();
        const FREE_INDICES: [u8; NUM_OPERATORS] = OperatorParameter::FrequencyFree.index_array();
        const FINE_INDICES: [u8; NUM_OPERATORS] = OperatorParameter::FrequencyFine.index_array();

        assert!(operator_index < NUM_OPERATORS);

        operator_data.wave_type = operator_parameters.wave_type.get_value();

        if let Some(p) = &mut operator_parameters.mod_targets {
            operator_data.modulation_targets = p.get_value();
        }

        let envelope_volume = voice_operator
            .volume_envelope
            .get_volume(log10table, &operator_parameters.volume_envelope);

        set_value_for_both_channels(
            &mut operator_data.envelope_volume,
            sample_index,
            envelope_volume as f64,
        );

        let volume = operator_parameters
            .volume
            .get_value_with_lfo_addition(lfo_values.get(VOLUME_INDICES[operator_index]));

        let volume_active = operator_parameters.active.get_value();

        set_value_for_both_channels(
            &mut operator_data.volume,
            sample_index,
            (volume * volume_active) as f64,
        );

        let mix_out = operator_parameters
            .mix_out
            .get_value_with_lfo_addition(lfo_values.get(MIX_INDICES[operator_index]));

        set_value_for_both_channels(&mut operator_data.mix_out, sample_index, mix_out as f64);

        let mod_out = operator_parameters.mod_out.as_mut().map_or(0.0, |p| {
            p.get_value_with_lfo_addition(lfo_values.get(MOD_INDICES[operator_index]))
        });

        set_value_for_both_channels(&mut operator_data.mod_out, sample_index, mod_out as f64);

        let feedback = operator_parameters
            .feedback
            .get_value_with_lfo_addition(lfo_values.get(FEEDBACK_INDICES[operator_index]));

        set_value_for_both_channels(&mut operator_data.feedback, sample_index, feedback as f64);

        let panning = operator_parameters
            .panning
            .get_value_with_lfo_addition(lfo_values.get(PANNING_INDICES[operator_index]));

        set_value_for_both_channels(&mut operator_data.panning, sample_index, panning as f64);

        {
            let [l, r] = operator_parameters.panning.left_and_right;

            let sample_index_offset = sample_index * 2;

            operator_data.constant_power_panning[sample_index_offset] = l as f64;
            operator_data.constant_power_panning[sample_index_offset + 1] = r as f64;
        }

        let frequency_ratio = operator_parameters
            .frequency_ratio
            .get_value_with_lfo_addition(lfo_values.get(RATIO_INDICES[operator_index]));
        let frequency_free = operator_parameters
            .frequency_free
            .get_value_with_lfo_addition(lfo_values.get(FREE_INDICES[operator_index]));
        let frequency_fine = operator_parameters
            .frequency_fine
            .get_value_with_lfo_addition(lfo_values.get(FINE_INDICES[operator_index]));

        let frequency =
            voice_base_frequency * frequency_ratio.value * frequency_free * frequency_fine;
        let new_phase = voice_operator.last_phase.0 + frequency * time_per_sample.0;

        set_value_for_both_channels(&mut operator_data.phase, sample_index, new_phase);

        // Save phase
        voice_operator.last_phase.0 = new_phase;
    }

    #[feature_gate]
    #[target_feature_enable]
    unsafe fn gen_audio(
        rng: &mut fastrand::Rng,
        voices: &[VoiceData<{ Pd::WIDTH }>],
        audio_buffer_lefts: &mut [f32],
        audio_buffer_rights: &mut [f32],
    ) {
        // Pd::SAMPLES * 2 because of two channels. Even index = left channel
        let mut mix_out_sum = Pd::new_zeroed();

        for voice_data in voices.iter() {
            let operator_generate_audio = run_operator_dependency_analysis(voice_data);

            // Voice modulation input storage, indexed by operator
            let mut voice_modulation_inputs = [Pd::new_zeroed(); 4];

            let key_velocity = Pd::from_arr(voice_data.key_velocity);
            let master_volume = Pd::from_arr(voice_data.master_volume);

            // Go through operators downwards, starting with operator 4
            for operator_index in (0..4).map(|i| 3 - i) {
                // Possibly skip generation based on previous dependency analysis
                if !operator_generate_audio[operator_index] {
                    continue;
                }

                let operator_voice_data = &voice_data.operators[operator_index];

                let (mix_out, mod_out) = gen_voice_operator_audio(
                    rng,
                    operator_voice_data,
                    voice_modulation_inputs[operator_index],
                    key_velocity,
                );

                mix_out_sum += mix_out * master_volume;

                // Add modulation output to target operators' modulation inputs
                for target in operator_voice_data.modulation_targets.active_indices() {
                    voice_modulation_inputs[target] += mod_out;
                }
            }
        }

        let mix_out_arr = (mix_out_sum * Pd::new(MASTER_VOLUME_FACTOR))
            .min(Pd::new(LIMIT))
            .max(Pd::new(-LIMIT))
            .to_arr();

        for (sample_index, chunk) in mix_out_arr.chunks_exact(2).enumerate() {
            audio_buffer_lefts[sample_index] = chunk[0] as f32;
            audio_buffer_rights[sample_index] = chunk[1] as f32;
        }
    }

    #[feature_gate]
    #[target_feature_enable]
    unsafe fn gen_voice_operator_audio(
        rng: &mut fastrand::Rng,
        operator_data: &VoiceOperatorData<{ Pd::WIDTH }>,
        modulation_inputs: Pd,
        key_velocity: Pd,
    ) -> (Pd, Pd) {
        let sample = if operator_data.wave_type == WaveType::WhiteNoise {
            let mut random_numbers = <Pd as SimdPackedDouble>::Arr::default();

            for chunk in random_numbers.chunks_exact_mut(2) {
                let random = rng.f64();

                chunk[0] = random;
                chunk[1] = random;
            }

            // Convert random numbers to range -1.0 to 1.0
            Pd::new(2.0) * (Pd::from_arr(random_numbers) - Pd::new(0.5))
        } else {
            let phase = Pd::from_arr(operator_data.phase);
            let feedback = Pd::from_arr(operator_data.feedback);

            let phase = phase * Pd::new(TAU);

            (phase + (key_velocity * (feedback * phase.fast_sin()) + modulation_inputs)).fast_sin()
        };

        let volume = Pd::from_arr(operator_data.volume);
        let envelope_volume = Pd::from_arr(operator_data.envelope_volume);
        let panning = Pd::from_arr(operator_data.panning);

        let sample = sample * key_velocity * volume * envelope_volume;

        // Mix channels depending on panning of current operator. If panned to
        // the middle, just pass through the stereo signals. If panned to any
        // side, mix out the original stereo signals and mix in mono.
        let sample = {
            let mono_mix_factor = mono_mix_factor(panning);
            let mono = sample.pairwise_horizontal_sum() * Pd::new(0.5);

            (mono_mix_factor * mono) + ((Pd::new(1.0) - mono_mix_factor) * sample)
        };

        let mix_out = {
            let pan_factor = Pd::from_arr(operator_data.constant_power_panning);
            let mix_out = Pd::from_arr(operator_data.mix_out);

            sample * pan_factor * mix_out
        };
        let mod_out = {
            let pan_factor = linear_panning_factor(panning);
            let mod_out = Pd::from_arr(operator_data.mod_out);

            sample * pan_factor * mod_out
        };

        (mix_out, mod_out)
    }

    /// Operator dependency analysis to allow skipping audio generation when possible
    #[feature_gate]
    #[target_feature_enable]
    unsafe fn run_operator_dependency_analysis(voice_data: &VoiceData<{ Pd::WIDTH }>) -> [bool; 4] {
        let mut operator_generate_audio = [true; 4];
        let mut operator_mix_out_active = [false; 4];

        for operator_index in 0..4 {
            let volume = Pd::from_arr(voice_data.operators[operator_index].volume);
            let mix_out = Pd::from_arr(voice_data.operators[operator_index].mix_out);
            let mod_out = Pd::from_arr(voice_data.operators[operator_index].mod_out);

            let volume_active = volume.any_over_zero();
            let mix_out_active = mix_out.any_over_zero();
            let mod_out_active = mod_out.any_over_zero();

            operator_generate_audio[operator_index] =
                volume_active & (mod_out_active | mix_out_active);
            operator_mix_out_active[operator_index] = mix_out_active;
        }

        for operator_index in 1..4 {
            let all_targets_inactive = voice_data.operators[operator_index]
                .modulation_targets
                .active_indices()
                .all(|mod_target| !operator_generate_audio[mod_target]);

            if all_targets_inactive & !operator_mix_out_active[operator_index] {
                operator_generate_audio[operator_index] = false;
            }
        }

        operator_generate_audio
    }

    #[feature_gate]
    #[target_feature_enable]
    unsafe fn set_value_for_both_channels(
        target: &mut [f64; Pd::WIDTH],
        sample_index: usize,
        value: f64,
    ) {
        let offset = sample_index * 2;

        target[offset..offset + 2].copy_from_slice(&[value, value]);
    }

    /// Linear panning. Get channel volume as number between 0.0 and 1.0
    #[feature_gate]
    #[target_feature_enable]
    unsafe fn linear_panning_factor(panning: Pd) -> Pd {
        ((Pd::new(1.0) - panning).interleave(panning) * Pd::new(2.0)).min(Pd::new(1.0))
    }

    /// Get amount of channel that should be derived from mono for stereo mix
    /// panning
    #[feature_gate]
    #[target_feature_enable]
    unsafe fn mono_mix_factor(panning: Pd) -> Pd {
        // Get panning as value between -1 and 1
        let pan = Pd::new(2.0) * (panning - Pd::new(0.5));

        (pan * Pd::new_from_pair(-1.0, 1.0)).max(Pd::new_zeroed())
    }

    #[cfg(test)]
    mod tests {
        #[test_feature_gate]
        use super::*;

        #[test_feature_gate]
        #[test]
        fn test_linear_panning_factor() {
            unsafe {
                assert_eq!(
                    Pd::to_arr(linear_panning_factor(Pd::new(0.0))),
                    Pd::to_arr(Pd::new_from_pair(1.0, 0.0))
                );
                assert_eq!(
                    Pd::to_arr(linear_panning_factor(Pd::new(0.25))),
                    Pd::to_arr(Pd::new_from_pair(1.0, 0.5))
                );
                assert_eq!(
                    Pd::to_arr(linear_panning_factor(Pd::new(0.5))),
                    Pd::to_arr(Pd::new_from_pair(1.0, 1.0))
                );
                assert_eq!(
                    Pd::to_arr(linear_panning_factor(Pd::new(0.75))),
                    Pd::to_arr(Pd::new_from_pair(0.5, 1.0))
                );
                assert_eq!(
                    Pd::to_arr(linear_panning_factor(Pd::new(1.0))),
                    Pd::to_arr(Pd::new_from_pair(0.0, 1.0))
                );
            }
        }

        #[feature_gate]
        #[test_feature_gate]
        #[test]
        fn test_mono_mix_factor() {
            unsafe {
                assert_eq!(
                    Pd::to_arr(mono_mix_factor(Pd::new(0.0))),
                    Pd::to_arr(Pd::new_from_pair(1.0, 0.0))
                );
                assert_eq!(
                    Pd::to_arr(mono_mix_factor(Pd::new(0.25))),
                    Pd::to_arr(Pd::new_from_pair(0.5, 0.0))
                );
                assert_eq!(
                    Pd::to_arr(mono_mix_factor(Pd::new(0.5))),
                    Pd::to_arr(Pd::new_from_pair(0.0, 0.0))
                );
                assert_eq!(
                    Pd::to_arr(mono_mix_factor(Pd::new(0.75))),
                    Pd::to_arr(Pd::new_from_pair(0.0, 0.5))
                );
                assert_eq!(
                    Pd::to_arr(mono_mix_factor(Pd::new(1.0))),
                    Pd::to_arr(Pd::new_from_pair(0.0, 1.0))
                );
            }
        }
    }
}
