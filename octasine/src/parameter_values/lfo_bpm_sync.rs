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

    fn from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_patch(sync: f64) -> Self {
        Self(sync <= 0.5)
    }
    fn to_patch(self) -> f64 {
        if self.0 {
            0.0
        } else {
            1.0
        }
    }
    fn get_formatted(self) -> String {
        if self.0 {
            "On".to_string()
        } else {
            "Off".to_string()
        }
    }
    fn convert_patch_to_audio_formatted(value: f64) -> String {
        Self::from_patch(value).get_formatted()
    }
    fn from_text(text: String) -> Option<Self> {
        match text.to_lowercase().as_ref() {
            "true" | "on" => Some(Self(true)),
            "false" | "off" => Some(Self(false)),
            _ => None,
        }
    }
}
