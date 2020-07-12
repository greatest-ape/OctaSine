//! Processing parameters

use array_init::array_init;

use vst2_helpers::processing_parameters::*;

pub mod parameters;
pub use parameters::*;

use crate::constants::*;


#[derive(Debug, Clone)]
pub struct ProcessingParameterOperatorEnvelope {
    pub attack_duration: ProcessingParameterOperatorAttackDuration,
    pub attack_end_value: ProcessingParameterOperatorAttackVolume,
    pub decay_duration: ProcessingParameterOperatorDecayDuration,
    pub decay_end_value: ProcessingParameterOperatorDecayVolume,
    pub release_duration: ProcessingParameterOperatorReleaseDuration,
}

impl ProcessingParameterOperatorEnvelope {
    fn new() -> Self {
        Self {
            attack_duration: ProcessingParameterOperatorAttackDuration::default(),
            attack_end_value: ProcessingParameterOperatorAttackVolume::default(),
            decay_duration: ProcessingParameterOperatorDecayDuration::default(),
            decay_end_value: ProcessingParameterOperatorDecayVolume::default(),
            release_duration: ProcessingParameterOperatorReleaseDuration::default(),
        }
    }
}


#[derive(Debug)]
pub struct ProcessingParameterOperator {
    pub volume: ProcessingParameterOperatorVolume,
    pub wave_type: ProcessingParameterOperatorWaveType,
    pub panning: ProcessingParameterOperatorPanning,
    pub additive_factor: ProcessingParameterOperatorAdditiveFactor,
    pub output_operator: Option<ProcessingParameterOperatorModulationTarget>,
    pub frequency_ratio: ProcessingParameterOperatorFrequencyRatio,
    pub frequency_free: ProcessingParameterOperatorFrequencyFree,
    pub frequency_fine: ProcessingParameterOperatorFrequencyFine,
    pub feedback: ProcessingParameterOperatorFeedback,
    pub modulation_index: ProcessingParameterOperatorModulationIndex,
    pub volume_envelope: ProcessingParameterOperatorEnvelope,
}


impl ProcessingParameterOperator {
    pub fn new(operator_index: usize) -> Self {
        Self {
            volume: ProcessingParameterOperatorVolume::default(),
            wave_type: ProcessingParameterOperatorWaveType::default(),
            panning: ProcessingParameterOperatorPanning::default(),
            additive_factor: ProcessingParameterOperatorAdditiveFactor::default(),
            output_operator: ProcessingParameterOperatorModulationTarget::opt_new(operator_index),
            frequency_ratio: ProcessingParameterOperatorFrequencyRatio::default(),
            frequency_free: ProcessingParameterOperatorFrequencyFree::default(),
            frequency_fine: ProcessingParameterOperatorFrequencyFine::default(),
            feedback: ProcessingParameterOperatorFeedback::default(),
            modulation_index: ProcessingParameterOperatorModulationIndex::default(),
            volume_envelope: ProcessingParameterOperatorEnvelope::new(),
        }
    }
}


pub type ProcessingParameterOperators = [ProcessingParameterOperator; NUM_OPERATORS];

pub struct ProcessingParameters {
    pub master_volume: ProcessingParameterMasterVolume,
    pub master_frequency: ProcessingParameterMasterFrequency,
    pub operators: ProcessingParameterOperators,
}

impl Default for ProcessingParameters {
    fn default() -> Self {
        Self {
            master_volume: ProcessingParameterMasterVolume::default(),
            master_frequency: ProcessingParameterMasterFrequency::default(),
            operators: array_init(ProcessingParameterOperator::new),
        }
    }
}


#[allow(clippy::len_without_is_empty)]
impl ProcessingParameters {
    pub fn get(&mut self, index: usize) -> Option<&mut dyn ProcessingParameterPresetValueAccess> {
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
                use ProcessingParameterOperatorModulationTarget::*;

                let opt_p = self.operators[2].output_operator.as_mut();

                if let Some(OperatorIndex2(p)) = opt_p {
                    Some(p as &mut dyn ProcessingParameterPresetValueAccess)
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
                use ProcessingParameterOperatorModulationTarget::*;

                let opt_p = self.operators[3].output_operator.as_mut();

                if let Some(OperatorIndex3(p)) = opt_p {
                    Some(p as &mut dyn ProcessingParameterPresetValueAccess)
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

    #[test]
    fn test_operator_panning_left_and_right(){
        use vst2_helpers::processing_parameters::interpolatable_value::*;
        use super::*;

        let mut operator = ProcessingParameterOperatorPanning::default();

        let mut time = TimeCounter(0.0);
        let mut value = operator.get_value(time);

        operator.set_from_preset_value(1.0);

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