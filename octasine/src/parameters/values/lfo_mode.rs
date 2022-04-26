use super::ParameterValue;
use crate::common::*;
use crate::constants::*;
use crate::parameters::utils::*;

#[derive(Debug, Clone, Copy)]
pub struct LfoModeValue(pub LfoMode);

impl Default for LfoModeValue {
    fn default() -> Self {
        Self(DEFAULT_LFO_MODE)
    }
}

impl ParameterValue for LfoModeValue {
    type Value = LfoMode;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(&LFO_MODE_STEPS[..], sync))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(&LFO_MODE_STEPS[..], self.0)
    }
    fn format(self) -> String {
        match self.0 {
            LfoMode::Once => "ONCE".to_string(),
            LfoMode::Forever => "LOOP".to_string(),
        }
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
    fn from_text(text: String) -> Option<Self> {
        match text.to_lowercase().as_ref() {
            "once" => Some(Self(LfoMode::Once)),
            "forever" => Some(Self(LfoMode::Forever)),
            _ => None,
        }
    }
}
