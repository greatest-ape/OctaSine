use std::f32::consts::TAU;

use compact_str::CompactString;

use crate::common::*;

use super::{
    utils::{map_patch_value_to_step, map_step_to_patch_value},
    {ParameterValue, SerializableRepresentation},
};

const OPERATOR_WAVEFORMS: &[WaveType] = &[
    WaveType::Sine,
    WaveType::Square,
    WaveType::Triangle,
    WaveType::Saw,
    WaveType::WhiteNoise,
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum WaveType {
    #[default]
    Sine,
    Square,
    Triangle,
    Saw,
    WhiteNoise,
}

impl WaveformChoices for WaveType {
    fn calculate_for_current(self, phase: Phase) -> f32 {
        match self {
            Self::Sine => ::sleef_trig::Sleef_sinf1_u35purec_range125(phase.0 as f32 * TAU),
            Self::Saw => crate::math::wave::saw(phase.0) as f32,
            Self::Triangle => crate::math::wave::triangle(phase.0) as f32,
            Self::Square => crate::math::wave::square(phase.0) as f32,
            Self::WhiteNoise => {
                // Ensure same numbers are generated each time for GUI
                // consistency. This will however break if fastrand changes
                // its algorithm.
                let seed = phase.0.to_bits() + 2;

                // Generate f64 because that exact value looks nice
                ((fastrand::Rng::with_seed(seed).f64() - 0.5) * 2.0) as f32
            }
        }
    }
    fn choices() -> &'static [Self] {
        OPERATOR_WAVEFORMS
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct OperatorWaveTypeValue(pub WaveType);

impl ParameterValue for OperatorWaveTypeValue {
    type Value = WaveType;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: &str) -> Option<Self> {
        match text.to_lowercase().trim() {
            "sine" => Some(Self(WaveType::Sine)),
            "square" => Some(Self(WaveType::Square)),
            "triangle" => Some(Self(WaveType::Triangle)),
            "saw" => Some(Self(WaveType::Saw)),
            "noise" => Some(Self(WaveType::WhiteNoise)),
            _ => None,
        }
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(map_patch_value_to_step(OPERATOR_WAVEFORMS, value))
    }
    fn to_patch(self) -> f32 {
        map_step_to_patch_value(OPERATOR_WAVEFORMS, self.0)
    }
    fn get_formatted(self) -> CompactString {
        match self.0 {
            WaveType::Sine => "SINE".into(),
            WaveType::Square => "SQUARE".into(),
            WaveType::Triangle => "TRIANGLE".into(),
            WaveType::Saw => "SAW".into(),
            WaveType::WhiteNoise => "NOISE".into(),
        }
    }

    fn get_serializable(&self) -> SerializableRepresentation {
        SerializableRepresentation::Other(self.get_formatted())
    }
}
