use array_init::array_init;

mod interpolatable_value;
pub mod parameters;

use crate::common::{NUM_LFOS, NUM_OPERATORS};

use crate::parameters::values::*;

use parameters::*;

pub trait AudioParameter {
    type Value;

    fn advance_one_sample(&mut self);
    fn get_value(&self) -> Self::Value;
    fn set_from_sync(&mut self, value: f64);
    fn get_value_with_lfo_addition(&mut self, lfo_addition: Option<f64>) -> Self::Value;
}

pub struct AudioParameters {
    pub master_volume: MasterVolumeAudioParameter,
    pub master_frequency: FreeFrequencyAudioParameter<MasterFrequencyValue>,
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

#[allow(clippy::len_without_is_empty)]
impl AudioParameters {
    pub fn set_from_sync(&mut self, index: usize, value: f64) {
        match index {
            // Master parameters
            0 => self.master_volume.set_from_sync(value),
            1 => self.master_frequency.set_from_sync(value),
            // Operator 1
            2 => self.operators[0].volume.set_from_sync(value),
            3 => self.operators[0].active.set_from_sync(value),
            4 => self.operators[0].mix.set_from_sync(value),
            5 => self.operators[0].panning.set_from_sync(value),
            6 => self.operators[0].wave_type.set_from_sync(value),
            7 => self.operators[0].feedback.set_from_sync(value),
            8 => self.operators[0].frequency_ratio.set_from_sync(value),
            9 => self.operators[0].frequency_free.set_from_sync(value),
            10 => self.operators[0].frequency_fine.set_from_sync(value),
            11 => self.operators[0]
                .volume_envelope
                .attack_duration
                .set_from_sync(value),
            12 => self.operators[0]
                .volume_envelope
                .attack_end_value
                .set_from_sync(value),
            13 => self.operators[0]
                .volume_envelope
                .decay_duration
                .set_from_sync(value),
            14 => self.operators[0]
                .volume_envelope
                .decay_end_value
                .set_from_sync(value),
            15 => self.operators[0]
                .volume_envelope
                .release_duration
                .set_from_sync(value),
            // Operator 2
            16 => self.operators[1].volume.set_from_sync(value),
            17 => self.operators[1].active.set_from_sync(value),
            18 => self.operators[1].mix.set_from_sync(value),
            19 => self.operators[1].panning.set_from_sync(value),
            20 => self.operators[1].wave_type.set_from_sync(value),
            21 => {
                use OperatorModulationTargetAudioParameter::*;

                let opt_p = self.operators[1].output_operator.as_mut();

                if let Some(Two(p)) = opt_p {
                    p.set_from_sync(value)
                }
            }
            22 => {
                if let Some(p) = self.operators[1].modulation_index.as_mut() {
                    p.set_from_sync(value)
                }
            }
            23 => self.operators[1].feedback.set_from_sync(value),
            24 => self.operators[1].frequency_ratio.set_from_sync(value),
            25 => self.operators[1].frequency_free.set_from_sync(value),
            26 => self.operators[1].frequency_fine.set_from_sync(value),
            27 => self.operators[1]
                .volume_envelope
                .attack_duration
                .set_from_sync(value),
            28 => self.operators[1]
                .volume_envelope
                .attack_end_value
                .set_from_sync(value),
            29 => self.operators[1]
                .volume_envelope
                .decay_duration
                .set_from_sync(value),
            30 => self.operators[1]
                .volume_envelope
                .decay_end_value
                .set_from_sync(value),
            31 => self.operators[1]
                .volume_envelope
                .release_duration
                .set_from_sync(value),
            // Operator 3
            32 => self.operators[2].volume.set_from_sync(value),
            33 => self.operators[2].active.set_from_sync(value),
            34 => self.operators[2].mix.set_from_sync(value),
            35 => self.operators[2].panning.set_from_sync(value),
            36 => self.operators[2].wave_type.set_from_sync(value),
            37 => {
                use OperatorModulationTargetAudioParameter::*;

                let opt_p = self.operators[2].output_operator.as_mut();

                if let Some(Three(p)) = opt_p {
                    p.set_from_sync(value)
                }
            }
            38 => {
                if let Some(p) = self.operators[2].modulation_index.as_mut() {
                    p.set_from_sync(value)
                }
            }
            39 => self.operators[2].feedback.set_from_sync(value),
            40 => self.operators[2].frequency_ratio.set_from_sync(value),
            41 => self.operators[2].frequency_free.set_from_sync(value),
            42 => self.operators[2].frequency_fine.set_from_sync(value),
            43 => self.operators[2]
                .volume_envelope
                .attack_duration
                .set_from_sync(value),
            44 => self.operators[2]
                .volume_envelope
                .attack_end_value
                .set_from_sync(value),
            45 => self.operators[2]
                .volume_envelope
                .decay_duration
                .set_from_sync(value),
            46 => self.operators[2]
                .volume_envelope
                .decay_end_value
                .set_from_sync(value),
            47 => self.operators[2]
                .volume_envelope
                .release_duration
                .set_from_sync(value),
            // Operator 4
            48 => self.operators[3].volume.set_from_sync(value),
            49 => self.operators[3].active.set_from_sync(value),
            50 => self.operators[3].mix.set_from_sync(value),
            51 => self.operators[3].panning.set_from_sync(value),
            52 => self.operators[3].wave_type.set_from_sync(value),
            53 => {
                use OperatorModulationTargetAudioParameter::*;

                let opt_p = self.operators[3].output_operator.as_mut();

                if let Some(Four(p)) = opt_p {
                    p.set_from_sync(value)
                }
            }
            54 => {
                if let Some(p) = self.operators[3].modulation_index.as_mut() {
                    p.set_from_sync(value)
                }
            }
            55 => self.operators[3].feedback.set_from_sync(value),
            56 => self.operators[3].frequency_ratio.set_from_sync(value),
            57 => self.operators[3].frequency_free.set_from_sync(value),
            58 => self.operators[3].frequency_fine.set_from_sync(value),
            59 => self.operators[3]
                .volume_envelope
                .attack_duration
                .set_from_sync(value),
            60 => self.operators[3]
                .volume_envelope
                .attack_end_value
                .set_from_sync(value),
            61 => self.operators[3]
                .volume_envelope
                .decay_duration
                .set_from_sync(value),
            62 => self.operators[3]
                .volume_envelope
                .decay_end_value
                .set_from_sync(value),
            63 => self.operators[3]
                .volume_envelope
                .release_duration
                .set_from_sync(value),
            // LFOs
            64 => self.lfos[0].target_parameter.set_from_sync(value),
            65 => self.lfos[0].bpm_sync.set_from_sync(value),
            66 => self.lfos[0].frequency_ratio.set_from_sync(value),
            67 => self.lfos[0].frequency_free.set_from_sync(value),
            68 => self.lfos[0].mode.set_from_sync(value),
            69 => self.lfos[0].shape.set_from_sync(value),
            70 => self.lfos[0].amount.set_from_sync(value),
            71 => self.lfos[1].target_parameter.set_from_sync(value),
            72 => self.lfos[1].bpm_sync.set_from_sync(value),
            73 => self.lfos[1].frequency_ratio.set_from_sync(value),
            74 => self.lfos[1].frequency_free.set_from_sync(value),
            75 => self.lfos[1].mode.set_from_sync(value),
            76 => self.lfos[1].shape.set_from_sync(value),
            77 => self.lfos[1].amount.set_from_sync(value),
            78 => self.lfos[2].target_parameter.set_from_sync(value),
            79 => self.lfos[2].bpm_sync.set_from_sync(value),
            80 => self.lfos[2].frequency_ratio.set_from_sync(value),
            81 => self.lfos[2].frequency_free.set_from_sync(value),
            82 => self.lfos[2].mode.set_from_sync(value),
            83 => self.lfos[2].shape.set_from_sync(value),
            84 => self.lfos[2].amount.set_from_sync(value),
            85 => self.lfos[3].target_parameter.set_from_sync(value),
            86 => self.lfos[3].bpm_sync.set_from_sync(value),
            87 => self.lfos[3].frequency_ratio.set_from_sync(value),
            88 => self.lfos[3].frequency_free.set_from_sync(value),
            89 => self.lfos[3].mode.set_from_sync(value),
            90 => self.lfos[3].shape.set_from_sync(value),
            91 => self.lfos[3].amount.set_from_sync(value),
            _ => (),
        }
    }

    pub fn len(&self) -> usize {
        87
    }

    pub fn advance_one_sample(&mut self) {
        self.master_volume.advance_one_sample();
        self.master_frequency.advance_one_sample();

        for operator in self.operators.iter_mut() {
            operator.advance_one_sample();
        }

        for lfo in self.lfos.iter_mut() {
            lfo.advance_one_sample();
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
    pub frequency_free: FreeFrequencyAudioParameter<OperatorFrequencyFreeValue>,
    pub frequency_fine: SimpleAudioParameter<OperatorFrequencyFineValue>,
    pub feedback: InterpolatableAudioParameter<OperatorFeedbackValue>,
    pub modulation_index: Option<InterpolatableAudioParameter<OperatorModulationIndexValue>>,
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

    pub fn advance_one_sample(&mut self) {
        self.volume.advance_one_sample();
        self.active.advance_one_sample();
        self.mix.advance_one_sample();
        self.wave_type.advance_one_sample();
        self.panning.advance_one_sample();
        if let Some(ref mut output_operator) = self.output_operator {
            output_operator.advance_one_sample();
        }
        self.frequency_ratio.advance_one_sample();
        self.frequency_free.advance_one_sample();
        self.frequency_fine.advance_one_sample();
        self.feedback.advance_one_sample();
        if let Some(modulation_index) = self.modulation_index.as_mut() {
            modulation_index.advance_one_sample();
        }
        self.volume_envelope.advance_one_sample();
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
    fn advance_one_sample(&mut self) {
        self.attack_duration.advance_one_sample();
        self.attack_end_value.advance_one_sample();
        self.decay_duration.advance_one_sample();
        self.decay_end_value.advance_one_sample();
        self.release_duration.advance_one_sample();
    }
}

pub struct AudioParameterLfo {
    pub target_parameter: LfoTargetAudioParameter,
    pub bpm_sync: SimpleAudioParameter<LfoBpmSyncValue>,
    pub frequency_ratio: SimpleAudioParameter<LfoFrequencyRatioValue>,
    pub frequency_free: FreeFrequencyAudioParameter<LfoFrequencyFreeValue>,
    pub mode: SimpleAudioParameter<LfoModeValue>,
    pub shape: SimpleAudioParameter<LfoShapeValue>,
    pub amount: LfoAmountAudioParameter,
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
        }
    }

    fn advance_one_sample(&mut self) {
        self.target_parameter.advance_one_sample();
        self.bpm_sync.advance_one_sample();
        self.frequency_ratio.advance_one_sample();
        self.frequency_free.advance_one_sample();
        self.mode.advance_one_sample();
        self.shape.advance_one_sample();
        self.amount.advance_one_sample();
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_operator_panning_left_and_right() {
        use super::interpolatable_value::INTERPOLATION_STEPS;
        use super::*;

        let mut operator = OperatorPanningAudioParameter::default();

        let mut value = operator.get_value();

        operator.set_from_sync(1.0);

        let mut left_and_right = [0.0, 0.0];

        for _ in 0..INTERPOLATION_STEPS {
            operator.advance_one_sample();

            let new_value = operator.get_value();
            let new_left_and_right = operator.left_and_right;

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
