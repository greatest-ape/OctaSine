use array_init::array_init;

use crate::atomics::changed_parameters::ChangedParametersInfo;
use crate::constants::*;

pub mod common;
pub mod parameters;

pub use common::*;
pub use parameters::*;


#[derive(Debug)]
pub struct SyncOperatorEnvelope {
    pub attack_duration: SyncOperatorAttackDuration,
    pub attack_end_value: SyncOperatorAttackVolume,
    pub decay_duration: SyncOperatorDecayDuration,
    pub decay_end_value: SyncOperatorDecayVolume,
    pub release_duration: SyncOperatorReleaseDuration,
}

impl SyncOperatorEnvelope {
    pub fn new(operator_index: usize) -> Self {
        Self {
            attack_duration: SyncOperatorAttackDuration::new(operator_index),
            attack_end_value: SyncOperatorAttackVolume::new(operator_index),
            decay_duration: SyncOperatorDecayDuration::new(operator_index),
            decay_end_value: SyncOperatorDecayVolume::new(operator_index),
            release_duration: SyncOperatorReleaseDuration::new(operator_index),
        }
    }
}



#[derive(Debug)]
pub struct SyncOperator {
    pub volume: SyncOperatorVolume,
    pub wave_type: SyncOperatorWaveType,
    pub panning: SyncOperatorPanning,
    pub additive_factor: SyncOperatorAdditiveFactor,
    pub output_operator: Option<SyncOperatorModulationTarget>,
    pub frequency_ratio: SyncOperatorFrequencyRatio,
    pub frequency_free: SyncOperatorFrequencyFree,
    pub frequency_fine: SyncOperatorFrequencyFine,
    pub feedback: SyncOperatorFeedback,
    pub modulation_index: SyncOperatorModulationIndex,
    pub volume_envelope: SyncOperatorEnvelope,
}

impl SyncOperator {
    pub fn new(operator_index: usize) -> Self {
        Self {
            volume: SyncOperatorVolume::new(operator_index),
            wave_type: SyncOperatorWaveType::new(operator_index),
            panning: SyncOperatorPanning::new(operator_index),
            additive_factor: SyncOperatorAdditiveFactor::new(operator_index),
            output_operator: SyncOperatorModulationTarget::opt_new(operator_index),
            frequency_ratio: SyncOperatorFrequencyRatio::new(operator_index),
            frequency_free: SyncOperatorFrequencyFree::new(operator_index),
            frequency_fine: SyncOperatorFrequencyFine::new(operator_index),
            feedback: SyncOperatorFeedback::new(operator_index),
            modulation_index: SyncOperatorModulationIndex::new(operator_index),
            volume_envelope: SyncOperatorEnvelope::new(operator_index),
        }
    }
}



pub struct SyncParameters {
    pub master_volume: SyncMasterVolume,
    pub master_frequency: SyncMasterFrequency,
    pub operators: [SyncOperator; NUM_OPERATORS],
    pub changed_info: ChangedParametersInfo,
}

impl SyncParameters {
    pub fn new() -> Self {
        Self {
            master_volume: SyncMasterVolume::default(),
            master_frequency: SyncMasterFrequency::default(),
            operators: array_init(|i| SyncOperator::new(i)),
            changed_info: ChangedParametersInfo::new(),
        }
    }
}


impl SyncParameters {
    pub fn get(&self, index: usize) -> Option<&SyncParameter> {
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
                use SyncOperatorModulationTarget::*;

                let opt_p = self.operators[2].output_operator.as_ref();

                if let Some(OperatorIndex2(p)) = opt_p {
                    Some(p as &SyncParameter)
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
                use SyncOperatorModulationTarget::*;

                let opt_p = self.operators[3].output_operator.as_ref();

                if let Some(OperatorIndex3(p)) = opt_p {
                    Some(p as &SyncParameter)
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
    fn test_sync_parameters_len(){
        // Required for ChangedParametersInfo
        assert!(SyncParameters::new().len() <= 64);
    }
}