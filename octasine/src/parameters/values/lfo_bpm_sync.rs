use super::ParameterValue;

#[derive(Debug, Clone, Copy)]
pub struct LfoBpmSyncValue(pub bool);

impl Default for LfoBpmSyncValue {
    fn default() -> Self {
        Self(true)
    }
}

impl ParameterValue for LfoBpmSyncValue {
    type Value = bool;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(sync <= 0.5)
    }
    fn to_sync(self) -> f64 {
        if self.0 {
            0.0
        } else {
            1.0
        }
    }
    fn format(self) -> String {
        if self.0 {
            "On".to_string()
        } else {
            "Off".to_string()
        }
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
    fn from_text(text: String) -> Option<Self> {
        match text.to_lowercase().as_ref() {
            "true" | "on" => Some(Self(true)),
            "false" | "off" => Some(Self(false)),
            _ => None,
        }
    }
}
