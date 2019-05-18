use crate::common::*;
use crate::constants::*;

use crate::{interpolatable_parameter, simple_parameter};
use super::common::*;
use super::utils::*;


#[derive(Debug, Clone)]
pub struct MasterVolume {
    value: TimeInterpolatableValue,
}

interpolatable_parameter!(MasterVolume);

impl MasterVolume {
    pub fn new() -> Self {
        Self {
            value: TimeInterpolatableValue::new(1.0),
        }
    }
    pub fn get_full_parameter_name(&self) -> String {
        "Master volume".to_string()
    }
    pub fn from_parameter_value(&self, value: f64) -> f64 {
        value * 2.0
    }
    pub fn to_parameter_value(&self, value: f64) -> f64 {
        value / 2.0
    }
    pub fn parse_string_value(&self, value: String) -> Option<f64> {
        value.parse::<f64>().ok().map(|value| {
            let max = self.from_parameter_value(1.0);
            let min = self.from_parameter_value(0.0);

            value.max(min).min(max)
        })
    }
}


#[derive(Debug, Clone, Copy)]
pub struct MasterFrequency {
    pub value: f64
}

simple_parameter!(MasterFrequency);

impl MasterFrequency {
    pub fn get_full_parameter_name(&self) -> String {
        "Master frequency".to_string()
    }

    pub fn from_parameter_value(&self, value: f64) -> f64 {
        map_parameter_value_to_value_with_steps(&MASTER_FREQUENCY_STEPS, value)
    }
    pub fn to_parameter_value(&self, value: f64) -> f64 {
        map_value_to_parameter_value_with_steps(&MASTER_FREQUENCY_STEPS, value)
    }
    pub fn parse_string_value(&self, value: String) -> Option<f64> {
        value.parse::<f64>().ok().map(|value| {
            let max = self.from_parameter_value(1.0);
            let min = self.from_parameter_value(0.0);

            value.max(min).min(max)
        })
    }
}