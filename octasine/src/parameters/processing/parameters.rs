use std::f64::consts::FRAC_PI_2;

use vst2_helpers::processing_parameters::*;
use vst2_helpers::processing_parameters::interpolatable_value::*;

use vst2_helpers::*;

use crate::common::*;
use crate::constants::*;

use super::values::*;


// Master volume

create_interpolatable_processing_parameter!(
    ProcessingParameterMasterVolume,
    MasterVolume,
    DEFAULT_MASTER_VOLUME,
    TimeCounter
);


// Master frequency

create_simple_processing_parameter!(
    ProcessingParameterMasterFrequency,
    MasterFrequency,
    f64,
    DEFAULT_MASTER_FREQUENCY
);


// Operator volume

create_interpolatable_processing_parameter!(
    ProcessingParameterOperatorVolume,
    OperatorVolume,
    DEFAULT_OPERATOR_VOLUME,
    TimeCounter
);

impl ProcessingParameterOperatorVolume {
    pub fn new(operator_index: usize) -> Self {
        let mut parameter = Self::default();

        if operator_index > 0 {
            parameter.set_value(0.0);
        }

        parameter
    }
}


// Additive factor

create_interpolatable_processing_parameter!(
    ProcessingParameterOperatorAdditiveFactor,
    OperatorAdditive,
    DEFAULT_OPERATOR_ADDITIVE_FACTOR,
    TimeCounter
);


// Frequency - ratio

create_simple_processing_parameter!(
    ProcessingParameterOperatorFrequencyRatio,
    OperatorFrequencyRatio,
    f64,
    DEFAULT_OPERATOR_FREQUENCY_RATIO
);


// Frequency - free

create_simple_processing_parameter!(
    ProcessingParameterOperatorFrequencyFree,
    OperatorFrequencyFree,
    f64,
    DEFAULT_OPERATOR_FREQUENCY_FREE
);


// Frequency - fine

create_simple_processing_parameter!(
    ProcessingParameterOperatorFrequencyFine,
    OperatorFrequencyFine,
    f64,
    DEFAULT_OPERATOR_FREQUENCY_FINE
);


// Feedback

create_interpolatable_processing_parameter!(
    ProcessingParameterOperatorFeedback,
    OperatorFeedback,
    DEFAULT_OPERATOR_FEEDBACK,
    TimeCounter
);


// Modulation index

create_interpolatable_processing_parameter!(
    ProcessingParameterOperatorModulationIndex,
    OperatorModulationIndex,
    DEFAULT_OPERATOR_MODULATION_INDEX,
    TimeCounter
);


// Wave type

create_simple_processing_parameter!(
    ProcessingParameterOperatorWaveType,
    OperatorWaveType,
    WaveType,
    DEFAULT_OPERATOR_WAVE_TYPE
);


// Attack duration

create_simple_processing_parameter!(
    ProcessingParameterOperatorAttackDuration,
    OperatorAttackDuration,
    f64,
    DEFAULT_ENVELOPE_ATTACK_DURATION
);


// Attack volume

create_simple_processing_parameter!(
    ProcessingParameterOperatorAttackVolume,
    OperatorAttackVolume,
    f64,
    DEFAULT_ENVELOPE_ATTACK_VOLUME
);


// Decay duration

create_simple_processing_parameter!(
    ProcessingParameterOperatorDecayDuration,
    OperatorDecayDuration,
    f64,
    DEFAULT_ENVELOPE_DECAY_DURATION
);


// Decay volume

create_simple_processing_parameter!(
    ProcessingParameterOperatorDecayVolume,
    OperatorDecayVolume,
    f64,
    DEFAULT_ENVELOPE_DECAY_VOLUME
);


// Release duration

create_simple_processing_parameter!(
    ProcessingParameterOperatorReleaseDuration,
    OperatorReleaseDuration,
    f64,
    DEFAULT_ENVELOPE_RELEASE_DURATION
);


// Modulation target

create_simple_processing_parameter!(
    ProcessingParameterOperatorModulationTarget2,
    OperatorModulationTarget2,
    usize,
    DEFAULT_OPERATOR_3_MOD_TARGET
);


create_simple_processing_parameter!(
    ProcessingParameterOperatorModulationTarget3,
    OperatorModulationTarget3,
    usize,
    DEFAULT_OPERATOR_4_MOD_TARGET
);


#[derive(Debug)]
pub enum ProcessingParameterOperatorModulationTarget {
    OperatorIndex2(ProcessingParameterOperatorModulationTarget2),
    OperatorIndex3(ProcessingParameterOperatorModulationTarget3),
}


impl ProcessingParameterOperatorModulationTarget {
    pub fn opt_new(operator_index: usize) -> Option<Self> {
        match operator_index {
            2 => Some(ProcessingParameterOperatorModulationTarget::OperatorIndex2(
                ProcessingParameterOperatorModulationTarget2::default()
            )),
            3 => Some(ProcessingParameterOperatorModulationTarget::OperatorIndex3(
                ProcessingParameterOperatorModulationTarget3::default()
            )),
            _ => None
        }
    }
}


// Panning

#[derive(Debug, Clone)]
pub struct ProcessingParameterOperatorPanning {
    value: InterpolatableProcessingValue,
    pub left_and_right: [f64; 2],
}


impl ProcessingParameterOperatorPanning {
    pub fn calculate_left_and_right(panning: f64) -> [f64; 2] {
        let pan_phase = panning * FRAC_PI_2;

        [pan_phase.cos(), pan_phase.sin()]
    }
}


impl ProcessingParameter for ProcessingParameterOperatorPanning {
    type Value = f64;
    type ExtraData = TimeCounter;

    fn get_value(&mut self, time: Self::ExtraData) -> Self::Value {
        let mut opt_new_left_and_right = None;

        let value = self.value.get_value(time, &mut |new_panning| {
            opt_new_left_and_right =
                Some(Self::calculate_left_and_right(new_panning));
        });

        if let Some(new_left_and_right) = opt_new_left_and_right {
            self.left_and_right = new_left_and_right;
        }

        value
    }
    fn set_value(&mut self, value: Self::Value) {
        self.value.set_value(value)
    }
    fn get_target_value(&self) -> Self::Value {
        self.value.target_value
    }
    fn set_from_sync(&mut self, value: f64) {
        self.set_value(OperatorPanning::from_sync(value).0)
    }
}


impl Default for ProcessingParameterOperatorPanning {
    fn default() -> Self {
        let default = DEFAULT_OPERATOR_PANNING;

        Self {
            value: InterpolatableProcessingValue::new(default),
            left_and_right: Self::calculate_left_and_right(default),
        }
    }
}
