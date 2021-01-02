use array_init::array_init;

pub mod interpolatable_value;
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
    pub fn new() -> Self {
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
            volume: ProcessingParameterOperatorVolume::new(operator_index),
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

#[derive(Default)]
pub struct ProcessingParameterLfo {
    pub target_parameter: ProcessingParameterLfoTargetParameter,
    pub shape: ProcessingParameterLfoShape,
    pub mode: ProcessingParameterLfoMode,
    pub bpm_sync: ProcessingParameterLfoBpmSync,
    pub speed: ProcessingParameterLfoSpeed,
    pub magnitude: ProcessingParameterLfoMagnitude,
}

pub type ProcessingParameterLfos = [ProcessingParameterLfo; NUM_LFOS];

pub struct ProcessingParameters {
    pub master_volume: ProcessingParameterMasterVolume,
    pub master_frequency: ProcessingParameterMasterFrequency,
    pub operators: ProcessingParameterOperators,
    pub lfos: ProcessingParameterLfos,
}


impl Default for ProcessingParameters {
    fn default() -> Self {
        Self {
            master_volume: ProcessingParameterMasterVolume::default(),
            master_frequency: ProcessingParameterMasterFrequency::default(),
            operators: array_init(ProcessingParameterOperator::new),
            lfos: array_init(|_| ProcessingParameterLfo::default()),
        }
    }
}


#[allow(clippy::len_without_is_empty)]
impl ProcessingParameters {
    pub fn set_from_sync(&mut self, index: usize, value: f64){
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
            10 => self.operators[0].volume_envelope.attack_duration.set_from_sync(value),
            11 => self.operators[0].volume_envelope.attack_end_value.set_from_sync(value),
            12 => self.operators[0].volume_envelope.decay_duration.set_from_sync(value),
            13 => self.operators[0].volume_envelope.decay_end_value.set_from_sync(value),
            14 => self.operators[0].volume_envelope.release_duration.set_from_sync(value),
            15 => self.operators[1].volume.set_from_sync(value),
            16 => self.operators[1].panning.set_from_sync(value),
            17 => self.operators[1].wave_type.set_from_sync(value),
            18 => self.operators[1].additive_factor.set_from_sync(value),
            19 => self.operators[1].modulation_index.set_from_sync(value),
            20 => self.operators[1].feedback.set_from_sync(value),
            21 => self.operators[1].frequency_ratio.set_from_sync(value),
            22 => self.operators[1].frequency_free.set_from_sync(value),
            23 => self.operators[1].frequency_fine.set_from_sync(value),
            24 => self.operators[1].volume_envelope.attack_duration.set_from_sync(value),
            25 => self.operators[1].volume_envelope.attack_end_value.set_from_sync(value),
            26 => self.operators[1].volume_envelope.decay_duration.set_from_sync(value),
            27 => self.operators[1].volume_envelope.decay_end_value.set_from_sync(value),
            28 => self.operators[1].volume_envelope.release_duration.set_from_sync(value),
            29 => self.operators[2].volume.set_from_sync(value),
            30 => self.operators[2].panning.set_from_sync(value),
            31 => self.operators[2].wave_type.set_from_sync(value),
            32 => self.operators[2].additive_factor.set_from_sync(value),
            33 => {
                use ProcessingParameterOperatorModulationTarget::*;

                let opt_p = self.operators[2].output_operator.as_mut();

                if let Some(OperatorIndex2(p)) = opt_p {
                    p.set_from_sync(value)
                }            },
            34 => self.operators[2].modulation_index.set_from_sync(value),
            35 => self.operators[2].feedback.set_from_sync(value),
            36 => self.operators[2].frequency_ratio.set_from_sync(value),
            37 => self.operators[2].frequency_free.set_from_sync(value),
            38 => self.operators[2].frequency_fine.set_from_sync(value),
            39 => self.operators[2].volume_envelope.attack_duration.set_from_sync(value),
            40 => self.operators[2].volume_envelope.attack_end_value.set_from_sync(value),
            41 => self.operators[2].volume_envelope.decay_duration.set_from_sync(value),
            42 => self.operators[2].volume_envelope.decay_end_value.set_from_sync(value),
            43 => self.operators[2].volume_envelope.release_duration.set_from_sync(value),
            44 => self.operators[3].volume.set_from_sync(value),
            45 => self.operators[3].panning.set_from_sync(value),
            46 => self.operators[3].wave_type.set_from_sync(value),
            47 => self.operators[3].additive_factor.set_from_sync(value),
            48 => {
                use ProcessingParameterOperatorModulationTarget::*;

                let opt_p = self.operators[3].output_operator.as_mut();

                if let Some(OperatorIndex3(p)) = opt_p {
                    p.set_from_sync(value)
                }
            },
            49 => self.operators[3].modulation_index.set_from_sync(value),
            50 => self.operators[3].feedback.set_from_sync(value),
            51 => self.operators[3].frequency_ratio.set_from_sync(value),
            52 => self.operators[3].frequency_free.set_from_sync(value),
            53 => self.operators[3].frequency_fine.set_from_sync(value),
            54 => self.operators[3].volume_envelope.attack_duration.set_from_sync(value),
            55 => self.operators[3].volume_envelope.attack_end_value.set_from_sync(value),
            56 => self.operators[3].volume_envelope.decay_duration.set_from_sync(value),
            57 => self.operators[3].volume_envelope.decay_end_value.set_from_sync(value),
            58 => self.operators[3].volume_envelope.release_duration.set_from_sync(value),
            59 => self.lfos[0].target_parameter.set_from_sync(value),
            60 => self.lfos[0].shape.set_from_sync(value),
            61 => self.lfos[0].mode.set_from_sync(value),
            62 => self.lfos[0].bpm_sync.set_from_sync(value),
            63 => self.lfos[0].speed.set_from_sync(value),
            64 => self.lfos[0].magnitude.set_from_sync(value),
            _  => ()
        }
    }

    pub fn len(&self) -> usize {
        65
    }
}


#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_operator_panning_left_and_right(){
        use crate::common::*;
        use super::interpolatable_value::*;
        use super::*;

        let mut operator = ProcessingParameterOperatorPanning::default();

        let mut time = TimeCounter(0.0);
        let mut value = operator.get_value(time);

        operator.set_from_sync(1.0);

        let n = INTERPOLATION_SAMPLES_PER_STEP * INTERPOLATION_STEPS + 1;
        let mut left_and_right = [0.0, 0.0];

        for i in 0..n {
            let new_value = operator.get_value(time);
            let new_left_and_right = operator.left_and_right;

            #[allow(clippy::float_cmp)]
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