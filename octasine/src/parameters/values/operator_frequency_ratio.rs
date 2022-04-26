use super::ParameterValue;
use crate::constants::*;
use crate::parameters::utils::*;

#[derive(Debug, Clone, Copy)]
pub struct OperatorFrequencyRatioValue(f64);

impl Default for OperatorFrequencyRatioValue {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_FREQUENCY_RATIO)
    }
}

impl ParameterValue for OperatorFrequencyRatioValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(&OPERATOR_RATIO_STEPS[..], sync))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(&OPERATOR_RATIO_STEPS[..], self.0)
    }
    fn format(self) -> String {
        format!("{:.04}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
    fn from_text(text: String) -> Option<Self> {
        text.parse::<f64>()
            .ok()
            .map(|value| Self(round_to_step(&OPERATOR_RATIO_STEPS[..], value)))
    }
}
