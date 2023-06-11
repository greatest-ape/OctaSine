pub mod common;
mod lfo_active;
mod lfo_amount;
mod lfo_frequency_free;
mod lfo_target;
mod master_frequency;
mod master_volume;
mod operator_active;
mod operator_frequency_fine;
mod operator_frequency_free;
mod operator_mix;
mod operator_mod_target;
mod operator_panning;
mod operator_sustain_volume;
mod operator_volume;

use array_init::array_init;

use crate::common::{SampleRate, NUM_LFOS, NUM_OPERATORS};
use crate::parameters::glide_active::GlideActiveValue;
use crate::parameters::glide_bpm_sync::GlideBpmSyncValue;
use crate::parameters::glide_mode::GlideModeValue;
use crate::parameters::glide_retrigger::GlideRetriggerValue;
use crate::parameters::glide_time::GlideTimeValue;
use crate::parameters::master_pitch_bend_range::{
    MasterPitchBendRangeDownValue, MasterPitchBendRangeUpValue,
};
use crate::parameters::velocity_sensitivity::VelocitySensitivityValue;
use crate::parameters::voice_mode::VoiceModeValue;
use crate::parameters::*;

use self::common::{AudioParameter, InterpolatableAudioParameter, SimpleAudioParameter};
use self::lfo_active::LfoActiveAudioParameter;
use self::lfo_amount::LfoAmountAudioParameter;
use self::lfo_frequency_free::LfoFrequencyFreeAudioParameter;
use self::lfo_target::LfoTargetAudioParameter;
use self::master_frequency::MasterFrequencyAudioParameter;
use self::master_volume::MasterVolumeAudioParameter;
use self::operator_frequency_fine::OperatorFrequencyFineAudioParameter;
use self::operator_frequency_free::OperatorFrequencyFreeAudioParameter;
use self::operator_mix::OperatorMixAudioParameter;
use self::operator_mod_target::OperatorModulationTargetAudioParameter;
use self::operator_panning::OperatorPanningAudioParameter;
use self::operator_sustain_volume::OperatorSustainVolumeAudioParameter;
use self::operator_volume::OperatorVolumeAudioParameter;

trait AudioParameterPatchInteraction {
    fn set_patch_value(&mut self, value: f32);
    #[cfg(test)]
    fn compare_patch_value(&mut self, value: f32) -> bool;
}

impl<P: AudioParameter> AudioParameterPatchInteraction for P {
    fn set_patch_value(&mut self, value: f32) {
        self.set_from_patch(value)
    }
    #[cfg(test)]
    fn compare_patch_value(&mut self, value: f32) -> bool {
        let a = P::ParameterValue::new_from_patch(value).to_patch();
        let b = self.get_parameter_value().to_patch();

        (a - b).abs() <= 1.0 / 100_000.0
    }
}

pub struct AudioParameters {
    pub master_volume: MasterVolumeAudioParameter,
    pub master_frequency: MasterFrequencyAudioParameter,
    pub master_pitch_bend_range_up: SimpleAudioParameter<MasterPitchBendRangeUpValue>,
    pub master_pitch_bend_range_down: SimpleAudioParameter<MasterPitchBendRangeDownValue>,
    pub volume_velocity_sensitivity: InterpolatableAudioParameter<VelocitySensitivityValue>,
    pub voice_mode: SimpleAudioParameter<VoiceModeValue>,
    pub glide_active: SimpleAudioParameter<GlideActiveValue>,
    pub glide_time: SimpleAudioParameter<GlideTimeValue>,
    pub glide_bpm_sync: SimpleAudioParameter<GlideBpmSyncValue>,
    pub glide_mode: SimpleAudioParameter<GlideModeValue>,
    pub glide_retrigger: SimpleAudioParameter<GlideRetriggerValue>,
    pub operators: [OperatorAudioParameters; NUM_OPERATORS],
    pub lfos: [LfoAudioParameters; NUM_LFOS],
}

impl Default for AudioParameters {
    fn default() -> Self {
        Self {
            master_volume: Default::default(),
            master_frequency: Default::default(),
            master_pitch_bend_range_up: Default::default(),
            master_pitch_bend_range_down: Default::default(),
            volume_velocity_sensitivity: Default::default(),
            voice_mode: Default::default(),
            glide_active: Default::default(),
            glide_time: Default::default(),
            glide_bpm_sync: Default::default(),
            glide_mode: Default::default(),
            glide_retrigger: Default::default(),
            operators: array_init(OperatorAudioParameters::new),
            lfos: array_init(LfoAudioParameters::new),
        }
    }
}

macro_rules! impl_patch_interaction {
    ($name:ident, $input:ty, $output:ty, $f:expr) => {
        pub fn $name(&mut self, parameter: Parameter, input: $input) -> Option<$output> {
            match parameter {
                Parameter::None => None,
                Parameter::Master(p) => match p {
                    MasterParameter::Volume => $f(&mut self.master_volume, input),
                    MasterParameter::Frequency => $f(&mut self.master_frequency, input),
                    MasterParameter::PitchBendRangeUp => {
                        $f(&mut self.master_pitch_bend_range_up, input)
                    }
                    MasterParameter::PitchBendRangeDown => {
                        $f(&mut self.master_pitch_bend_range_down, input)
                    }
                    MasterParameter::VelocitySensitivityVolume => {
                        $f(&mut self.volume_velocity_sensitivity, input)
                    }
                    MasterParameter::VoiceMode => $f(&mut self.voice_mode, input),
                    MasterParameter::GlideActive => $f(&mut self.glide_active, input),
                    MasterParameter::GlideTime => $f(&mut self.glide_time, input),
                    MasterParameter::GlideBpmSync => $f(&mut self.glide_bpm_sync, input),
                    MasterParameter::GlideMode => $f(&mut self.glide_mode, input),
                    MasterParameter::GlideRetrigger => $f(&mut self.glide_retrigger, input),
                },
                Parameter::Operator(index, p) => {
                    use OperatorParameter::*;

                    let operator = &mut self.operators[index as usize];

                    match p {
                        Volume => $f(&mut operator.volume, input),
                        Active => $f(&mut operator.active, input),
                        MixOut => $f(&mut operator.mix_out, input),
                        Panning => $f(&mut operator.panning, input),
                        WaveType => $f(&mut operator.wave_type, input),
                        ModTargets => {
                            if let Some(p) = &mut operator.mod_targets {
                                $f(p, input)
                            } else {
                                None
                            }
                        }
                        ModOut => {
                            if let Some(p) = operator.mod_out.as_mut() {
                                $f(p, input)
                            } else {
                                None
                            }
                        }
                        Feedback => $f(&mut operator.feedback, input),
                        FrequencyRatio => $f(&mut operator.frequency_ratio, input),
                        FrequencyFree => $f(&mut operator.frequency_free, input),
                        FrequencyFine => $f(&mut operator.frequency_fine, input),
                        AttackDuration => $f(&mut operator.volume_envelope.attack_duration, input),
                        DecayDuration => $f(&mut operator.volume_envelope.decay_duration, input),
                        SustainVolume => $f(&mut operator.volume_envelope.sustain_volume, input),
                        ReleaseDuration => {
                            $f(&mut operator.volume_envelope.release_duration, input)
                        }
                        EnvelopeLockGroup => $f(&mut operator.volume_envelope.lock_group, input),
                        VelocitySensitivityModOut => {
                            $f(&mut operator.velocity_sensitivity_mod_out, input)
                        }
                        VelocitySensitivityFeedback => {
                            $f(&mut operator.velocity_sensitivity_feedback, input)
                        }
                    }
                }
                Parameter::Lfo(index, p) => {
                    let lfo = &mut self.lfos[index as usize];

                    match p {
                        LfoParameter::Target => $f(&mut lfo.target, input),
                        LfoParameter::BpmSync => $f(&mut lfo.bpm_sync, input),
                        LfoParameter::FrequencyRatio => $f(&mut lfo.frequency_ratio, input),
                        LfoParameter::FrequencyFree => $f(&mut lfo.frequency_free, input),
                        LfoParameter::Mode => $f(&mut lfo.mode, input),
                        LfoParameter::Shape => $f(&mut lfo.shape, input),
                        LfoParameter::Amount => $f(&mut lfo.amount, input),
                        LfoParameter::Active => $f(&mut lfo.active, input),
                        LfoParameter::KeySync => $f(&mut lfo.key_sync, input),
                    }
                }
            }
        }
    };
}

impl AudioParameters {
    impl_patch_interaction!(
        set_parameter_from_patch,
        f32,
        (),
        |p: &mut dyn AudioParameterPatchInteraction, v| {
            p.set_patch_value(v);
            Some(())
        }
    );

    #[cfg(test)]
    impl_patch_interaction!(
        compare_patch_value,
        f32,
        bool,
        |p: &mut dyn AudioParameterPatchInteraction, v| Some(p.compare_patch_value(v))
    );

    pub fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.master_volume.advance_one_sample(sample_rate);
        self.master_frequency.advance_one_sample(sample_rate);
        self.volume_velocity_sensitivity
            .advance_one_sample(sample_rate);

        for operator in self.operators.iter_mut() {
            operator.advance_one_sample(sample_rate);
        }

        for lfo in self.lfos.iter_mut() {
            lfo.advance_one_sample(sample_rate);
        }
    }
}

pub struct OperatorAudioParameters {
    pub active: InterpolatableAudioParameter<OperatorActiveValue>,
    pub wave_type: SimpleAudioParameter<OperatorWaveTypeValue>,
    pub volume: OperatorVolumeAudioParameter,
    pub panning: OperatorPanningAudioParameter,
    pub mix_out: OperatorMixAudioParameter,
    pub mod_out: Option<InterpolatableAudioParameter<OperatorModOutValue>>,
    pub mod_targets: Option<OperatorModulationTargetAudioParameter>,
    pub feedback: InterpolatableAudioParameter<OperatorFeedbackValue>,
    pub frequency_ratio: SimpleAudioParameter<OperatorFrequencyRatioValue>,
    pub frequency_free: OperatorFrequencyFreeAudioParameter,
    pub frequency_fine: OperatorFrequencyFineAudioParameter,
    pub volume_envelope: OperatorEnvelopeAudioParameters,
    pub velocity_sensitivity_mod_out: InterpolatableAudioParameter<VelocitySensitivityValue>,
    pub velocity_sensitivity_feedback: InterpolatableAudioParameter<VelocitySensitivityValue>,
}

impl OperatorAudioParameters {
    pub fn new(operator_index: usize) -> Self {
        let modulation_index = if operator_index == 0 {
            None
        } else {
            Some(Default::default())
        };

        Self {
            active: Default::default(),
            wave_type: Default::default(),
            volume: Default::default(),
            panning: OperatorPanningAudioParameter::default(),
            mix_out: OperatorMixAudioParameter::new(operator_index),
            mod_out: modulation_index,
            mod_targets: OperatorModulationTargetAudioParameter::opt_new(operator_index),
            feedback: Default::default(),
            frequency_ratio: Default::default(),
            frequency_free: Default::default(),
            frequency_fine: Default::default(),
            volume_envelope: Default::default(),
            velocity_sensitivity_mod_out: Default::default(),
            velocity_sensitivity_feedback: Default::default(),
        }
    }

    pub fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.active.advance_one_sample(sample_rate);
        self.volume.advance_one_sample(sample_rate);
        self.wave_type.advance_one_sample(sample_rate);
        self.panning.advance_one_sample(sample_rate);
        if let Some(mod_targets) = &mut self.mod_targets {
            mod_targets.advance_one_sample(sample_rate);
        }
        self.mix_out.advance_one_sample(sample_rate);
        if let Some(mod_out) = self.mod_out.as_mut() {
            mod_out.advance_one_sample(sample_rate);
        }
        self.feedback.advance_one_sample(sample_rate);
        self.frequency_ratio.advance_one_sample(sample_rate);
        self.frequency_free.advance_one_sample(sample_rate);
        self.frequency_fine.advance_one_sample(sample_rate);
        self.volume_envelope.advance_one_sample(sample_rate);
        self.velocity_sensitivity_mod_out
            .advance_one_sample(sample_rate);
        self.velocity_sensitivity_feedback
            .advance_one_sample(sample_rate);
    }
}

#[derive(Default)]
pub struct OperatorEnvelopeAudioParameters {
    pub attack_duration: SimpleAudioParameter<OperatorAttackDurationValue>,
    pub decay_duration: SimpleAudioParameter<OperatorDecayDurationValue>,
    pub sustain_volume: OperatorSustainVolumeAudioParameter,
    pub release_duration: SimpleAudioParameter<OperatorReleaseDurationValue>,
    pub lock_group: SimpleAudioParameter<OperatorEnvelopeGroupValue>,
}

impl OperatorEnvelopeAudioParameters {
    fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.attack_duration.advance_one_sample(sample_rate);
        self.decay_duration.advance_one_sample(sample_rate);
        self.sustain_volume.advance_one_sample(sample_rate);
        self.release_duration.advance_one_sample(sample_rate);
        self.lock_group.advance_one_sample(sample_rate);
    }
}

pub struct LfoAudioParameters {
    pub target: LfoTargetAudioParameter,
    pub bpm_sync: SimpleAudioParameter<LfoBpmSyncValue>,
    pub frequency_ratio: SimpleAudioParameter<LfoFrequencyRatioValue>,
    pub frequency_free: LfoFrequencyFreeAudioParameter,
    pub mode: SimpleAudioParameter<LfoModeValue>,
    pub shape: SimpleAudioParameter<LfoShapeValue>,
    pub amount: LfoAmountAudioParameter,
    pub active: LfoActiveAudioParameter,
    pub key_sync: SimpleAudioParameter<LfoKeySyncValue>,
}

impl LfoAudioParameters {
    fn new(lfo_index: usize) -> Self {
        Self {
            target: LfoTargetAudioParameter::new(lfo_index),
            bpm_sync: Default::default(),
            frequency_ratio: Default::default(),
            frequency_free: Default::default(),
            mode: Default::default(),
            shape: Default::default(),
            amount: Default::default(),
            active: Default::default(),
            key_sync: Default::default(),
        }
    }

    fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.target.advance_one_sample(sample_rate);
        self.bpm_sync.advance_one_sample(sample_rate);
        self.frequency_ratio.advance_one_sample(sample_rate);
        self.frequency_free.advance_one_sample(sample_rate);
        self.mode.advance_one_sample(sample_rate);
        self.shape.advance_one_sample(sample_rate);
        self.amount.advance_one_sample(sample_rate);
        self.active.advance_one_sample(sample_rate);
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::audio::interpolation::InterpolationDuration;

    use super::*;

    #[test]
    fn test_operator_panning_left_and_right() {
        let sample_rate = SampleRate::default();
        let num_steps = InterpolationDuration::approx_1ms().samples(sample_rate);

        let mut audio_parameter = OperatorPanningAudioParameter::default();

        let mut value = audio_parameter.get_value();
        let mut left_and_right = [0.0, 0.0];

        audio_parameter.set_from_patch(1.0);

        for _ in 0..num_steps {
            audio_parameter.advance_one_sample(sample_rate);

            let new_value = audio_parameter.get_value();
            let new_left_and_right = audio_parameter.left_and_right;

            assert_ne!(value, new_value);
            assert_ne!(left_and_right, new_left_and_right);

            value = new_value;
            left_and_right = new_left_and_right;
        }

        assert_approx_eq!(value, 1.0);

        assert_approx_eq!(left_and_right[0], 0.0);
        assert_approx_eq!(left_and_right[1], 1.0);
    }
}
