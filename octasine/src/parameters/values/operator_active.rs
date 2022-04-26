use super::ParameterValue;

#[derive(Debug, Clone, Copy)]
pub struct OperatorVolumeToggleValue(f64);

impl Default for OperatorVolumeToggleValue {
    fn default() -> Self {
        Self(1.0)
    }
}

impl ParameterValue for OperatorVolumeToggleValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value.round())
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(value: f64) -> Self {
        Self(value.round())
    }
    fn to_sync(self) -> f64 {
        self.0
    }
    fn format(self) -> String {
        if self.0 < 0.5 {
            "Off".into()
        } else {
            "On".into()
        }
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}
