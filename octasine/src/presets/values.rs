use vst2_helpers::processing_parameters::utils::*;

use crate::constants::*;


pub trait ProcessingValueConversion {
    fn from_sync(value: f64) -> Self;
    fn to_sync(self) -> f64;
    fn format(self) -> String;
    fn format_sync(value: f64) -> String;
}


#[derive(Debug, Clone, Copy)]
pub enum ProcessingValue {
    MasterVolume(MasterVolume),
    MasterFrequency(MasterFrequency),
    OperatorVolume(OperatorVolume),
    OperatorAdditive(OperatorAdditive),
    OperatorFrequencyRatio(OperatorFrequencyRatio),
    OperatorFrequencyFree(OperatorFrequencyFree),
    OperatorFrequencyFine(OperatorFrequencyFine),
    OperatorFeedback(OperatorFeedback),
    OperatorModulationIndex(OperatorModulationIndex),
}


#[derive(Debug, Clone, Copy)]
pub struct MasterVolume(pub f64);


impl Default for MasterVolume {
    fn default() -> Self {
        Self(DEFAULT_MASTER_VOLUME)
    }
}


impl ProcessingValueConversion for MasterVolume {
    fn from_sync(value: f64) -> Self {
        Self(value * 2.0)
    }
    fn to_sync(self) -> f64 {
        self.0 / 2.0
    }
    fn format(self) -> String {
        format!("{:.2}", 20.0 * self.0.log10())
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct MasterFrequency(pub f64);


impl Default for MasterFrequency {
    fn default() -> Self {
        Self(DEFAULT_MASTER_FREQUENCY)
    }
}


impl ProcessingValueConversion for MasterFrequency {
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(
            &MASTER_FREQUENCY_STEPS,
            sync
        ))
    }
    fn to_sync(self) -> f64 {
        map_value_to_parameter_value_with_steps(&MASTER_FREQUENCY_STEPS, self.0)
    }
    fn format(self) -> String {
        format!("{:.02}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct OperatorVolume(pub f64);


impl OperatorVolume {
    pub fn new(index: usize) -> Self {
        if index == 0 {
            Self(DEFAULT_OPERATOR_VOLUME)
        } else {
            Self(0.0)
        }
    }
}


impl ProcessingValueConversion for OperatorVolume {
    fn from_sync(sync: f64) -> Self {
        Self(sync * 2.0)
    }
    fn to_sync(self) -> f64 {
        self.0 / 2.0
    }
    fn format(self) -> String {
        format!("{:.02}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct OperatorAdditive(pub f64);


impl Default for OperatorAdditive {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_ADDITIVE_FACTOR)
    }
}


impl ProcessingValueConversion for OperatorAdditive {
    fn from_sync(sync: f64) -> Self {
        Self(sync)
    }
    fn to_sync(self) -> f64 {
        self.0
    }
    fn format(self) -> String {
        format!("{:.02}%", (self.0 * 100.0))
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct OperatorFrequencyRatio(pub f64);


impl Default for OperatorFrequencyRatio {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_FREQUENCY_RATIO)
    }
}


impl ProcessingValueConversion for OperatorFrequencyRatio {
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(&OPERATOR_RATIO_STEPS[..], sync))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(&OPERATOR_RATIO_STEPS[..], self.0)
    }
    fn format(self) -> String {
        format!("{:.02}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct OperatorFrequencyFree(pub f64);


impl Default for OperatorFrequencyFree {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_FREQUENCY_FREE)
    }
}


impl ProcessingValueConversion for OperatorFrequencyFree {
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(&OPERATOR_FREE_STEPS, sync))
    }
    fn to_sync(self) -> f64 {
        map_value_to_parameter_value_with_steps(&OPERATOR_FREE_STEPS, self.0)
    }
    fn format(self) -> String {
        format!("{:.02}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct OperatorFrequencyFine(pub f64);


impl Default for OperatorFrequencyFine {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_FREQUENCY_FINE)
    }
}


impl ProcessingValueConversion for OperatorFrequencyFine {
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(&OPERATOR_FINE_STEPS, sync))
    }
    fn to_sync(self) -> f64 {
        map_value_to_parameter_value_with_steps(&OPERATOR_FINE_STEPS, self.0)
    }
    fn format(self) -> String {
        format!("{:.02}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct OperatorFeedback(pub f64);


impl Default for OperatorFeedback {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_FEEDBACK)
    }
}


impl ProcessingValueConversion for OperatorFeedback {
    fn from_sync(sync: f64) -> Self {
        Self(sync)
    }
    fn to_sync(self) -> f64 {
        self.0
    }
    fn format(self) -> String {
        format!("{:.02}%", (self.0 * 100.0))
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct OperatorModulationIndex(pub f64);


impl Default for OperatorModulationIndex {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_MODULATION_INDEX)
    }
}


impl ProcessingValueConversion for OperatorModulationIndex {
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(&OPERATOR_BETA_STEPS[..], sync))
    }
    fn to_sync(self) -> f64 {
        map_value_to_parameter_value_with_steps(&OPERATOR_BETA_STEPS[..], self.0)
    }
    fn format(self) -> String {
        format!("{:.02}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}