use std::str::FromStr;

use compact_str::{format_compact, CompactString};

use super::{
    utils::{map_patch_value_to_step, map_step_to_patch_value, parse_valid_f32, parse_valid_f64},
    ParameterValue, SerializableRepresentation,
};

pub const ENVELOPE_MAX_DURATION: f64 = 4.0;
pub const ENVELOPE_MIN_DURATION: f64 = 0.00333;

/// After this duration, the envelope slope does not get mixed with linear
/// slope at all
pub const ENVELOPE_CURVE_TAKEOVER: f64 = 0.05;
pub const ENVELOPE_CURVE_TAKEOVER_RECIP: f64 = 1.0 / ENVELOPE_CURVE_TAKEOVER;

const DEFAULT_ATTACK: f64 = ENVELOPE_MIN_DURATION;
const DEFAULT_DECAY: f64 = ENVELOPE_MIN_DURATION;
const DEFAULT_SUSTAIN: f32 = 1.0;
const DEFAULT_RELEASE: f64 = 0.25;

macro_rules! impl_duration_parameter_value {
    ($struct_name:ident) => {
        impl ParameterValue for $struct_name {
            type Value = f64;

            fn new_from_audio(value: Self::Value) -> Self {
                Self(value)
            }

            fn get(self) -> Self::Value {
                self.0
            }
            fn new_from_patch(value: f32) -> Self {
                // Force some decay to avoid clicks
                Self((value as f64 * ENVELOPE_MAX_DURATION).max(ENVELOPE_MIN_DURATION))
            }
            fn to_patch(self) -> f32 {
                (self.0 / ENVELOPE_MAX_DURATION) as f32
            }

            fn get_formatted(self) -> CompactString {
                format_compact!("{:.02}", self.0)
            }

            fn new_from_text(text: &str) -> Option<Self> {
                parse_valid_f64(text, ENVELOPE_MIN_DURATION, ENVELOPE_MAX_DURATION)
                    .map(|v| Self(v.into()))
            }

            fn get_serializable(&self) -> SerializableRepresentation {
                SerializableRepresentation::Float(self.0.into())
            }
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub struct OperatorAttackDurationValue(f64);

impl Default for OperatorAttackDurationValue {
    fn default() -> Self {
        Self(DEFAULT_ATTACK)
    }
}

impl_duration_parameter_value!(OperatorAttackDurationValue);

#[derive(Debug, Clone, Copy)]
pub struct OperatorDecayDurationValue(f64);

impl Default for OperatorDecayDurationValue {
    fn default() -> Self {
        Self(DEFAULT_DECAY)
    }
}

impl_duration_parameter_value!(OperatorDecayDurationValue);

#[derive(Debug, Clone, Copy)]
pub struct OperatorReleaseDurationValue(f64);

impl Default for OperatorReleaseDurationValue {
    fn default() -> Self {
        Self(DEFAULT_RELEASE)
    }
}

impl_duration_parameter_value!(OperatorReleaseDurationValue);

#[derive(Debug, Clone, Copy)]
pub struct OperatorSustainVolumeValue(f32);

impl Default for OperatorSustainVolumeValue {
    fn default() -> Self {
        Self(DEFAULT_SUSTAIN)
    }
}

impl ParameterValue for OperatorSustainVolumeValue {
    type Value = f32;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: &str) -> Option<Self> {
        parse_valid_f32(text, 0.0, 1.0).map(Self)
    }

    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(value)
    }
    fn to_patch(self) -> f32 {
        self.0
    }
    fn get_formatted(self) -> CompactString {
        format_compact!("{:.04}", self.0)
    }

    fn get_serializable(&self) -> SerializableRepresentation {
        SerializableRepresentation::Float(self.0.into())
    }
}

const LOCK_STEPS: &[OperatorEnvelopeGroupValue] = &[
    OperatorEnvelopeGroupValue::Off,
    OperatorEnvelopeGroupValue::A,
    OperatorEnvelopeGroupValue::B,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorEnvelopeGroupValue {
    Off,
    A,
    B,
}

impl Default for OperatorEnvelopeGroupValue {
    fn default() -> Self {
        Self::Off
    }
}

impl FromStr for OperatorEnvelopeGroupValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "off" | "none" => Ok(Self::Off),
            _ => Err(()),
        }
    }
}

impl ParameterValue for OperatorEnvelopeGroupValue {
    type Value = Self;

    fn new_from_audio(value: Self::Value) -> Self {
        value
    }
    fn new_from_text(text: &str) -> Option<Self> {
        text.parse().ok()
    }

    fn get(self) -> Self::Value {
        self
    }
    fn new_from_patch(value: f32) -> Self {
        map_patch_value_to_step(LOCK_STEPS, value)
    }
    fn to_patch(self) -> f32 {
        map_step_to_patch_value(LOCK_STEPS, self)
    }
    fn get_formatted(self) -> CompactString {
        format_compact!("{:?}", self)
    }

    fn get_serializable(&self) -> SerializableRepresentation {
        SerializableRepresentation::Other(self.get_formatted())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attack_duration_from_text() {
        assert_eq!(
            OperatorAttackDurationValue::new_from_text("0.0")
                .unwrap()
                .get(),
            ENVELOPE_MIN_DURATION
        );

        assert_eq!(
            OperatorAttackDurationValue::new_from_text("1.0")
                .unwrap()
                .get(),
            1.0
        );

        assert_eq!(
            OperatorAttackDurationValue::new_from_text("10.0")
                .unwrap()
                .get(),
            ENVELOPE_MAX_DURATION
        );
    }
}
