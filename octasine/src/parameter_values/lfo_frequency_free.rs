use super::utils::*;
use super::ParameterValue;

const LFO_FREQUENCY_FREE_STEPS: [f64; 7] = [1.0 / 16.0, 0.5, 0.9, 1.0, 1.1, 2.0, 16.0];

#[derive(Debug, Clone, Copy)]
pub struct LfoFrequencyFreeValue(pub f64);

impl Default for LfoFrequencyFreeValue {
    fn default() -> Self {
        Self(1.0)
    }
}

impl ParameterValue for LfoFrequencyFreeValue {
    type Value = f64;

    fn from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_patch(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(
            &LFO_FREQUENCY_FREE_STEPS,
            sync,
        ))
    }
    fn to_patch(self) -> f64 {
        map_value_to_parameter_value_with_steps(&LFO_FREQUENCY_FREE_STEPS, self.0)
    }
    fn get_formatted(self) -> String {
        format!("{:.04}", self.0)
    }
}
