use super::utils::*;
use super::ParameterValue;

pub const OPERATOR_FINE_STEPS: [f64; 17] = [
    0.8, 0.85, 0.9, 0.95, 0.97, 0.98, 0.99, 0.995, 1.0, 1.005, 1.01, 1.02, 1.03, 1.05, 1.1, 1.15,
    1.2,
];

#[derive(Debug, Clone, Copy)]
pub struct OperatorFrequencyFineValue(f64);

impl Default for OperatorFrequencyFineValue {
    fn default() -> Self {
        Self(1.0)
    }
}

impl ParameterValue for OperatorFrequencyFineValue {
    type Value = f64;

    fn from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_patch(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(
            &OPERATOR_FINE_STEPS,
            sync,
        ))
    }
    fn to_patch(self) -> f64 {
        map_value_to_parameter_value_with_steps(&OPERATOR_FINE_STEPS, self.0)
    }
    fn get_formatted(self) -> String {
        format!("{:.04}", self.0)
    }
}
