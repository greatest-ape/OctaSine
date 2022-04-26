use super::ParameterValue;

#[derive(Debug, Clone, Copy)]
pub struct OperatorVolumeValue(f64);

impl Default for OperatorVolumeValue {
    fn default() -> Self {
        Self(1.0)
    }
}

impl ParameterValue for OperatorVolumeValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(value: f64) -> Self {
        Self(value * 2.0)
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
}
