use super::ParameterValue;
use crate::parameters::utils::*;

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

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(
            &LFO_FREQUENCY_FREE_STEPS,
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_value_to_parameter_value_with_steps(&LFO_FREQUENCY_FREE_STEPS, self.0)
    }
    fn format(self) -> String {
        format!("{:.04}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
    fn from_text(_: String) -> Option<Self> {
        None // FIXME
    }
}
