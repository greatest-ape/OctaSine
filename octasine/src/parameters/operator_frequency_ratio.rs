use std::f64::consts::PI;
use std::str::FromStr;

use arrayvec::ArrayString;
use compact_str::CompactString;
use once_cell::sync::Lazy;

use super::utils::*;
use super::ParameterValue;
use super::SerializableRepresentation;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ratio {
    pub name: ArrayString<20>,
    pub value: f64,
}

impl Ratio {
    fn new<N>(value: f64, name: N) -> Self
    where
        String: From<N>,
    {
        let name = ArrayString::from_str(&String::from(name)).unwrap();

        Self { name, value }
    }
}

static OPERATOR_RATIO_STEPS: Lazy<Vec<Ratio>> = Lazy::new(|| {
    let mut ratios = vec![
        Ratio::new(1.0 / 8.0, "1/8"), // 0.125
        Ratio::new(1.0 / 6.0, "1/6"), // 0.167
        Ratio::new(1.0 / 5.0, "1/5"), // 0.20
        Ratio::new(1.0 / 4.0, "1/4"), // 0.25
        Ratio::new(1.0 / 3.0, "1/3"), // 0.33 (perfect 5th)
        Ratio::new(3.0 / 8.0, "3/8"), // 0.375 (perfect 4th)
        Ratio::new(1.0 / 2.0, "1/2"), // 0.50
        Ratio::new(2.0 / 3.0, "2/3"), // 0.67 (perfect 5th)
        Ratio::new(3.0 / 4.0, "3/4"), // 0.75 (perfect 4th)
        Ratio::new(4.0 / 5.0, "4/5"), // 0.80
        Ratio::new(5.0 / 6.0, "5/6"), // 0.83
        Ratio::new(1.0, "1"),
        Ratio::new(75. / 64., "75/64"), // 1.17 (just augmented second)
        Ratio::new(6.0 / 5.0, "6/5"),   // 1.20 (minor 3rd)
        Ratio::new(5.0 / 4.0, "5/4"),   // 1.25 (major 3rd)
        Ratio::new(4.0 / 3.0, "4/3"),   // 1.33 (perfect 4th)
        Ratio::new(3.0 / 2.0, "3/2"),   // 1.50 (perfect 5th)
        Ratio::new(5.0 / 3.0, "5/3"),   // 1.67 (major 6th)
        Ratio::new(2.0, "2"),
        Ratio::new(5.0 / 2.0, "5/2"), // 2.50 (major 3rd)
        Ratio::new(3.0, "3"),
        Ratio::new(10. / 3., "10/3"), // 3.33 (major 6th)
        Ratio::new(4.0, "4"),
        Ratio::new(5.0 / 1.0, "5"), // 5.00 (major 3rd)
        Ratio::new(6.0, "6"),
        Ratio::new(8.0, "8"),
    ];

    // Add DX ratios

    for i in 0..16 {
        match i {
            5 | 15 => {}
            0 => {
                let value = 0.5 * 2.0f64.sqrt();
                ratios.push(Ratio::new(value, "sqrt(2)/2".to_string()));
            }
            1 => {
                let value = 2.0f64.sqrt();
                ratios.push(Ratio::new(value, "sqrt(2)".to_string()));
            }
            i => {
                let value = i as f64 * 2.0f64.sqrt();
                ratios.push(Ratio::new(value, format!("{} sqrt(2)", i)));
            }
        }
    }
    for i in 0..16 {
        match i {
            0 => {
                let value = 0.5 * 3.0f64.sqrt();
                ratios.push(Ratio::new(value, "sqrt(3)/2".to_string()));
            }
            1 => {
                let value = 3.0f64.sqrt();
                ratios.push(Ratio::new(value, "sqrt(3)".to_string()));
            }
            i => {
                let value = i as f64 * 3.0f64.sqrt();
                ratios.push(Ratio::new(value, format!("{} sqrt(3)", i)));
            }
        }
    }

    for i in 1..8 {
        let name = if i == 1 {
            String::from("π")
        } else {
            format!("{}π", i)
        };

        ratios.push(Ratio::new(i as f64 * PI, name));
    }

    for i in [1, 2, 6, 9, 10, 14, 18, 22, 26, 27, 30] {
        let factor = i as f64 / 4.0;

        ratios.push(Ratio::new(factor * PI, format!("{}π", factor)));
    }

    ratios.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());

    ratios
});

#[derive(Debug, Clone, Copy)]
pub struct OperatorFrequencyRatioValue(Ratio);

impl Default for OperatorFrequencyRatioValue {
    fn default() -> Self {
        Self(
            *OPERATOR_RATIO_STEPS
                .iter()
                .find(|r| r.value == 1.0)
                .unwrap(),
        )
    }
}

impl ParameterValue for OperatorFrequencyRatioValue {
    type Value = Ratio;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: &str) -> Option<Self> {
        let text = text
            .trim()
            .to_lowercase()
            .replace(" pi", "π")
            .replace("pi", "π");

        for ratio in OPERATOR_RATIO_STEPS.iter() {
            if ratio.name.as_str() == text {
                return Some(Self(*ratio));
            }
        }

        if let Ok(value) = text.parse::<f64>() {
            if value == 3.14 {
                return OPERATOR_RATIO_STEPS
                    .iter()
                    .find(|r| r.value == PI)
                    .copied()
                    .map(Self);
            }

            for ratio in OPERATOR_RATIO_STEPS.iter() {
                if value == ratio.value {
                    return Some(Self(*ratio));
                }
            }
        }

        None
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(map_patch_value_to_step(&OPERATOR_RATIO_STEPS[..], value))
    }
    fn to_patch(self) -> f32 {
        map_step_to_patch_value(&OPERATOR_RATIO_STEPS[..], self.0)
    }
    fn get_formatted(self) -> CompactString {
        self.0.name.as_str().into()
    }

    fn get_serializable(&self) -> SerializableRepresentation {
        SerializableRepresentation::Float(self.0.value)
    }
    fn get_text_choices() -> Option<Vec<CompactString>> {
        Some(
            OPERATOR_RATIO_STEPS
                .iter()
                .map(|ratio| CompactString::new(ratio.name.as_str()))
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ratios() {
        let ratios = &OPERATOR_RATIO_STEPS;

        assert!(!ratios.is_empty());
        assert!(ratios.contains(&OperatorFrequencyRatioValue::default().get()));
    }
}
