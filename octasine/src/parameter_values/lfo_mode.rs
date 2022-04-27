use super::utils::*;
use super::ParameterValue;

const LFO_MODE_STEPS: [LfoMode; 2] = [LfoMode::Forever, LfoMode::Once];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LfoMode {
    Once,
    Forever,
}

impl Default for LfoMode {
    fn default() -> Self {
        Self::Forever
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LfoModeValue(pub LfoMode);

impl ParameterValue for LfoModeValue {
    type Value = LfoMode;

    fn from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_patch(sync: f64) -> Self {
        Self(map_parameter_value_to_step(&LFO_MODE_STEPS[..], sync))
    }
    fn to_patch(self) -> f64 {
        map_step_to_parameter_value(&LFO_MODE_STEPS[..], self.0)
    }
    fn get_formatted(self) -> String {
        match self.0 {
            LfoMode::Once => "ONCE".to_string(),
            LfoMode::Forever => "LOOP".to_string(),
        }
    }
    fn convert_patch_to_audio_formatted(value: f64) -> String {
        Self::from_patch(value).get_formatted()
    }
    fn from_text(text: String) -> Option<Self> {
        match text.to_lowercase().as_ref() {
            "once" => Some(Self(LfoMode::Once)),
            "forever" => Some(Self(LfoMode::Forever)),
            _ => None,
        }
    }
}
