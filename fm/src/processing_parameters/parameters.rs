use std::f32::consts::FRAC_PI_2;

use crate::common::*;
use crate::constants::*;

use super::common::*;
use super::utils::*;


// Macros

macro_rules! simple_parameter_string_parsing {
    ($struct_name:ident, $value:ident, $internal_type:ty) => {
        $value.parse::<$internal_type>().ok().map(|value| {
            let max = $struct_name::to_processing(1.0);
            let min = $struct_name::to_processing(0.0);

            value.max(min).min(max)
        })
    };
}


/// Implement ParameterValueConversion with 1-to-1 conversion
macro_rules! impl_parameter_value_conversion_identity {
    ($struct_name:ident) => {
        impl ParameterValueConversion for $struct_name {
            type ProcessingParameterValue = f32;

            fn to_processing(value: f32) -> Self::ProcessingParameterValue {
                value
            }
            fn to_preset(value: Self::ProcessingParameterValue) -> f32 {
                value
            }
            fn parse_string_value(value: String) -> Option<Self::ProcessingParameterValue> {
                simple_parameter_string_parsing!(Self, value, Self::ProcessingParameterValue)
            }

            fn format_processing(internal_value: Self::ProcessingParameterValue) -> String {
                format!("{:.02}", internal_value)
            }
        }
    };
}


macro_rules! create_interpolatable_processing_parameter {
    ($name:ident, $default:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            value: InterpolatableProcessingValue,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    value: InterpolatableProcessingValue::new($default)
                }
            }
        }

        impl ProcessingParameter for $name {
            type Value = f32;

            fn get_value(&mut self, time: TimeCounter) -> Self::Value {
                self.value.get_value(time, &mut |_| ())
            }
            fn get_target_value(&self) -> Self::Value {
                self.value.target_value
            }
            fn set_value(&mut self, value: Self::Value) {
                self.value.set_value(value)
            }
        }
    }
}


macro_rules! create_simple_processing_parameter {
    ($name:ident, $type:ty, $default:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub value: $type
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    value: $default
                }
            }
        }

        impl ProcessingParameter for $name {
            type Value = $type;

            fn get_value(&mut self, _: TimeCounter) -> Self::Value {
                self.value
            }
            fn get_target_value(&self) -> Self::Value {
                self.value
            }
            fn set_value(&mut self, value: Self::Value){
                self.value = value;
            }
        }
    };
}


/// Implement ParameterValueConversion for envelope durations
macro_rules! impl_envelope_duration_value_conversion {
    ($struct_name:ident) => {
        impl ParameterValueConversion for $struct_name {
            type ProcessingParameterValue = f32;

            fn to_processing(value: f32) -> Self::ProcessingParameterValue {
                // Force some decay to avoid clicks
                (value * ENVELOPE_MAX_DURATION)
                    .max(ENVELOPE_MIN_DURATION)
            }
            fn to_preset(value: Self::ProcessingParameterValue) -> f32 {
                value / ENVELOPE_MAX_DURATION
            }

            fn parse_string_value(value: String) -> Option<Self::ProcessingParameterValue> {
                value.parse::<f32>().ok().map(|value|
                    value.max(ENVELOPE_MIN_DURATION)
                        .min(ENVELOPE_MAX_DURATION)
                )
            }

            fn format_processing(internal_value: Self::ProcessingParameterValue) -> String {
                format!("{:.02}", internal_value)
            }
        }
    };
}


// Master volume

create_interpolatable_processing_parameter!(ProcessingParameterMasterVolume, DEFAULT_MASTER_VOLUME);

impl ParameterValueConversion for ProcessingParameterMasterVolume {
    type ProcessingParameterValue = f32;

    fn to_processing(value: f32) -> Self::ProcessingParameterValue {
        value * 2.0
    }
    fn to_preset(value: Self::ProcessingParameterValue) -> f32 {
        value / 2.0
    }
    fn parse_string_value(value: String) -> Option<Self::ProcessingParameterValue> {
        simple_parameter_string_parsing!(Self, value, Self::ProcessingParameterValue)
    }
    fn format_processing(internal_value: Self::ProcessingParameterValue) -> String {
        format!("{:.02}", internal_value)
    }
}


// Master frequency

create_simple_processing_parameter!(ProcessingParameterMasterFrequency, f32, DEFAULT_MASTER_FREQUENCY);

impl ParameterValueConversion for ProcessingParameterMasterFrequency {
    type ProcessingParameterValue = f32;

    fn to_processing(value: f32) -> Self::ProcessingParameterValue {
        map_parameter_value_to_value_with_steps(&MASTER_FREQUENCY_STEPS, value)
    }
    fn to_preset(value: Self::ProcessingParameterValue) -> f32 {
        map_value_to_parameter_value_with_steps(&MASTER_FREQUENCY_STEPS, value)
    }
    fn parse_string_value(value: String) -> Option<Self::ProcessingParameterValue> {
        simple_parameter_string_parsing!(Self, value, Self::ProcessingParameterValue)
    }
    fn format_processing(internal_value: Self::ProcessingParameterValue) -> String {
        format!("{:.02}", internal_value)
    }
}


// Operator volume

create_interpolatable_processing_parameter!(ProcessingParameterOperatorVolume, DEFAULT_OPERATOR_VOLUME);

impl ParameterValueConversion for ProcessingParameterOperatorVolume {
    type ProcessingParameterValue = f32;

    fn to_processing(value: f32) -> f32 {
        value * 2.0
    }
    fn to_preset(value: f32) -> f32 {
        value / 2.0
    }
    fn parse_string_value(value: String) -> Option<Self::ProcessingParameterValue> {
        simple_parameter_string_parsing!(Self, value, Self::ProcessingParameterValue)
    }

    fn format_processing(internal_value: Self::ProcessingParameterValue) -> String {
        format!("{:.02}", internal_value)
    }
}


// Additive factor

create_interpolatable_processing_parameter!(ProcessingParameterOperatorAdditiveFactor, DEFAULT_OPERATOR_ADDITIVE_FACTOR);
impl_parameter_value_conversion_identity!(ProcessingParameterOperatorAdditiveFactor);


// Frequency - ratio

create_simple_processing_parameter!(ProcessingParameterOperatorFrequencyRatio, f32, DEFAULT_OPERATOR_FREQUENCY_RATIO);

impl ParameterValueConversion for ProcessingParameterOperatorFrequencyRatio {
    type ProcessingParameterValue = f32;

    fn to_processing(value: f32) -> Self::ProcessingParameterValue {
        map_parameter_value_to_step(&OPERATOR_RATIO_STEPS[..], value)
    }
    fn to_preset(value: Self::ProcessingParameterValue) -> f32 {
        map_step_to_parameter_value(&OPERATOR_RATIO_STEPS[..], value)
    }

    fn parse_string_value(value: String) -> Option<f32> {
        value.parse::<f32>().ok().map(|value|
            round_to_step(&OPERATOR_RATIO_STEPS[..], value)
        )
    }

    fn format_processing(internal_value: Self::ProcessingParameterValue) -> String {
        format!("{:.02}", internal_value)
    }
}


// Frequency - free

create_simple_processing_parameter!(ProcessingParameterOperatorFrequencyFree, f32, DEFAULT_OPERATOR_FREQUENCY_FREE);

impl ParameterValueConversion for ProcessingParameterOperatorFrequencyFree {
    type ProcessingParameterValue = f32;

    fn to_processing(value: f32) -> Self::ProcessingParameterValue {
        map_parameter_value_to_value_with_steps(&OPERATOR_FREE_STEPS, value)
    }
    fn to_preset(value: Self::ProcessingParameterValue) -> f32 {
        map_value_to_parameter_value_with_steps(&OPERATOR_FREE_STEPS, value)
    }
    fn parse_string_value(value: String) -> Option<Self::ProcessingParameterValue> {
        simple_parameter_string_parsing!(Self, value, Self::ProcessingParameterValue)
    }

    fn format_processing(internal_value: Self::ProcessingParameterValue) -> String {
        format!("{:.02}", internal_value)
    }
}


// Frequency - fine

create_simple_processing_parameter!(ProcessingParameterOperatorFrequencyFine, f32, DEFAULT_OPERATOR_FREQUENCY_FINE);

impl ParameterValueConversion for ProcessingParameterOperatorFrequencyFine {
    type ProcessingParameterValue = f32;

    fn to_processing(value: f32) -> Self::ProcessingParameterValue {
        map_parameter_value_to_value_with_steps(&OPERATOR_FINE_STEPS, value)
    }
    fn to_preset(value: Self::ProcessingParameterValue) -> f32 {
        map_value_to_parameter_value_with_steps(&OPERATOR_FINE_STEPS, value)
    }
    fn parse_string_value(value: String) -> Option<Self::ProcessingParameterValue> {
        simple_parameter_string_parsing!(Self, value, Self::ProcessingParameterValue)
    }

    fn format_processing(internal_value: Self::ProcessingParameterValue) -> String {
        format!("{:.02}", internal_value)
    }
}


// Feedback

create_interpolatable_processing_parameter!(ProcessingParameterOperatorFeedback, DEFAULT_OPERATOR_FEEDBACK);

impl_parameter_value_conversion_identity!(ProcessingParameterOperatorFeedback);


// Modulation index

create_interpolatable_processing_parameter!(ProcessingParameterOperatorModulationIndex, DEFAULT_OPERATOR_MODULATION_INDEX);

impl ParameterValueConversion for ProcessingParameterOperatorModulationIndex {
    type ProcessingParameterValue = f32;

    fn to_processing(value: f32) -> Self::ProcessingParameterValue {
        map_parameter_value_to_value_with_steps(&OPERATOR_BETA_STEPS[..], value)
    }
    fn to_preset(value: Self::ProcessingParameterValue) -> f32 {
        map_value_to_parameter_value_with_steps(&OPERATOR_BETA_STEPS[..], value)
    }
    fn parse_string_value(value: String) -> Option<Self::ProcessingParameterValue> {
        simple_parameter_string_parsing!(Self, value, Self::ProcessingParameterValue)
    }
    fn format_processing(internal_value: Self::ProcessingParameterValue) -> String {
        format!("{:.02}", internal_value)
    }
}


// Wave type

create_simple_processing_parameter!(ProcessingParameterOperatorWaveType, WaveType, DEFAULT_OPERATOR_WAVE_TYPE);

impl ParameterValueConversion for ProcessingParameterOperatorWaveType {
    type ProcessingParameterValue = WaveType;

    fn to_processing(value: f32) -> WaveType {
        if value <= 0.5 {
            WaveType::Sine
        }
        else {
            WaveType::WhiteNoise
        }
    }
    fn to_preset(value: WaveType) -> f32 {
        match value {
            WaveType::Sine => 0.0,
            WaveType::WhiteNoise => 1.0,
        }
    }

    fn parse_string_value(value: String) -> Option<WaveType> {
        let value = value.to_lowercase();

        if value == "sine" {
            return Some(WaveType::Sine);
        } else if value == "noise" || value == "white noise" {
            return Some(WaveType::WhiteNoise);
        }

        if let Ok(value) = value.parse::<f32>() {
            return Some(Self::to_processing(value));
        }

        None
    }
    fn format_processing(internal_value: Self::ProcessingParameterValue) -> String {
        match internal_value {
            WaveType::Sine => "Sine".to_string(),
            WaveType::WhiteNoise => "White noise".to_string(),
        }
    }
}


// Attack duration

create_simple_processing_parameter!(ProcessingParameterOperatorAttackDuration, f32, DEFAULT_ENVELOPE_ATTACK_DURATION);
impl_envelope_duration_value_conversion!(ProcessingParameterOperatorAttackDuration);


// Attack volume

create_simple_processing_parameter!(ProcessingParameterOperatorAttackVolume, f32, DEFAULT_ENVELOPE_ATTACK_VOLUME);
impl_parameter_value_conversion_identity!(ProcessingParameterOperatorAttackVolume);


// Decay duration

create_simple_processing_parameter!(ProcessingParameterOperatorDecayDuration, f32, DEFAULT_ENVELOPE_DECAY_DURATION);
impl_envelope_duration_value_conversion!(ProcessingParameterOperatorDecayDuration);


// Decay volume

create_simple_processing_parameter!(ProcessingParameterOperatorDecayVolume, f32, DEFAULT_ENVELOPE_DECAY_VOLUME);
impl_parameter_value_conversion_identity!(ProcessingParameterOperatorDecayVolume);


// Release duration

create_simple_processing_parameter!(ProcessingParameterOperatorReleaseDuration, f32, DEFAULT_ENVELOPE_RELEASE_DURATION);
impl_envelope_duration_value_conversion!(ProcessingParameterOperatorReleaseDuration);


// Modulation target

create_simple_processing_parameter!(ProcessingParameterOperatorModulationTarget2, usize, DEFAULT_OPERATOR_3_MOD_TARGET);

impl ParameterValueConversion for ProcessingParameterOperatorModulationTarget2 {
    type ProcessingParameterValue = usize;

    fn to_processing(value: f32) -> Self::ProcessingParameterValue {
        map_parameter_value_to_step(&[0, 1], value)
    }
    fn to_preset(value: Self::ProcessingParameterValue) -> f32 {
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

    fn to_processing(value: f32) -> Self::ProcessingParameterValue {
        map_parameter_value_to_step(&[0, 1, 2], value)
    }
    fn to_preset(value: Self::ProcessingParameterValue) -> f32 {
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
    pub left_and_right: [f32; 2],
}

impl ProcessingParameterOperatorPanning {
    pub fn calculate_left_and_right(panning: f32) -> [f32; 2] {
        let pan_phase = panning * FRAC_PI_2;

        [pan_phase.cos(), pan_phase.sin()]
    }
}

impl ProcessingParameter for ProcessingParameterOperatorPanning {
    type Value = f32;

    fn get_value(&mut self, time: TimeCounter) -> Self::Value {
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

impl_parameter_value_conversion_identity!(ProcessingParameterOperatorPanning);