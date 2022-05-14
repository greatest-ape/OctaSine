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
mod operator_volume;

use array_init::array_init;

use crate::common::{SampleRate, NUM_LFOS, NUM_OPERATORS};
use crate::parameter_values::*;

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
use self::operator_volume::OperatorVolumeAudioParameter;

pub struct AudioParameters {
    pub master_volume: MasterVolumeAudioParameter,
    pub master_frequency: MasterFrequencyAudioParameter,
    pub operators: [AudioParameterOperator; NUM_OPERATORS],
    pub lfos: [AudioParameterLfo; NUM_LFOS],
}

impl Default for AudioParameters {
    fn default() -> Self {
        Self {
            master_volume: Default::default(),
            master_frequency: Default::default(),
            operators: array_init(AudioParameterOperator::new),
            lfos: array_init(AudioParameterLfo::new),
        }
    }
}

impl AudioParameters {
    pub fn set_from_patch(&mut self, parameter: Parameter, value: f64) {
        match parameter {
            Parameter::Master(MasterParameter::Volume) => self.master_volume.set_from_patch(value),
            Parameter::Master(MasterParameter::Frequency) => {
                self.master_volume.set_from_patch(value)
            }
            Parameter::Operator(index, p) => {
                let operator = &mut self.operators[index];

                match p {
                    OperatorParameter::Volume => operator.volume.set_from_patch(value),
                    OperatorParameter::Active => operator.active.set_from_patch(value),
                    OperatorParameter::MixOut => operator.mix.set_from_patch(value),
                    OperatorParameter::Panning => operator.panning.set_from_patch(value),
                    OperatorParameter::WaveType => operator.wave_type.set_from_patch(value),
                    OperatorParameter::ModTargets => {
                        use OperatorModulationTargetAudioParameter::{Four, Three, Two};

                        match operator.output_operator {
                            Some(Two(p)) => p.set_from_patch(value),
                            Some(Three(p)) => p.set_from_patch(value),
                            Some(Four(p)) => p.set_from_patch(value),
                            None => (),
                        }
                    }
                    OperatorParameter::ModOut => match operator.modulation_index {
                        Some(p) => p.set_from_patch(value),
                        None => (),
                    },
                    OperatorParameter::Feedback => operator.feedback.set_from_patch(value),
                    OperatorParameter::FrequencyRatio => {
                        operator.frequency_ratio.set_from_patch(value)
                    }
                    OperatorParameter::FrequencyFree => {
                        operator.frequency_free.set_from_patch(value)
                    }
                    OperatorParameter::FrequencyFine => {
                        operator.frequency_fine.set_from_patch(value)
                    }
                    OperatorParameter::AttackDuration => operator
                        .volume_envelope
                        .attack_duration
                        .set_from_patch(value),
                    OperatorParameter::AttackValue => operator
                        .volume_envelope
                        .attack_end_value
                        .set_from_patch(value),
                    OperatorParameter::DecayDuration => operator
                        .volume_envelope
                        .decay_duration
                        .set_from_patch(value),
                    OperatorParameter::DecayValue => operator
                        .volume_envelope
                        .decay_end_value
                        .set_from_patch(value),
                    OperatorParameter::ReleaseDuration => operator
                        .volume_envelope
                        .release_duration
                        .set_from_patch(value),
                }
            }
            Parameter::Lfo(index, p) => {
                let lfo = &mut self.lfos[index];

                match p {
                    LfoParameter::Target => lfo.target_parameter.set_from_sync(value),
                    LfoParameter::BpmSync => lfo.bpm_sync.set_from_patch(value),
                    LfoParameter::FrequencyRatio => lfo.frequency_ratio.set_from_patch(value),
                    LfoParameter::FrequencyFree => lfo.frequency_free.set_from_patch(value),
                    LfoParameter::Mode => lfo.mode.set_from_patch(value),
                    LfoParameter::Shape => lfo.shape.set_from_patch(value),
                    LfoParameter::Amount => lfo.amount.set_from_patch(value),
                    LfoParameter::Active => lfo.active.set_from_patch(value),
                }
            }
        }
    }

    pub fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.master_volume.advance_one_sample(sample_rate);
        self.master_frequency.advance_one_sample(sample_rate);

        for operator in self.operators.iter_mut() {
            operator.advance_one_sample(sample_rate);
        }

        for lfo in self.lfos.iter_mut() {
            lfo.advance_one_sample(sample_rate);
        }
    }
}

pub struct AudioParameterOperator {
    pub volume: OperatorVolumeAudioParameter,
    pub active: InterpolatableAudioParameter<OperatorActiveValue>,
    pub mix: OperatorMixAudioParameter,
    pub wave_type: SimpleAudioParameter<OperatorWaveTypeValue>,
    pub panning: OperatorPanningAudioParameter,
    pub output_operator: Option<OperatorModulationTargetAudioParameter>,
    pub frequency_ratio: SimpleAudioParameter<OperatorFrequencyRatioValue>,
    pub frequency_free: OperatorFrequencyFreeAudioParameter,
    pub frequency_fine: OperatorFrequencyFineAudioParameter,
    pub feedback: InterpolatableAudioParameter<OperatorFeedbackValue>,
    pub modulation_index: Option<InterpolatableAudioParameter<OperatorModOutValue>>,
    pub volume_envelope: OperatorEnvelopeAudioParameter,
}

impl AudioParameterOperator {
    pub fn new(operator_index: usize) -> Self {
        let modulation_index = if operator_index == 0 {
            None
        } else {
            Some(Default::default())
        };

        Self {
            volume: Default::default(),
            active: Default::default(),
            mix: OperatorMixAudioParameter::new(operator_index),
            wave_type: Default::default(),
            panning: OperatorPanningAudioParameter::default(),
            output_operator: OperatorModulationTargetAudioParameter::opt_new(operator_index),
            frequency_ratio: Default::default(),
            frequency_free: Default::default(),
            frequency_fine: Default::default(),
            feedback: Default::default(),
            modulation_index,
            volume_envelope: Default::default(),
        }
    }

    pub fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.volume.advance_one_sample(sample_rate);
        self.active.advance_one_sample(sample_rate);
        self.mix.advance_one_sample(sample_rate);
        self.wave_type.advance_one_sample(sample_rate);
        self.panning.advance_one_sample(sample_rate);
        if let Some(ref mut output_operator) = self.output_operator {
            output_operator.advance_one_sample(sample_rate);
        }
        self.frequency_ratio.advance_one_sample(sample_rate);
        self.frequency_free.advance_one_sample(sample_rate);
        self.frequency_fine.advance_one_sample(sample_rate);
        self.feedback.advance_one_sample(sample_rate);
        if let Some(modulation_index) = self.modulation_index.as_mut() {
            modulation_index.advance_one_sample(sample_rate);
        }
        self.volume_envelope.advance_one_sample(sample_rate);
    }
}

#[derive(Default)]
pub struct OperatorEnvelopeAudioParameter {
    pub attack_duration: SimpleAudioParameter<OperatorAttackDurationValue>,
    pub attack_end_value: SimpleAudioParameter<OperatorAttackVolumeValue>,
    pub decay_duration: SimpleAudioParameter<OperatorDecayDurationValue>,
    pub decay_end_value: SimpleAudioParameter<OperatorDecayVolumeValue>,
    pub release_duration: SimpleAudioParameter<OperatorReleaseDurationValue>,
}

impl OperatorEnvelopeAudioParameter {
    fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.attack_duration.advance_one_sample(sample_rate);
        self.attack_end_value.advance_one_sample(sample_rate);
        self.decay_duration.advance_one_sample(sample_rate);
        self.decay_end_value.advance_one_sample(sample_rate);
        self.release_duration.advance_one_sample(sample_rate);
    }
}

pub struct AudioParameterLfo {
    pub target_parameter: LfoTargetAudioParameter,
    pub bpm_sync: SimpleAudioParameter<LfoBpmSyncValue>,
    pub frequency_ratio: SimpleAudioParameter<LfoFrequencyRatioValue>,
    pub frequency_free: LfoFrequencyFreeAudioParameter,
    pub mode: SimpleAudioParameter<LfoModeValue>,
    pub shape: SimpleAudioParameter<LfoShapeValue>,
    pub amount: LfoAmountAudioParameter,
    pub active: LfoActiveAudioParameter,
}

impl AudioParameterLfo {
    fn new(lfo_index: usize) -> Self {
        Self {
            target_parameter: LfoTargetAudioParameter::new(lfo_index),
            bpm_sync: Default::default(),
            frequency_ratio: Default::default(),
            frequency_free: Default::default(),
            mode: Default::default(),
            shape: Default::default(),
            amount: Default::default(),
            active: Default::default(),
        }
    }

    fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.target_parameter.advance_one_sample(sample_rate);
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

    use crate::audio::common::InterpolationDuration;

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
