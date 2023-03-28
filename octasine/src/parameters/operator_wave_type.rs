use std::f32::consts::TAU;

use compact_str::CompactString;

use crate::common::*;

use super::ParameterValue;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum WaveType {
    #[default]
    Sine,
    WhiteNoise,
}

impl WaveformChoices for WaveType {
    fn calculate_for_current(self, phase: Phase) -> f32 {
        match self {
            Self::Sine => ::sleef_trig::Sleef_sinf1_u35purec_range125(phase.0 as f32 * TAU),
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
        &[Self::Sine, Self::WhiteNoise]
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
        let value = text.to_lowercase();

        if value.contains("sin") {
            Some(OperatorWaveTypeValue(WaveType::Sine))
        } else if value.contains("noise") {
            Some(OperatorWaveTypeValue(WaveType::WhiteNoise))
        } else {
            None
        }
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        if value <= 0.5 {
            Self(WaveType::Sine)
        } else {
            Self(WaveType::WhiteNoise)
        }
    }
    fn to_patch(self) -> f32 {
        match self.0 {
            WaveType::Sine => 0.0,
            WaveType::WhiteNoise => 1.0,
        }
    }
    fn get_formatted(self) -> CompactString {
        match self.0 {
            WaveType::Sine => "SINE".into(),
            WaveType::WhiteNoise => "NOISE".into(),
        }
    }
}
