use crate::common::*;

use super::ParameterValue;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WaveType {
    Sine,
    WhiteNoise,
}

impl Default for WaveType {
    fn default() -> Self {
        Self::Sine
    }
}
impl CalculateCurve for WaveType {
    fn calculate(self, phase: Phase) -> f32 {
        match self {
            Self::Sine => crate::parameters::lfo_shape::sine(phase),
            Self::WhiteNoise => {
                // Ensure same numbers are generated each time for GUI
                // consistency. This will however break if fastrand changes
                // its algorithm.
                let seed = phase.0.to_bits() + 2;

                (fastrand::Rng::with_seed(seed).f32() - 0.5) * 2.0
            }
        }
    }
    fn steps() -> &'static [Self] {
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
    fn get_formatted(self) -> String {
        match self.0 {
            WaveType::Sine => "SINE".to_string(),
            WaveType::WhiteNoise => "NOISE".to_string(),
        }
    }
    fn new_from_text(text: String) -> Option<Self> {
        let value = text.to_lowercase();

        if value.contains("sin") {
            Some(OperatorWaveTypeValue(WaveType::Sine))
        } else if value.contains("noise") {
            Some(OperatorWaveTypeValue(WaveType::WhiteNoise))
        } else {
            None
        }
    }
}
