use std::f64::consts::FRAC_PI_2;

use vst2_helpers::processing_parameters::*;
use vst2_helpers::processing_parameters::utils::*;
use vst2_helpers::processing_parameters::interpolatable_value::*;

use vst2_helpers::*;

use crate::common::*;
use crate::constants::*;


// Master volume

create_interpolatable_processing_parameter!(
    ProcessingParameterMasterVolume,
    DEFAULT_MASTER_VOLUME,
    TimeCounter
);


// Master frequency

create_simple_processing_parameter!(
    ProcessingParameterMasterFrequency,
    f64,
    DEFAULT_MASTER_FREQUENCY
);


// Operator volume

create_interpolatable_processing_parameter!(ProcessingParameterOperatorVolume, DEFAULT_OPERATOR_VOLUME, TimeCounter);

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

create_interpolatable_processing_parameter!(ProcessingParameterOperatorAdditiveFactor, DEFAULT_OPERATOR_ADDITIVE_FACTOR, TimeCounter);


// Frequency - ratio

create_simple_processing_parameter!(ProcessingParameterOperatorFrequencyRatio, f64, DEFAULT_OPERATOR_FREQUENCY_RATIO);


// Frequency - free

create_simple_processing_parameter!(ProcessingParameterOperatorFrequencyFree, f64, DEFAULT_OPERATOR_FREQUENCY_FREE);


// Frequency - fine

create_simple_processing_parameter!(ProcessingParameterOperatorFrequencyFine, f64, DEFAULT_OPERATOR_FREQUENCY_FINE);


// Feedback

create_interpolatable_processing_parameter!(ProcessingParameterOperatorFeedback, DEFAULT_OPERATOR_FEEDBACK, TimeCounter);


// Modulation index

create_interpolatable_processing_parameter!(ProcessingParameterOperatorModulationIndex, DEFAULT_OPERATOR_MODULATION_INDEX, TimeCounter);


// Wave type

create_simple_processing_parameter!(ProcessingParameterOperatorWaveType, WaveType, DEFAULT_OPERATOR_WAVE_TYPE);


// Attack duration

create_simple_processing_parameter!(ProcessingParameterOperatorAttackDuration, f64, DEFAULT_ENVELOPE_ATTACK_DURATION);


// Attack volume

create_simple_processing_parameter!(ProcessingParameterOperatorAttackVolume, f64, DEFAULT_ENVELOPE_ATTACK_VOLUME);


// Decay duration

create_simple_processing_parameter!(ProcessingParameterOperatorDecayDuration, f64, DEFAULT_ENVELOPE_DECAY_DURATION);


// Decay volume

create_simple_processing_parameter!(ProcessingParameterOperatorDecayVolume, f64, DEFAULT_ENVELOPE_DECAY_VOLUME);


// Release duration

create_simple_processing_parameter!(ProcessingParameterOperatorReleaseDuration, f64, DEFAULT_ENVELOPE_RELEASE_DURATION);


// Modulation target

create_simple_processing_parameter!(ProcessingParameterOperatorModulationTarget2, usize, DEFAULT_OPERATOR_3_MOD_TARGET);

impl ParameterValueConversion for ProcessingParameterOperatorModulationTarget2 {
    type ProcessingParameterValue = usize;

    fn to_processing(value: f64) -> Self::ProcessingParameterValue {
        map_parameter_value_to_step(&[0, 1], value)
    }
    fn to_preset(value: Self::ProcessingParameterValue) -> f64 {
        map_step_to_parameter_value(&[0, 1], value)
    }

    fn parse_string_value(value: String) -> Option<usize> {
        if let Ok(value) = value.parse::<usize>(){
            if value == 1 || value == 2 {
                return Some(value - 1);
            }
        }

        None
    }

    fn format_processing(internal_value: Self::ProcessingParameterValue) -> String {
        format!("Operator {}", internal_value + 1)
    }
}


create_simple_processing_parameter!(ProcessingParameterOperatorModulationTarget3, usize, DEFAULT_OPERATOR_4_MOD_TARGET);

impl ParameterValueConversion for ProcessingParameterOperatorModulationTarget3 {
    type ProcessingParameterValue = usize;

    fn to_processing(value: f64) -> Self::ProcessingParameterValue {
        map_parameter_value_to_step(&[0, 1, 2], value)
    }
    fn to_preset(value: Self::ProcessingParameterValue) -> f64 {
        map_step_to_parameter_value(&[0, 1, 2], value)
    }

    fn parse_string_value(value: String) -> Option<usize> {
        if let Ok(value) = value.parse::<usize>(){
            if value == 1 || value == 2 || value == 3 {
                return Some(value - 1);
            }
        }

        None
    }

    fn format_processing(internal_value: Self::ProcessingParameterValue) -> String {
        format!("Operator {}", internal_value + 1)
    }
}


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
