use super::ParameterValue;
use crate::constants::*;

#[derive(Debug, Clone, Copy)]
pub struct OperatorMixValue(f64);

impl OperatorMixValue {
    pub fn new(index: usize) -> Self {
        if index == 0 {
            Self(DEFAULT_OPERATOR_VOLUME)
        } else {
            Self(0.0)
        }
    }
}

impl ParameterValue for OperatorMixValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(sync * 2.0)
    }
    fn to_sync(self) -> f64 {
        self.0 / 2.0
    }
    fn format(self) -> String {
        format!("{:.04}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
    fn from_text(text: String) -> Option<Self> {
        text.parse::<f64>().map(|v| Self(v.max(0.0).min(2.0))).ok()
    }
}
