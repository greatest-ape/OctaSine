use crate::common::*;
use crate::constants::*;

use crate::{
    impl_parameter_value_access_interpolatable,
    impl_parameter_value_access_simple,
    impl_parameter_string_parsing_simple,
    impl_get_value_for_interpolatable_parameter,
};

use super::common::*;
use super::utils::*;


// Master volume

#[derive(Debug, Clone)]
pub struct MasterVolume {
    value: TimeInterpolatableValue,
}

impl_parameter_value_access_interpolatable!(MasterVolume);
impl_parameter_string_parsing_simple!(MasterVolume);
impl_get_value_for_interpolatable_parameter!(MasterVolume);

impl MasterVolume {
    pub fn new() -> Self {
        Self {
            value: TimeInterpolatableValue::new(1.0),
        }
    }
}

impl ParameterValueConversion<f64> for MasterVolume {
    fn from_parameter_value(&self, value: f64) -> f64 {
        value * 2.0
    }
    fn to_parameter_value(&self, value: f64) -> f64 {
        value / 2.0
    }
}

impl ParameterGetName for MasterVolume {
    fn get_parameter_name(&self) -> String {
        "Master volume".to_string()
    }
}

impl ParameterGetUnit for MasterVolume {}


// Master frequency

#[derive(Debug, Clone, Copy)]
pub struct MasterFrequency {
    pub value: f64
}

impl_parameter_value_access_simple!(MasterFrequency);
impl_parameter_string_parsing_simple!(MasterFrequency);

impl ParameterValueConversion<f64> for MasterFrequency {
    fn from_parameter_value(&self, value: f64) -> f64 {
        map_parameter_value_to_value_with_steps(&MASTER_FREQUENCY_STEPS, value)
    }
    fn to_parameter_value(&self, value: f64) -> f64 {
        map_value_to_parameter_value_with_steps(&MASTER_FREQUENCY_STEPS, value)
    }
}

impl ParameterGetName for MasterFrequency {
    fn get_parameter_name(&self) -> String {
        "Master frequency".to_string()
    }
}

impl ParameterGetUnit for MasterFrequency {
    fn get_parameter_unit_of_measurement(&self) -> String {
        "Hz".to_string()
    }
}