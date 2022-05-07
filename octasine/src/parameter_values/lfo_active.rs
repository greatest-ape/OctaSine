use super::ParameterValue;

#[derive(Debug, Clone, Copy)]
pub struct LfoActiveValue(f64);

impl Default for LfoActiveValue {
    fn default() -> Self {
        Self(1.0)
    }
}

impl ParameterValue for LfoActiveValue {
    type Value = f64;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value.round())
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f64) -> Self {
        Self(value.round())
    }
    fn to_patch(self) -> f64 {
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
