use super::utils::*;
use super::ParameterValue;

pub const OPERATOR_FREE_STEPS: [f64; 14] = [
    0.001, 0.0625, 0.125, 0.25, 0.5, 0.75, 1.0, 1.5, 2.0, 3.0, 4.0, 16.0, 64.0, 256.0,
];

#[derive(Debug, Clone, Copy)]
pub struct OperatorFrequencyFreeValue(f64);

impl Default for OperatorFrequencyFreeValue {
    fn default() -> Self {
        Self(1.0)
    }
}

impl ParameterValue for OperatorFrequencyFreeValue {
    type Value = f64;

    fn from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_patch(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(
            &OPERATOR_FREE_STEPS,
            sync,
        ))
    }
    fn to_patch(self) -> f64 {
        map_value_to_parameter_value_with_steps(&OPERATOR_FREE_STEPS, self.0)
    }
    fn get_formatted(self) -> String {
        format!("{:.04}", self.0)
    }
}
