use std::f32::consts::FRAC_PI_2;

use crate::common::*;
use crate::constants::*;

use super::super::utils::*;
use super::super::common::*;

use super::common::*;


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
            type ProcessingValue = f32;

            fn to_processing(value: f32) -> Self::ProcessingValue {
                value
            }
            fn to_sync(value: Self::ProcessingValue) -> f32 {
                value
            }
            fn parse_string_value(value: String) -> Option<Self::ProcessingValue> {
                simple_parameter_string_parsing!(Self, value, Self::ProcessingValue)
            }

            fn format_processing(internal_value: Self::ProcessingValue) -> String {
                format!("{:.02}", internal_value)
            }
        }
    };
}


macro_rules! create_interpolatable_processing_parameter {
    ($name:ident, $default:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            value: TimeInterpolatableValue,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    value: TimeInterpolatableValue::new($default)
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
            type ProcessingValue = f32;

            fn to_processing(value: f32) -> Self::ProcessingValue {
                // Force some decay to avoid clicks
                (value * ENVELOPE_MAX_DURATION)
                    .max(ENVELOPE_MIN_DURATION)
            }
            fn to_sync(value: Self::ProcessingValue) -> f32 {
                value / ENVELOPE_MAX_DURATION
            }

            fn parse_string_value(value: String) -> Option<Self::ProcessingValue> {
                value.parse::<f32>().ok().map(|value|
                    value.max(ENVELOPE_MIN_DURATION)
                        .min(ENVELOPE_MAX_DURATION)
                )
            }

            fn format_processing(internal_value: Self::ProcessingValue) -> String {
                format!("{:.02}", internal_value)
            }
        }
    };
}


// Master volume

create_interpolatable_processing_parameter!(ProcessingMasterVolume, DEFAULT_MASTER_VOLUME);

impl ParameterValueConversion for ProcessingMasterVolume {
    type ProcessingValue = f32;

    fn to_processing(value: f32) -> Self::ProcessingValue {
        value * 2.0
    }
    fn to_sync(value: Self::ProcessingValue) -> f32 {
        value / 2.0
    }
    fn parse_string_value(value: String) -> Option<Self::ProcessingValue> {
        simple_parameter_string_parsing!(Self, value, Self::ProcessingValue)
    }
    fn format_processing(internal_value: Self::ProcessingValue) -> String {
        format!("{:.02}", internal_value)
    }
}


// Master frequency

create_simple_processing_parameter!(ProcessingMasterFrequency, f32, DEFAULT_MASTER_FREQUENCY);

impl ParameterValueConversion for ProcessingMasterFrequency {
    type ProcessingValue = f32;

    fn to_processing(value: f32) -> Self::ProcessingValue {
        map_parameter_value_to_value_with_steps(&MASTER_FREQUENCY_STEPS, value)
    }
    fn to_sync(value: Self::ProcessingValue) -> f32 {
        map_value_to_parameter_value_with_steps(&MASTER_FREQUENCY_STEPS, value)
    }
    fn parse_string_value(value: String) -> Option<Self::ProcessingValue> {
        simple_parameter_string_parsing!(Self, value, Self::ProcessingValue)
    }
    fn format_processing(internal_value: Self::ProcessingValue) -> String {
        format!("{:.02}", internal_value)
    }
}


// Operator volume

create_interpolatable_processing_parameter!(ProcessingOperatorVolume, OPERATOR_DEFAULT_VOLUME);

impl ParameterValueConversion for ProcessingOperatorVolume {
    type ProcessingValue = f32;

    fn to_processing(value: f32) -> f32 {
        value * 2.0
    }
    fn to_sync(value: f32) -> f32 {
        value / 2.0
    }
    fn parse_string_value(value: String) -> Option<Self::ProcessingValue> {
        simple_parameter_string_parsing!(Self, value, Self::ProcessingValue)
    }

    fn format_processing(internal_value: Self::ProcessingValue) -> String {
        format!("{:.02}", internal_value)
    }
}


// Additive factor

create_interpolatable_processing_parameter!(ProcessingOperatorAdditiveFactor, OPERATOR_DEFAULT_ADDITIVE_FACTOR);
impl_parameter_value_conversion_identity!(ProcessingOperatorAdditiveFactor);


// Frequency - ratio

create_simple_processing_parameter!(ProcessingOperatorFrequencyRatio, f32, OPERATOR_DEFAULT_FREQUENCY_RATIO);

impl ParameterValueConversion for ProcessingOperatorFrequencyRatio {
    type ProcessingValue = f32;

    fn to_processing(value: f32) -> Self::ProcessingValue {
        map_parameter_value_to_step(&OPERATOR_RATIO_STEPS[..], value)
    }
    fn to_sync(value: Self::ProcessingValue) -> f32 {
        map_step_to_parameter_value(&OPERATOR_RATIO_STEPS[..], value)
    }

    fn parse_string_value(value: String) -> Option<f32> {
        value.parse::<f32>().ok().map(|value|
            round_to_step(&OPERATOR_RATIO_STEPS[..], value)
        )
    }

    fn format_processing(internal_value: Self::ProcessingValue) -> String {
        format!("{:.02}", internal_value)
    }
}


// Frequency - free

create_simple_processing_parameter!(ProcessingOperatorFrequencyFree, f32, OPERATOR_DEFAULT_FREQUENCY_FREE);

impl ParameterValueConversion for ProcessingOperatorFrequencyFree {
    type ProcessingValue = f32;

    fn to_processing(value: f32) -> Self::ProcessingValue {
        map_parameter_value_to_value_with_steps(&OPERATOR_FREE_STEPS, value)
    }
    fn to_sync(value: Self::ProcessingValue) -> f32 {
        map_value_to_parameter_value_with_steps(&OPERATOR_FREE_STEPS, value)
    }
    fn parse_string_value(value: String) -> Option<Self::ProcessingValue> {
        simple_parameter_string_parsing!(Self, value, Self::ProcessingValue)
    }

    fn format_processing(internal_value: Self::ProcessingValue) -> String {
        format!("{:.02}", internal_value)
    }
}


// Frequency - fine

create_simple_processing_parameter!(ProcessingOperatorFrequencyFine, f32, OPERATOR_DEFAULT_FREQUENCY_FINE);

impl ParameterValueConversion for ProcessingOperatorFrequencyFine {
    type ProcessingValue = f32;

    fn to_processing(value: f32) -> Self::ProcessingValue {
        map_parameter_value_to_value_with_steps(&OPERATOR_FINE_STEPS, value)
    }
    fn to_sync(value: Self::ProcessingValue) -> f32 {
        map_value_to_parameter_value_with_steps(&OPERATOR_FINE_STEPS, value)
    }
    fn parse_string_value(value: String) -> Option<Self::ProcessingValue> {
        simple_parameter_string_parsing!(Self, value, Self::ProcessingValue)
    }

    fn format_processing(internal_value: Self::ProcessingValue) -> String {
        format!("{:.02}", internal_value)
    }
}


// Feedback

create_interpolatable_processing_parameter!(ProcessingOperatorFeedback, OPERATOR_DEFAULT_FEEDBACK);

impl_parameter_value_conversion_identity!(ProcessingOperatorFeedback);


// Modulation index

create_interpolatable_processing_parameter!(ProcessingOperatorModulationIndex, OPERATOR_DEFAULT_MODULATION_INDEX);

impl ParameterValueConversion for ProcessingOperatorModulationIndex {
    type ProcessingValue = f32;

    fn to_processing(value: f32) -> Self::ProcessingValue {
        map_parameter_value_to_value_with_steps(&OPERATOR_BETA_STEPS[..], value)
    }
    fn to_sync(value: Self::ProcessingValue) -> f32 {
        map_value_to_parameter_value_with_steps(&OPERATOR_BETA_STEPS[..], value)
    }
    fn parse_string_value(value: String) -> Option<Self::ProcessingValue> {
        simple_parameter_string_parsing!(Self, value, Self::ProcessingValue)
    }
    fn format_processing(internal_value: Self::ProcessingValue) -> String {
        format!("{:.02}", internal_value)
    }
}


// Wave type

create_simple_processing_parameter!(ProcessingOperatorWaveType, WaveType, OPERATOR_DEFAULT_WAVE_TYPE);

impl ParameterValueConversion for ProcessingOperatorWaveType {
    type ProcessingValue = WaveType;

    fn to_processing(value: f32) -> WaveType {
        if value <= 0.5 {
            WaveType::Sine
        }
        else {
            WaveType::WhiteNoise
        }
    }
    fn to_sync(value: WaveType) -> f32 {
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
    fn format_processing(internal_value: Self::ProcessingValue) -> String {
        match internal_value {
            WaveType::Sine => "Sine".to_string(),
            WaveType::WhiteNoise => "White noise".to_string(),
        }
    }
}


// Attack duration

create_simple_processing_parameter!(ProcessingOperatorAttackDuration, f32, DEFAULT_ENVELOPE_ATTACK_DURATION);
impl_envelope_duration_value_conversion!(ProcessingOperatorAttackDuration);


// Attack volume

create_simple_processing_parameter!(ProcessingOperatorAttackVolume, f32, DEFAULT_ENVELOPE_ATTACK_VOLUME);
impl_parameter_value_conversion_identity!(ProcessingOperatorAttackVolume);


// Decay duration

create_simple_processing_parameter!(ProcessingOperatorDecayDuration, f32, DEFAULT_ENVELOPE_DECAY_DURATION);
impl_envelope_duration_value_conversion!(ProcessingOperatorDecayDuration);


// Decay volume

create_simple_processing_parameter!(ProcessingOperatorDecayVolume, f32, DEFAULT_ENVELOPE_DECAY_VOLUME);
impl_parameter_value_conversion_identity!(ProcessingOperatorDecayVolume);


// Release duration

create_simple_processing_parameter!(ProcessingOperatorReleaseDuration, f32, DEFAULT_ENVELOPE_RELEASE_DURATION);
impl_envelope_duration_value_conversion!(ProcessingOperatorReleaseDuration);


// Modulation target

create_simple_processing_parameter!(ProcessingOperatorModulationTarget2, usize, OPERATOR_3_DEFAULT_MOD_TARGET);

impl ParameterValueConversion for ProcessingOperatorModulationTarget2 {
    type ProcessingValue = usize;

    fn to_processing(value: f32) -> Self::ProcessingValue {
        map_parameter_value_to_step(&[0, 1], value)
    }
    fn to_sync(value: Self::ProcessingValue) -> f32 {
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

    fn format_processing(internal_value: Self::ProcessingValue) -> String {
        format!("Operator {}", internal_value + 1)
    }
}


create_simple_processing_parameter!(ProcessingOperatorModulationTarget3, usize, OPERATOR_4_DEFAULT_MOD_TARGET);

impl ParameterValueConversion for ProcessingOperatorModulationTarget3 {
    type ProcessingValue = usize;

    fn to_processing(value: f32) -> Self::ProcessingValue {
        map_parameter_value_to_step(&[0, 1, 2], value)
    }
    fn to_sync(value: Self::ProcessingValue) -> f32 {
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

    fn format_processing(internal_value: Self::ProcessingValue) -> String {
        format!("Operator {}", internal_value + 1)
    }
}


#[derive(Debug)]
pub enum ProcessingOperatorModulationTarget {
    OperatorIndex2(ProcessingOperatorModulationTarget2),
    OperatorIndex3(ProcessingOperatorModulationTarget3),
}


impl ProcessingOperatorModulationTarget {
    pub fn opt_new(operator_index: usize) -> Option<Self> {
        match operator_index {
            2 => Some(ProcessingOperatorModulationTarget::OperatorIndex2(
                ProcessingOperatorModulationTarget2::default()
            )),
            3 => Some(ProcessingOperatorModulationTarget::OperatorIndex3(
                ProcessingOperatorModulationTarget3::default()
            )),
            _ => None
        }
    }
}


// Panning

#[derive(Debug, Clone)]
pub struct ProcessingOperatorPanning {
    value: TimeInterpolatableValue,
    pub left_and_right: [f32; 2],
}

impl ProcessingOperatorPanning {
    pub fn calculate_left_and_right(panning: f32) -> [f32; 2] {
        let pan_phase = panning * FRAC_PI_2;

        [pan_phase.cos(), pan_phase.sin()]
    }
}

impl ProcessingParameter for ProcessingOperatorPanning {
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

impl Default for ProcessingOperatorPanning {
    fn default() -> Self {
        let default = OPERATOR_DEFAULT_PANNING;

        Self {
            value: TimeInterpolatableValue::new(default),
            left_and_right: Self::calculate_left_and_right(default),
        }
    }
}

impl_parameter_value_conversion_identity!(ProcessingOperatorPanning);