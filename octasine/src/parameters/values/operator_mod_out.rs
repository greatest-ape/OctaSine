use super::ParameterValue;
use crate::constants::*;
use crate::parameters::utils::*;

#[derive(Debug, Clone, Copy)]
pub struct OperatorModulationIndexValue(f64);

impl Default for OperatorModulationIndexValue {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_MODULATION_INDEX)
    }
}

impl ParameterValue for OperatorModulationIndexValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(
            &OPERATOR_BETA_STEPS[..],
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_value_to_parameter_value_with_steps(&OPERATOR_BETA_STEPS[..], self.0)
    }
    fn format(self) -> String {
        format!("{:.04}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}
