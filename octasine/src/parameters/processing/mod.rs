use array_init::array_init;

mod interpolatable_value;
pub mod parameters;

use super::values::*;
use crate::constants::*;
use parameters::*;

pub trait ProcessingParameter {
    type Value;
    type ExtraData;

    fn get_value(&mut self, extra_data: Self::ExtraData) -> Self::Value;
    fn set_from_sync(&mut self, value: f64);
    fn get_value_with_lfo_addition(
        &mut self,
        extra_data: Self::ExtraData,
        lfo_addition: Option<f64>,
    ) -> Self::Value;
}

pub struct ProcessingParameters {
    pub master_volume: InterpolatableProcessingParameter<MasterVolumeValue>,
    pub master_frequency: SimpleProcessingParameter<MasterFrequencyValue>,
    pub operators: [ProcessingParameterOperator; NUM_OPERATORS],
    pub lfos: [ProcessingParameterLfo; NUM_LFOS],
}

impl Default for ProcessingParameters {
    fn default() -> Self {
        Self {
            master_volume: Default::default(),
            master_frequency: Default::default(),
            operators: array_init(ProcessingParameterOperator::new),
            lfos: array_init(|lfo_index| ProcessingParameterLfo::new(lfo_index)),
        }
    }
}

#[allow(clippy::len_without_is_empty)]
impl ProcessingParameters {
    pub fn set_from_sync(&mut self, index: usize, value: f64) {
        match index {
            0 => self.master_volume.set_from_sync(value),
            1 => self.master_frequency.set_from_sync(value),
            2 => self.operators[0].volume.set_from_sync(value),
            3 => self.operators[0].panning.set_from_sync(value),
            4 => self.operators[0].wave_type.set_from_sync(value),
            5 => self.operators[0].modulation_index.set_from_sync(value),
            6 => self.operators[0].feedback.set_from_sync(value),
            7 => self.operators[0].frequency_ratio.set_from_sync(value),
            8 => self.operators[0].frequency_free.set_from_sync(value),
            9 => self.operators[0].frequency_fine.set_from_sync(value),
            10 => self.operators[0]
                .volume_envelope
                .attack_duration
                .set_from_sync(value),
            11 => self.operators[0]
                .volume_envelope
                .attack_end_value
                .set_from_sync(value),
            12 => self.operators[0]
                .volume_envelope
                .decay_duration
                .set_from_sync(value),
            13 => self.operators[0]
                .volume_envelope
                .decay_end_value
                .set_from_sync(value),
            14 => self.operators[0]
                .volume_envelope
                .release_duration
                .set_from_sync(value),
            15 => self.operators[1].volume.set_from_sync(value),
            16 => self.operators[1].panning.set_from_sync(value),
            17 => self.operators[1].wave_type.set_from_sync(value),
            18 => self.operators[1].additive_factor.set_from_sync(value),
            19 => self.operators[1].modulation_index.set_from_sync(value),
            20 => self.operators[1].feedback.set_from_sync(value),
            21 => self.operators[1].frequency_ratio.set_from_sync(value),
            22 => self.operators[1].frequency_free.set_from_sync(value),
            23 => self.operators[1].frequency_fine.set_from_sync(value),
            24 => self.operators[1]
                .volume_envelope
                .attack_duration
                .set_from_sync(value),
            25 => self.operators[1]
                .volume_envelope
                .attack_end_value
                .set_from_sync(value),
            26 => self.operators[1]
                .volume_envelope
                .decay_duration
                .set_from_sync(value),
            27 => self.operators[1]
                .volume_envelope
                .decay_end_value
                .set_from_sync(value),
            28 => self.operators[1]
                .volume_envelope
                .release_duration
                .set_from_sync(value),
            29 => self.operators[2].volume.set_from_sync(value),
            30 => self.operators[2].panning.set_from_sync(value),
            31 => self.operators[2].wave_type.set_from_sync(value),
            32 => self.operators[2].additive_factor.set_from_sync(value),
            33 => {
                use OperatorModulationTargetProcessingParameter::*;

                let opt_p = self.operators[2].output_operator.as_mut();

                if let Some(Three(p)) = opt_p {
                    p.set_from_sync(value)
                }
            }
            34 => self.operators[2].modulation_index.set_from_sync(value),
            35 => self.operators[2].feedback.set_from_sync(value),
            36 => self.operators[2].frequency_ratio.set_from_sync(value),
            37 => self.operators[2].frequency_free.set_from_sync(value),
            38 => self.operators[2].frequency_fine.set_from_sync(value),
            39 => self.operators[2]
                .volume_envelope
                .attack_duration
                .set_from_sync(value),
            40 => self.operators[2]
                .volume_envelope
                .attack_end_value
                .set_from_sync(value),
            41 => self.operators[2]
                .volume_envelope
                .decay_duration
                .set_from_sync(value),
            42 => self.operators[2]
                .volume_envelope
                .decay_end_value
                .set_from_sync(value),
            43 => self.operators[2]
                .volume_envelope
                .release_duration
                .set_from_sync(value),
            44 => self.operators[3].volume.set_from_sync(value),
            45 => self.operators[3].panning.set_from_sync(value),
            46 => self.operators[3].wave_type.set_from_sync(value),
            47 => self.operators[3].additive_factor.set_from_sync(value),
            48 => {
                use OperatorModulationTargetProcessingParameter::*;

                let opt_p = self.operators[3].output_operator.as_mut();

                if let Some(Four(p)) = opt_p {
                    p.set_from_sync(value)
                }
            }
            49 => self.operators[3].modulation_index.set_from_sync(value),
            50 => self.operators[3].feedback.set_from_sync(value),
            51 => self.operators[3].frequency_ratio.set_from_sync(value),
            52 => self.operators[3].frequency_free.set_from_sync(value),
            53 => self.operators[3].frequency_fine.set_from_sync(value),
            54 => self.operators[3]
                .volume_envelope
                .attack_duration
                .set_from_sync(value),
            55 => self.operators[3]
                .volume_envelope
                .attack_end_value
                .set_from_sync(value),
            56 => self.operators[3]
                .volume_envelope
                .decay_duration
                .set_from_sync(value),
            57 => self.operators[3]
                .volume_envelope
                .decay_end_value
                .set_from_sync(value),
            58 => self.operators[3]
                .volume_envelope
                .release_duration
                .set_from_sync(value),
            59 => self.lfos[0].target_parameter.set_from_sync(value),
            60 => self.lfos[0].bpm_sync.set_from_sync(value),
            61 => self.lfos[0].frequency_ratio.set_from_sync(value),
            62 => self.lfos[0].frequency_free.set_from_sync(value),
            63 => self.lfos[0].mode.set_from_sync(value),
            64 => self.lfos[0].shape.set_from_sync(value),
            65 => self.lfos[0].amount.set_from_sync(value),
            66 => self.lfos[1].target_parameter.set_from_sync(value),
            67 => self.lfos[1].bpm_sync.set_from_sync(value),
            68 => self.lfos[1].frequency_ratio.set_from_sync(value),
            69 => self.lfos[1].frequency_free.set_from_sync(value),
            70 => self.lfos[1].mode.set_from_sync(value),
            71 => self.lfos[1].shape.set_from_sync(value),
            72 => self.lfos[1].amount.set_from_sync(value),
            73 => self.lfos[2].target_parameter.set_from_sync(value),
            74 => self.lfos[2].bpm_sync.set_from_sync(value),
            75 => self.lfos[2].frequency_ratio.set_from_sync(value),
            76 => self.lfos[2].frequency_free.set_from_sync(value),
            77 => self.lfos[2].mode.set_from_sync(value),
            78 => self.lfos[2].shape.set_from_sync(value),
            79 => self.lfos[2].amount.set_from_sync(value),
            80 => self.lfos[3].target_parameter.set_from_sync(value),
            81 => self.lfos[3].bpm_sync.set_from_sync(value),
            82 => self.lfos[3].frequency_ratio.set_from_sync(value),
            83 => self.lfos[3].frequency_free.set_from_sync(value),
            84 => self.lfos[3].mode.set_from_sync(value),
            85 => self.lfos[3].shape.set_from_sync(value),
            86 => self.lfos[3].amount.set_from_sync(value),
            _ => (),
        }
    }

    pub fn len(&self) -> usize {
        87
    }
}

pub struct ProcessingParameterOperator {
    pub volume: OperatorVolumeProcessingParameter,
    pub wave_type: SimpleProcessingParameter<OperatorWaveTypeValue>,
    pub panning: OperatorPanningProcessingParameter,
    pub additive_factor: InterpolatableProcessingParameter<OperatorAdditiveValue>,
    pub output_operator: Option<OperatorModulationTargetProcessingParameter>,
    pub frequency_ratio: SimpleProcessingParameter<OperatorFrequencyRatioValue>,
    pub frequency_free: SimpleProcessingParameter<OperatorFrequencyFreeValue>,
    pub frequency_fine: SimpleProcessingParameter<OperatorFrequencyFineValue>,
    pub feedback: InterpolatableProcessingParameter<OperatorFeedbackValue>,
    pub modulation_index: InterpolatableProcessingParameter<OperatorModulationIndexValue>,
    pub volume_envelope: OperatorEnvelopeProcessingParameter,
}

impl ProcessingParameterOperator {
    pub fn new(operator_index: usize) -> Self {
        Self {
            volume: OperatorVolumeProcessingParameter::new(operator_index),
            wave_type: Default::default(),
            panning: OperatorPanningProcessingParameter::default(),
            additive_factor: Default::default(),
            output_operator: OperatorModulationTargetProcessingParameter::opt_new(operator_index),
            frequency_ratio: Default::default(),
            frequency_free: Default::default(),
            frequency_fine: Default::default(),
            feedback: Default::default(),
            modulation_index: Default::default(),
            volume_envelope: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct OperatorEnvelopeProcessingParameter {
    pub attack_duration: SimpleProcessingParameter<OperatorAttackDurationValue>,
    pub attack_end_value: SimpleProcessingParameter<OperatorAttackVolumeValue>,
    pub decay_duration: SimpleProcessingParameter<OperatorDecayDurationValue>,
    pub decay_end_value: SimpleProcessingParameter<OperatorDecayVolumeValue>,
    pub release_duration: SimpleProcessingParameter<OperatorReleaseDurationValue>,
}

pub struct ProcessingParameterLfo {
    pub target_parameter: LfoTargetProcessingParameter,
    pub bpm_sync: SimpleProcessingParameter<LfoBpmSyncValue>,
    pub frequency_ratio: SimpleProcessingParameter<LfoFrequencyRatioValue>,
    pub frequency_free: SimpleProcessingParameter<LfoFrequencyFreeValue>,
    pub mode: SimpleProcessingParameter<LfoModeValue>,
    pub shape: SimpleProcessingParameter<LfoShapeValue>,
    pub amount: InterpolatableProcessingParameter<LfoAmountValue>,
}

impl ProcessingParameterLfo {
    fn new(lfo_index: usize) -> Self {
        Self {
            target_parameter: LfoTargetProcessingParameter::new(lfo_index),
            bpm_sync: Default::default(),
            frequency_ratio: Default::default(),
            frequency_free: Default::default(),
            mode: Default::default(),
            shape: Default::default(),
            amount: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_operator_panning_left_and_right() {
        use super::interpolatable_value::*;
        use super::*;
        use crate::common::*;

        let mut operator = OperatorPanningProcessingParameter::default();

        let mut time = TimeCounter(0.0);
        let mut value = operator.get_value(time);

        operator.set_from_sync(1.0);

        let n = INTERPOLATION_SAMPLES_PER_STEP * INTERPOLATION_STEPS + 1;
        let mut left_and_right = [0.0, 0.0];

        for i in 0..n {
            let new_value = operator.get_value(time);
            let new_left_and_right = operator.left_and_right;

            #[allow(clippy::float_cmp)]
            if i >= INTERPOLATION_SAMPLES_PER_STEP && i % INTERPOLATION_SAMPLES_PER_STEP == 0 {
                assert_ne!(value, new_value);
                assert_ne!(left_and_right, new_left_and_right);
            }

            value = new_value;
            left_and_right = new_left_and_right;

            time.0 += 1.0 / 44100.0;
        }

        assert_approx_eq!(left_and_right[0], 0.0);
        assert_approx_eq!(left_and_right[1], 1.0);
    }
}
