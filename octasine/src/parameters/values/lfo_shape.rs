use super::ParameterValue;
use crate::common::*;
use crate::constants::*;
use crate::parameters::utils::*;
#[derive(Debug, Clone, Copy)]
pub struct LfoShapeValue(pub LfoShape);

impl Default for LfoShapeValue {
    fn default() -> Self {
        Self(DEFAULT_LFO_SHAPE)
    }
}

impl ParameterValue for LfoShapeValue {
    type Value = LfoShape;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(&LFO_SHAPE_STEPS[..], sync))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(&LFO_SHAPE_STEPS[..], self.0)
    }
    fn format(self) -> String {
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
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
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
