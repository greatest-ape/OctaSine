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

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: &str) -> Option<Self> {
        match text.to_lowercase().as_ref() {
            "true" | "on" => Some(Self(true)),
            "false" | "off" => Some(Self(false)),
            _ => None,
        }
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(value <= 0.5)
    }
    fn to_patch(self) -> f32 {
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
}
