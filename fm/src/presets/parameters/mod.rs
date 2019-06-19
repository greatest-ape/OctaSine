use array_init::array_init;

use crate::constants::*;

pub mod atomic_float;
pub mod common;
pub mod parameters;

pub use atomic_float::AtomicFloat;
pub use common::*;
pub use parameters::*;


#[derive(Debug)]
pub struct PresetParameterOperatorEnvelope {
    pub attack_duration: PresetParameterOperatorAttackDuration,
    pub attack_end_value: PresetParameterOperatorAttackVolume,
    pub decay_duration: PresetParameterOperatorDecayDuration,
    pub decay_end_value: PresetParameterOperatorDecayVolume,
    pub release_duration: PresetParameterOperatorReleaseDuration,
}

impl PresetParameterOperatorEnvelope {
    pub fn new(operator_index: usize) -> Self {
        Self {
            attack_duration: PresetParameterOperatorAttackDuration::new(operator_index),
            attack_end_value: PresetParameterOperatorAttackVolume::new(operator_index),
            decay_duration: PresetParameterOperatorDecayDuration::new(operator_index),
            decay_end_value: PresetParameterOperatorDecayVolume::new(operator_index),
            release_duration: PresetParameterOperatorReleaseDuration::new(operator_index),
        }
    }
}



#[derive(Debug)]
pub struct PresetParameterOperator {
    pub volume: PresetParameterOperatorVolume,
    pub wave_type: PresetParameterOperatorWaveType,
    pub panning: PresetParameterOperatorPanning,
    pub additive_factor: PresetParameterOperatorAdditiveFactor,
    pub output_operator: Option<PresetParameterOperatorModulationTarget>,
    pub frequency_ratio: PresetParameterOperatorFrequencyRatio,
    pub frequency_free: PresetParameterOperatorFrequencyFree,
    pub frequency_fine: PresetParameterOperatorFrequencyFine,
    pub feedback: PresetParameterOperatorFeedback,
    pub modulation_index: PresetParameterOperatorModulationIndex,
    pub volume_envelope: PresetParameterOperatorEnvelope,
}

impl PresetParameterOperator {
    pub fn new(operator_index: usize) -> Self {
        Self {
            volume: PresetParameterOperatorVolume::new(operator_index),
            wave_type: PresetParameterOperatorWaveType::new(operator_index),
            panning: PresetParameterOperatorPanning::new(operator_index),
            additive_factor: PresetParameterOperatorAdditiveFactor::new(operator_index),
            output_operator: PresetParameterOperatorModulationTarget::opt_new(operator_index),
            frequency_ratio: PresetParameterOperatorFrequencyRatio::new(operator_index),
            frequency_free: PresetParameterOperatorFrequencyFree::new(operator_index),
            frequency_fine: PresetParameterOperatorFrequencyFine::new(operator_index),
            feedback: PresetParameterOperatorFeedback::new(operator_index),
            modulation_index: PresetParameterOperatorModulationIndex::new(operator_index),
            volume_envelope: PresetParameterOperatorEnvelope::new(operator_index),
        }
    }
}



pub struct PresetParameters {
    pub master_volume: PresetParameterMasterVolume,
    pub master_frequency: PresetParameterMasterFrequency,
    pub operators: [PresetParameterOperator; NUM_OPERATORS],
}

impl PresetParameters {
    pub fn new() -> Self {
        Self {
            master_volume: PresetParameterMasterVolume::default(),
            master_frequency: PresetParameterMasterFrequency::default(),
            operators: array_init(|i| PresetParameterOperator::new(i)),
        }
    }
}


impl PresetParameters {
    pub fn get(&self, index: usize) -> Option<&PresetParameter> {
        match index {
            0  => Some(&self.master_volume),
            1  => Some(&self.master_frequency),
            2  => Some(&self.operators[0].volume),
            3  => Some(&self.operators[0].panning),
            4  => Some(&self.operators[0].wave_type),
            5  => Some(&self.operators[0].modulation_index),
            6  => Some(&self.operators[0].feedback),
            7  => Some(&self.operators[0].frequency_ratio),
            8  => Some(&self.operators[0].frequency_free),
            9  => Some(&self.operators[0].frequency_fine),
            10 => Some(&self.operators[0].volume_envelope.attack_duration),
            11 => Some(&self.operators[0].volume_envelope.attack_end_value),
            12 => Some(&self.operators[0].volume_envelope.decay_duration),
            13 => Some(&self.operators[0].volume_envelope.decay_end_value),
            14 => Some(&self.operators[0].volume_envelope.release_duration),
            15 => Some(&self.operators[1].volume),
            16 => Some(&self.operators[1].panning),
            17 => Some(&self.operators[1].wave_type),
            18 => Some(&self.operators[1].additive_factor),
            19 => Some(&self.operators[1].modulation_index),
            20 => Some(&self.operators[1].feedback),
            21 => Some(&self.operators[1].frequency_ratio),
            22 => Some(&self.operators[1].frequency_free),
            23 => Some(&self.operators[1].frequency_fine),
            24 => Some(&self.operators[1].volume_envelope.attack_duration),
            25 => Some(&self.operators[1].volume_envelope.attack_end_value),
            26 => Some(&self.operators[1].volume_envelope.decay_duration),
            27 => Some(&self.operators[1].volume_envelope.decay_end_value),
            28 => Some(&self.operators[1].volume_envelope.release_duration),
            29 => Some(&self.operators[2].volume),
            30 => Some(&self.operators[2].panning),
            31 => Some(&self.operators[2].wave_type),
            32 => Some(&self.operators[2].additive_factor),
            33 => {
                use PresetParameterOperatorModulationTarget::*;

                let opt_p = self.operators[2].output_operator.as_ref();

                if let Some(OperatorIndex2(p)) = opt_p {
                    Some(p as &PresetParameter)
                } else {
                    None
                }
            },
            34 => Some(&self.operators[2].modulation_index),
            35 => Some(&self.operators[2].feedback),
            36 => Some(&self.operators[2].frequency_ratio),
            37 => Some(&self.operators[2].frequency_free),
            38 => Some(&self.operators[2].frequency_fine),
            39 => Some(&self.operators[2].volume_envelope.attack_duration),
            40 => Some(&self.operators[2].volume_envelope.attack_end_value),
            41 => Some(&self.operators[2].volume_envelope.decay_duration),
            42 => Some(&self.operators[2].volume_envelope.decay_end_value),
            43 => Some(&self.operators[2].volume_envelope.release_duration),
            44 => Some(&self.operators[3].volume),
            45 => Some(&self.operators[3].panning),
            46 => Some(&self.operators[3].wave_type),
            47 => Some(&self.operators[3].additive_factor),
            48 => {
                use PresetParameterOperatorModulationTarget::*;

                let opt_p = self.operators[3].output_operator.as_ref();

                if let Some(OperatorIndex3(p)) = opt_p {
                    Some(p as &PresetParameter)
                } else {
                    None
                }
            },
            49 => Some(&self.operators[3].modulation_index),
            50 => Some(&self.operators[3].feedback),
            51 => Some(&self.operators[3].frequency_ratio),
            52 => Some(&self.operators[3].frequency_free),
            53 => Some(&self.operators[3].frequency_fine),
            54 => Some(&self.operators[3].volume_envelope.attack_duration),
            55 => Some(&self.operators[3].volume_envelope.attack_end_value),
            56 => Some(&self.operators[3].volume_envelope.decay_duration),
            57 => Some(&self.operators[3].volume_envelope.decay_end_value),
            58 => Some(&self.operators[3].volume_envelope.release_duration),

            _  => None
        }
    }

    pub fn len(&self) -> usize {
        59
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preset_parameters_len(){
        // Required for ChangedParametersInfo
        assert!(PresetParameters::new().len() <= 64);
    }
}