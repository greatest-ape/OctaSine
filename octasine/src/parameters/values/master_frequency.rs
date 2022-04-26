use super::ParameterValue;
use crate::constants::*;
use crate::parameters::utils::*;

#[derive(Debug, Clone, Copy)]
pub struct MasterFrequencyValue(f64);

impl Default for MasterFrequencyValue {
    fn default() -> Self {
        Self(DEFAULT_MASTER_FREQUENCY)
    }
}

impl ParameterValue for MasterFrequencyValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(
            &MASTER_FREQUENCY_STEPS,
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_value_to_parameter_value_with_steps(&MASTER_FREQUENCY_STEPS, self.0)
    }
    fn format(self) -> String {
        if self.0 < 10000.0 {
            format!("{:.02} Hz", self.0)
        } else {
            format!("{:.02}", self.0)
        }
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}
