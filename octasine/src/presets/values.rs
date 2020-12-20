use vst2_helpers::processing_parameters::utils::*;

use crate::constants::*;


pub trait ProcessingValueConversion {
    fn from_sync(value: f64) -> Self;
    fn to_sync(self) -> f64;
    fn format(self) -> String;
    fn format_sync(value: f64) -> String;
}


#[derive(Debug, Clone, Copy)]
pub struct MasterVolume(pub f64);


impl Default for MasterVolume {
    fn default() -> Self {
        Self(DEFAULT_MASTER_VOLUME)
    }
}


impl ProcessingValueConversion for MasterVolume {
    fn from_sync(value: f64) -> Self {
        Self(value * 2.0)
    }
    fn to_sync(self) -> f64 {
        self.0 / 2.0
    }
    fn format(self) -> String {
        format!("{:.2}", 20.0 * self.0.log10())
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct MasterFrequency(pub f64);


impl Default for MasterFrequency {
    fn default() -> Self {
        Self(DEFAULT_MASTER_FREQUENCY)
    }
}


impl ProcessingValueConversion for MasterFrequency {
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(
            &MASTER_FREQUENCY_STEPS,
            sync
        ))
    }
    fn to_sync(self) -> f64 {
        map_value_to_parameter_value_with_steps(&MASTER_FREQUENCY_STEPS, self.0)
    }
    fn format(self) -> String {
        format!("{:.02}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
pub enum ProcessingValue {
    MasterVolume(MasterVolume),
    MasterFrequency(MasterFrequency),
}