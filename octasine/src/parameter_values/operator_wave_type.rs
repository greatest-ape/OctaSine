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
    fn calculate(self, phase: Phase) -> f64 {
        match self {
            Self::Sine => crate::parameter_values::lfo_shape::sine(phase),
            Self::WhiteNoise => {
                // Ensure same numbers are generated each time for GUI
                // consistency. This will however break if fastrand changes
                // its algorithm.
                let seed = phase.0.to_bits() + 2;

                (fastrand::Rng::with_seed(seed).f64() - 0.5) * 2.0
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

    fn from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_patch(sync: f64) -> Self {
        if sync <= 0.5 {
            Self(WaveType::Sine)
        } else {
            Self(WaveType::WhiteNoise)
        }
    }
    fn to_patch(self) -> f64 {
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
    fn convert_patch_to_audio_formatted(value: f64) -> String {
        Self::from_patch(value).get_formatted()
    }
    fn from_text(text: String) -> Option<Self> {
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
