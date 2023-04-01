use std::f32::consts::TAU;

use compact_str::CompactString;

use super::utils::*;
use super::ParameterValue;
use super::SerializableRepresentation;
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum LfoShape {
    Saw,
    ReverseSaw,
    #[default]
    Triangle,
    ReverseTriangle,
    Square,
    ReverseSquare,
    Sine,
    ReverseSine,
}

impl LfoShape {
    pub fn calculate(self, phase: Phase) -> f32 {
        match self {
            Self::Saw => lfo_saw(phase),
            Self::ReverseSaw => -lfo_saw(phase),
            Self::Triangle => lfo_triangle(phase),
            Self::ReverseTriangle => -lfo_triangle(phase),
            Self::Square => lfo_square(phase),
            Self::ReverseSquare => -lfo_square(phase),
            Self::Sine => lfo_sine(phase),
            Self::ReverseSine => -lfo_sine(phase),
        }
    }
}

impl WaveformChoices for LfoShape {
    fn calculate_for_current(self, phase: Phase) -> f32 {
        LfoShape::calculate(self, phase)
    }
    fn choices() -> &'static [Self] {
        &LFO_SHAPE_STEPS
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LfoShapeValue(pub LfoShape);

impl ParameterValue for LfoShapeValue {
    type Value = LfoShape;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: &str) -> Option<Self> {
        match text.trim().to_lowercase().as_ref() {
            "triangle" | "trng" => Some(Self(LfoShape::Triangle)),
            "reverse triangle" | "rev triangle" | "rev trng" => {
                Some(Self(LfoShape::ReverseTriangle))
            }
            "saw" => Some(Self(LfoShape::Saw)),
            "reverse saw" | "rev saw" => Some(Self(LfoShape::ReverseSaw)),
            "square" | "sqr" => Some(Self(LfoShape::Square)),
            "reverse square" | "rev square" | "rev sqr" => Some(Self(LfoShape::ReverseSquare)),
            "sine" => Some(Self(LfoShape::Sine)),
            "reverse sine" | "rev sine" => Some(Self(LfoShape::ReverseSine)),
            _ => None,
        }
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(map_patch_value_to_step(&LFO_SHAPE_STEPS[..], value))
    }
    fn to_patch(self) -> f32 {
        map_step_to_patch_value(&LFO_SHAPE_STEPS[..], self.0)
    }
    fn get_formatted(self) -> CompactString {
        match self.0 {
            LfoShape::Triangle => "TRIANGLE".into(),
            LfoShape::ReverseTriangle => "REV TRNG".into(),
            LfoShape::Saw => "SAW".into(),
            LfoShape::ReverseSaw => "REV SAW".into(),
            LfoShape::Square => "SQUARE".into(),
            LfoShape::ReverseSquare => "REV SQR".into(),
            LfoShape::Sine => "SINE".into(),
            LfoShape::ReverseSine => "REV SINE".into(),
        }
    }

    fn get_serializable(&self) -> SerializableRepresentation {
        SerializableRepresentation::Other(self.get_formatted())
    }
}

/// LFO triangle wave
fn lfo_triangle(phase: Phase) -> f32 {
    if phase.0 <= 0.25 {
        4.0 * phase.0 as f32
    } else if phase.0 <= 0.75 {
        1.0 - 4.0 * (phase.0 as f32 - 0.25)
    } else {
        -1.0 + 4.0 * (phase.0 as f32 - 0.75)
    }
}

/// LFO saw wave
fn lfo_saw(phase: Phase) -> f32 {
    (phase.0 as f32 - 0.5) * 2.0
}

/// LFO square wave
fn lfo_square(phase: Phase) -> f32 {
    // To check absense of branches, make function public and run:
    // `cargo asm --lib -p octasine "octasine::parameters::lfo_shape::square" --rust --color`

    const PEAK_END: f32 = 32.0 / 64.0;
    const BASE_START: f32 = 33.0 / 64.0;

    let transitioning = 1.0 - 2.0 * ((phase.0 as f32 - PEAK_END) / (BASE_START - PEAK_END));

    let mut v = -1.0;

    v = if phase.0 as f32 <= BASE_START {
        transitioning
    } else {
        v
    };
    v = if phase.0 as f32 <= PEAK_END { 1.0 } else { v };

    v
}

/// LFO sine wave
fn lfo_sine(phase: Phase) -> f32 {
    ::sleef_trig::Sleef_sinf1_u35purec_range125(phase.0 as f32 * TAU)
}
