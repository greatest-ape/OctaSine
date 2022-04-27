use std::f64::consts::TAU;

use super::utils::*;
use super::ParameterValue;
use crate::common::*;

pub const LFO_SHAPE_STEPS: [LfoShape; 8] = [
    LfoShape::Triangle,
    LfoShape::ReverseTriangle,
    LfoShape::Saw,
    LfoShape::ReverseSaw,
    LfoShape::Square,
    LfoShape::ReverseSquare,
    LfoShape::Sine,
    LfoShape::ReverseSine,
];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LfoShape {
    Saw,
    ReverseSaw,
    Triangle,
    ReverseTriangle,
    Square,
    ReverseSquare,
    Sine,
    ReverseSine,
}

impl Default for LfoShape {
    fn default() -> Self {
        Self::Triangle
    }
}

impl CalculateCurve for LfoShape {
    fn calculate(self, phase: Phase) -> f64 {
        match self {
            Self::Saw => saw(phase),
            Self::ReverseSaw => -saw(phase),
            Self::Triangle => triangle(phase),
            Self::ReverseTriangle => -triangle(phase),
            Self::Square => square(phase),
            Self::ReverseSquare => -square(phase),
            Self::Sine => sine(phase),
            Self::ReverseSine => -sine(phase),
        }
    }
    fn steps() -> &'static [Self] {
        &LFO_SHAPE_STEPS
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LfoShapeValue(pub LfoShape);

impl ParameterValue for LfoShapeValue {
    type Value = LfoShape;

    fn from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_patch(sync: f64) -> Self {
        Self(map_parameter_value_to_step(&LFO_SHAPE_STEPS[..], sync))
    }
    fn to_patch(self) -> f64 {
        map_step_to_parameter_value(&LFO_SHAPE_STEPS[..], self.0)
    }
    fn get_formatted(self) -> String {
        match self.0 {
            LfoShape::Triangle => "TRIANGLE".to_string(),
            LfoShape::ReverseTriangle => "REV TRNG".to_string(),
            LfoShape::Saw => "SAW".to_string(),
            LfoShape::ReverseSaw => "REV SAW".to_string(),
            LfoShape::Square => "SQUARE".to_string(),
            LfoShape::ReverseSquare => "REV SQR".to_string(),
            LfoShape::Sine => "SINE".to_string(),
            LfoShape::ReverseSine => "REV SINE".to_string(),
        }
    }
    fn from_text(text: String) -> Option<Self> {
        match text.to_lowercase().as_ref() {
            "triangle" => Some(Self(LfoShape::Triangle)),
            "reverse triangle" => Some(Self(LfoShape::ReverseTriangle)),
            "saw" => Some(Self(LfoShape::Saw)),
            "reverse saw" => Some(Self(LfoShape::ReverseSaw)),
            "square" => Some(Self(LfoShape::Square)),
            "reverse square" => Some(Self(LfoShape::ReverseSquare)),
            _ => None,
        }
    }
}

fn triangle(phase: Phase) -> f64 {
    if phase.0 <= 0.25 {
        4.0 * phase.0
    } else if phase.0 <= 0.75 {
        1.0 - 4.0 * (phase.0 - 0.25)
    } else {
        -1.0 + 4.0 * (phase.0 - 0.75)
    }
}

fn saw(phase: Phase) -> f64 {
    (phase.0 - 0.5) * 2.0
}

fn square(phase: Phase) -> f64 {
    let peak_end = 32.0 / 64.0;
    let base_start = 33.0 / 64.0;

    if phase.0 <= peak_end {
        1.0
    } else if phase.0 <= base_start {
        1.0 - 2.0 * ((phase.0 - peak_end) / (base_start - peak_end))
    } else {
        -1.0
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "simd")] {
        pub fn sine(phase: Phase) -> f64 {
            unsafe {
                ::sleef_sys::Sleef_cinz_sind1_u35purec(phase.0 * TAU)
            }
        }
    } else {
        pub fn sine(phase: Phase) -> f64 {
            (phase.0 * TAU).sin()
        }
    }
}
