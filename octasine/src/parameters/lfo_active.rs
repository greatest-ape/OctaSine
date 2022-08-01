use super::ParameterValue;

#[derive(Debug, Clone, Copy)]
pub struct LfoActiveValue(f32);

impl Default for LfoActiveValue {
    fn default() -> Self {
        Self(1.0)
    }
}

impl ParameterValue for LfoActiveValue {
    type Value = f32;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value.round())
    }
    fn new_from_text(text: String) -> Option<Self> {
        match text.trim().to_lowercase().as_str() {
            "on" | "active" => Some(Self(1.0)),
            "off" | "inactive" => Some(Self(0.0)),
            _ => None,
        }
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(value.round())
    }
    fn to_patch(self) -> f32 {
        self.0
    }
    fn get_formatted(self) -> String {
        if self.0 < 0.5 {
            "Off".into()
        } else {
            "On".into()
        }
    }
}
