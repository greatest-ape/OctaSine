use super::utils::*;
use super::ParameterValue;

pub const OPERATOR_RATIO_STEPS: [f64; 28] = [
    0.0625, 0.125, 0.2, 0.25, 0.33, 0.5, 0.55, 0.66, 0.6896, 0.8, 0.8333, 1.0, 1.2, 1.25, 1.33,
    1.45, 1.5, 1.8, 1.875, 2.0, 2.5, 3.0, 3.5, 4.0, 8.0, 16.0, 32.0, 64.0,
];

#[derive(Debug, Clone, Copy)]
pub struct OperatorFrequencyRatioValue(f64);

impl Default for OperatorFrequencyRatioValue {
    fn default() -> Self {
        Self(1.0)
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
