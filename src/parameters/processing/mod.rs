use array_init::array_init;

use crate::constants::*;

pub mod common;
pub mod parameters;

pub use common::*;
pub use parameters::*;


#[derive(Debug, Clone)]
pub struct ProcessingOperatorEnvelope {
    pub attack_duration: ProcessingOperatorAttackDuration,
    pub attack_end_value: ProcessingOperatorAttackVolume,
    pub decay_duration: ProcessingOperatorDecayDuration,
    pub decay_end_value: ProcessingOperatorDecayVolume,
    pub release_duration: ProcessingOperatorReleaseDuration,
}

impl ProcessingOperatorEnvelope {
    fn new() -> Self {
        Self {
            attack_duration: ProcessingOperatorAttackDuration::default(),
            attack_end_value: ProcessingOperatorAttackVolume::default(),
            decay_duration: ProcessingOperatorDecayDuration::default(),
            decay_end_value: ProcessingOperatorDecayVolume::default(),
            release_duration: ProcessingOperatorReleaseDuration::default(),
        }
    }
}


#[derive(Debug)]
pub struct ProcessingOperator {
    pub volume: ProcessingOperatorVolume,
    pub wave_type: ProcessingOperatorWaveType,
    pub panning: ProcessingOperatorPanning,
    pub additive_factor: ProcessingOperatorAdditiveFactor,
    pub output_operator: Option<ProcessingOperatorModulationTarget>,
    pub frequency_ratio: ProcessingOperatorFrequencyRatio,
    pub frequency_free: ProcessingOperatorFrequencyFree,
    pub frequency_fine: ProcessingOperatorFrequencyFine,
    pub feedback: ProcessingOperatorFeedback,
    pub modulation_index: ProcessingOperatorModulationIndex,
    pub volume_envelope: ProcessingOperatorEnvelope,
}


impl ProcessingOperator {
    pub fn new(operator_index: usize) -> Self {
        Self {
            volume: ProcessingOperatorVolume::default(),
            wave_type: ProcessingOperatorWaveType::default(),
            panning: ProcessingOperatorPanning::default(),
            additive_factor: ProcessingOperatorAdditiveFactor::default(),
            output_operator: ProcessingOperatorModulationTarget::opt_new(operator_index),
            frequency_ratio: ProcessingOperatorFrequencyRatio::default(),
            frequency_free: ProcessingOperatorFrequencyFree::default(),
            frequency_fine: ProcessingOperatorFrequencyFine::default(),
            feedback: ProcessingOperatorFeedback::default(),
            modulation_index: ProcessingOperatorModulationIndex::default(),
            volume_envelope: ProcessingOperatorEnvelope::new(),
        }
    }
}


pub type ProcessingOperators = [ProcessingOperator; NUM_OPERATORS];

pub struct ProcessingParameters {
    pub master_volume: ProcessingMasterVolume,
    pub master_frequency: ProcessingMasterFrequency,
    pub operators: ProcessingOperators,
}

impl ProcessingParameters {
    pub fn new() -> Self {
        Self {
            master_volume: ProcessingMasterVolume::default(),
            master_frequency: ProcessingMasterFrequency::default(),
            operators: array_init(|i| ProcessingOperator::new(i)),
        }
    }
}


impl ProcessingParameters {
    pub fn get(&mut self, index: usize) -> Option<&mut ProcessingParameterSyncValueAccess> {
        match index {
            0  => Some(&mut self.master_volume),
            1  => Some(&mut self.master_frequency),
            2  => Some(&mut self.operators[0].volume),
            3  => Some(&mut self.operators[0].panning),
            4  => Some(&mut self.operators[0].wave_type),
            5  => Some(&mut self.operators[0].modulation_index),
            6  => Some(&mut self.operators[0].feedback),
            7  => Some(&mut self.operators[0].frequency_ratio),
            8  => Some(&mut self.operators[0].frequency_free),
            9  => Some(&mut self.operators[0].frequency_fine),
            10 => Some(&mut self.operators[0].volume_envelope.attack_duration),
            11 => Some(&mut self.operators[0].volume_envelope.attack_end_value),
            12 => Some(&mut self.operators[0].volume_envelope.decay_duration),
            13 => Some(&mut self.operators[0].volume_envelope.decay_end_value),
            14 => Some(&mut self.operators[0].volume_envelope.release_duration),
            15 => Some(&mut self.operators[1].volume),
            16 => Some(&mut self.operators[1].panning),
            17 => Some(&mut self.operators[1].wave_type),
            18 => Some(&mut self.operators[1].additive_factor),
            19 => Some(&mut self.operators[1].modulation_index),
            20 => Some(&mut self.operators[1].feedback),
            21 => Some(&mut self.operators[1].frequency_ratio),
            22 => Some(&mut self.operators[1].frequency_free),
            23 => Some(&mut self.operators[1].frequency_fine),
            24 => Some(&mut self.operators[1].volume_envelope.attack_duration),
            25 => Some(&mut self.operators[1].volume_envelope.attack_end_value),
            26 => Some(&mut self.operators[1].volume_envelope.decay_duration),
            27 => Some(&mut self.operators[1].volume_envelope.decay_end_value),
            28 => Some(&mut self.operators[1].volume_envelope.release_duration),
            29 => Some(&mut self.operators[2].volume),
            30 => Some(&mut self.operators[2].panning),
            31 => Some(&mut self.operators[2].wave_type),
            32 => Some(&mut self.operators[2].additive_factor),
            33 => {
                use ProcessingOperatorModulationTarget::*;

                let opt_p = self.operators[2].output_operator.as_mut();

                if let Some(OperatorIndex2(p)) = opt_p {
                    Some(p as &mut ProcessingParameterSyncValueAccess)
                } else {
                    None
                }
            },
            34 => Some(&mut self.operators[2].modulation_index),
            35 => Some(&mut self.operators[2].feedback),
            36 => Some(&mut self.operators[2].frequency_ratio),
            37 => Some(&mut self.operators[2].frequency_free),
            38 => Some(&mut self.operators[2].frequency_fine),
            39 => Some(&mut self.operators[2].volume_envelope.attack_duration),
            40 => Some(&mut self.operators[2].volume_envelope.attack_end_value),
            41 => Some(&mut self.operators[2].volume_envelope.decay_duration),
            42 => Some(&mut self.operators[2].volume_envelope.decay_end_value),
            43 => Some(&mut self.operators[2].volume_envelope.release_duration),
            44 => Some(&mut self.operators[3].volume),
            45 => Some(&mut self.operators[3].panning),
            46 => Some(&mut self.operators[3].wave_type),
            47 => Some(&mut self.operators[3].additive_factor),
            48 => {
                use ProcessingOperatorModulationTarget::*;

                let opt_p = self.operators[3].output_operator.as_mut();

                if let Some(OperatorIndex3(p)) = opt_p {
                    Some(p as &mut ProcessingParameterSyncValueAccess)
                } else {
                    None
                }
            },
            49 => Some(&mut self.operators[3].modulation_index),
            50 => Some(&mut self.operators[3].feedback),
            51 => Some(&mut self.operators[3].frequency_ratio),
            52 => Some(&mut self.operators[3].frequency_free),
            53 => Some(&mut self.operators[3].frequency_fine),
            54 => Some(&mut self.operators[3].volume_envelope.attack_duration),
            55 => Some(&mut self.operators[3].volume_envelope.attack_end_value),
            56 => Some(&mut self.operators[3].volume_envelope.decay_duration),
            57 => Some(&mut self.operators[3].volume_envelope.decay_end_value),
            58 => Some(&mut self.operators[3].volume_envelope.release_duration),

            _  => None
        }
    }

    pub fn len(&self) -> usize {
        59
    }
}


#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::common::*;

    #[test]
    fn test_operator_panning_left_and_right(){
        use super::*;

        let mut operator = ProcessingOperatorPanning::default();

        let mut time = TimeCounter(0.0);
        let mut value = operator.get_value(time);

        operator.set_from_sync_value(1.0);

        let n = INTERPOLATION_SAMPLES_PER_STEP * INTERPOLATION_STEPS + 1;
        let mut left_and_right = [0.0, 0.0];

        for i in 0..n {
            let new_value = operator.get_value(time);
            let new_left_and_right = operator.left_and_right;

            if i >= INTERPOLATION_SAMPLES_PER_STEP &&
                i % INTERPOLATION_SAMPLES_PER_STEP == 0 {
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