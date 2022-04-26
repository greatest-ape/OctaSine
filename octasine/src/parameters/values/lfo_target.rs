use super::ParameterValue;
use crate::common::*;
use crate::parameters::utils::*;

#[derive(Debug, Clone, Copy)]
pub struct Lfo1TargetParameterValue(pub LfoTargetParameter);

impl Default for Lfo1TargetParameterValue {
    fn default() -> Self {
        Self(LfoTargetParameter::Master(LfoTargetMasterParameter::Volume))
    }
}

impl ParameterValue for Lfo1TargetParameterValue {
    type Value = LfoTargetParameter;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(
            get_lfo_target_parameters(0),
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(get_lfo_target_parameters(0), self.0)
    }
    fn format(self) -> String {
        self.0.to_string()
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lfo2TargetParameterValue(pub LfoTargetParameter);

impl Default for Lfo2TargetParameterValue {
    fn default() -> Self {
        Self(LfoTargetParameter::Master(LfoTargetMasterParameter::Volume))
    }
}

impl ParameterValue for Lfo2TargetParameterValue {
    type Value = LfoTargetParameter;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(
            get_lfo_target_parameters(1),
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(get_lfo_target_parameters(1), self.0)
    }
    fn format(self) -> String {
        self.0.to_string()
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lfo3TargetParameterValue(pub LfoTargetParameter);

impl Default for Lfo3TargetParameterValue {
    fn default() -> Self {
        Self(LfoTargetParameter::Master(LfoTargetMasterParameter::Volume))
    }
}

impl ParameterValue for Lfo3TargetParameterValue {
    type Value = LfoTargetParameter;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(
            get_lfo_target_parameters(2),
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(get_lfo_target_parameters(2), self.0)
    }
    fn format(self) -> String {
        self.0.to_string()
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lfo4TargetParameterValue(pub LfoTargetParameter);

impl Default for Lfo4TargetParameterValue {
    fn default() -> Self {
        Self(LfoTargetParameter::Master(LfoTargetMasterParameter::Volume))
    }
}

impl ParameterValue for Lfo4TargetParameterValue {
    type Value = LfoTargetParameter;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(
            get_lfo_target_parameters(3),
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(get_lfo_target_parameters(3), self.0)
    }
    fn format(self) -> String {
        self.0.to_string()
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}
