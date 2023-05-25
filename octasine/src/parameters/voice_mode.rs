use compact_str::CompactString;

use super::{
    utils::{map_patch_value_to_step, map_step_to_patch_value},
    ParameterValue, SerializableRepresentation,
};

const STEPS: &[VoiceMode] = &[VoiceMode::Polyphonic, VoiceMode::Monophonic];

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum VoiceMode {
    #[default]
    Polyphonic,
    Monophonic,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct VoiceModeValue(VoiceMode);

impl ParameterValue for VoiceModeValue {
    type Value = VoiceMode;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: &str) -> Option<Self> {
        let text = text.to_lowercase();

        if text.contains("poly") {
            Some(Self(VoiceMode::Polyphonic))
        } else if text.contains("mono") {
            Some(Self(VoiceMode::Monophonic))
        } else {
            None
        }
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(map_patch_value_to_step(&STEPS[..], value))
    }
    fn to_patch(self) -> f32 {
        map_step_to_patch_value(&STEPS[..], self.0)
    }
    fn get_formatted(self) -> CompactString {
        match self.0 {
            VoiceMode::Polyphonic => "POLY".into(),
            VoiceMode::Monophonic => "MONO".into(),
        }
    }

    fn get_serializable(&self) -> SerializableRepresentation {
        SerializableRepresentation::Other(self.get_formatted())
    }
}
