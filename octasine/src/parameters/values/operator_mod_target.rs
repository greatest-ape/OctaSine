use super::ParameterValue;
use crate::common::*;
use crate::parameters::utils::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Operator2ModulationTargetValue(ModTargetStorage<1>);

impl ParameterValue for Operator2ModulationTargetValue {
    type Value = ModTargetStorage<1>;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(
            Self::Value::permutations(),
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(Self::Value::permutations(), self.0)
    }
    fn format(self) -> String {
        self.0.to_string()
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Operator3ModulationTargetValue(ModTargetStorage<2>);

impl ParameterValue for Operator3ModulationTargetValue {
    type Value = ModTargetStorage<2>;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(
            Self::Value::permutations(),
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(Self::Value::permutations(), self.0)
    }
    fn format(self) -> String {
        self.0.to_string()
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Operator4ModulationTargetValue(ModTargetStorage<3>);

impl ParameterValue for Operator4ModulationTargetValue {
    type Value = ModTargetStorage<3>;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(
            Self::Value::permutations(),
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(Self::Value::permutations(), self.0)
    }
    fn format(self) -> String {
        self.0.to_string()
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}
