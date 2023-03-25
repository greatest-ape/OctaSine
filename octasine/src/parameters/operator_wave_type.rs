use std::f32::consts::TAU;

use crate::common::*;

use super::{
    utils::{map_patch_value_to_step, map_step_to_patch_value},
    ParameterValue,
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
        Self(map_patch_value_to_step(OPERATOR_WAVEFORMS, value))
    }
    fn to_patch(self) -> f32 {
        map_step_to_patch_value(OPERATOR_WAVEFORMS, self.0)
    }
    fn get_formatted(self) -> String {
        match self.0 {
            WaveType::Sine => "SINE".to_string(),
            WaveType::Square => "SQUARE".to_string(),
            WaveType::Triangle => "TRIANGLE".to_string(),
            WaveType::Saw => "SAW".to_string(),
            WaveType::WhiteNoise => "NOISE".to_string(),
        }
    }
}
