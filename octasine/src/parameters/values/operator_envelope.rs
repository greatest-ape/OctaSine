use super::ParameterValue;

pub const ENVELOPE_MAX_DURATION: f64 = 4.0;
pub const ENVELOPE_MIN_DURATION: f64 = 0.004;

/// After this duration, the envelope slope does not get mixed with linear
/// slope at all
pub const ENVELOPE_CURVE_TAKEOVER: f64 = ENVELOPE_MIN_DURATION * 10.0;
pub const ENVELOPE_CURVE_TAKEOVER_RECIP: f64 = 1.0 / ENVELOPE_CURVE_TAKEOVER;

const DEFAULT_ENVELOPE_ATTACK_DURATION: f64 = ENVELOPE_MIN_DURATION;
const DEFAULT_ENVELOPE_ATTACK_VOLUME: f64 = 1.0;
const DEFAULT_ENVELOPE_DECAY_DURATION: f64 = ENVELOPE_MIN_DURATION;
const DEFAULT_ENVELOPE_DECAY_VOLUME: f64 = 1.0;
const DEFAULT_ENVELOPE_RELEASE_DURATION: f64 = 0.25;

macro_rules! impl_envelope_duration_value_conversion {
    ($struct_name:ident) => {
        impl ParameterValue for $struct_name {
            type Value = f64;

            fn from_audio(value: Self::Value) -> Self {
                Self(value)
            }

            fn get(self) -> Self::Value {
                self.0
            }
            fn from_sync(value: f64) -> Self {
                // Force some decay to avoid clicks
                Self((value * ENVELOPE_MAX_DURATION).max(ENVELOPE_MIN_DURATION))
            }
            fn to_sync(self) -> f64 {
                self.0 / ENVELOPE_MAX_DURATION
            }

            fn format(self) -> String {
                format!("{:.02}", self.0)
            }

            fn format_sync(value: f64) -> String {
                Self::from_sync(value).format()
            }
            fn from_text(text: String) -> Option<Self> {
                text.parse::<f64>()
                    .map(|v| {
                        let v = v.min(ENVELOPE_MAX_DURATION).max(ENVELOPE_MIN_DURATION);

                        Self(v)
                    })
                    .ok()
            }
        }
    };
}

macro_rules! impl_identity_value_conversion {
    ($struct_name:ident) => {
        impl ParameterValue for $struct_name {
            type Value = f64;

            fn from_audio(value: Self::Value) -> Self {
                Self(value)
            }

            fn get(self) -> Self::Value {
                self.0
            }
            fn from_sync(value: f64) -> Self {
                Self(value)
            }
            fn to_sync(self) -> f64 {
                self.0
            }
            fn format(self) -> String {
                format!("{:.04}", self.0)
            }
            fn format_sync(value: f64) -> String {
                Self::from_sync(value).format()
            }
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub struct OperatorAttackDurationValue(f64);

impl Default for OperatorAttackDurationValue {
    fn default() -> Self {
        Self(DEFAULT_ENVELOPE_ATTACK_DURATION)
    }
}

impl_envelope_duration_value_conversion!(OperatorAttackDurationValue);

#[derive(Debug, Clone, Copy)]
pub struct OperatorDecayDurationValue(f64);

impl Default for OperatorDecayDurationValue {
    fn default() -> Self {
        Self(DEFAULT_ENVELOPE_DECAY_DURATION)
    }
}

impl_envelope_duration_value_conversion!(OperatorDecayDurationValue);

#[derive(Debug, Clone, Copy)]
pub struct OperatorReleaseDurationValue(f64);

impl Default for OperatorReleaseDurationValue {
    fn default() -> Self {
        Self(DEFAULT_ENVELOPE_RELEASE_DURATION)
    }
}

impl_envelope_duration_value_conversion!(OperatorReleaseDurationValue);

#[derive(Debug, Clone, Copy)]
pub struct OperatorAttackVolumeValue(f64);

impl Default for OperatorAttackVolumeValue {
    fn default() -> Self {
        Self(DEFAULT_ENVELOPE_ATTACK_VOLUME)
    }
}

impl_identity_value_conversion!(OperatorAttackVolumeValue);

#[derive(Debug, Clone, Copy)]
pub struct OperatorDecayVolumeValue(f64);

impl Default for OperatorDecayVolumeValue {
    fn default() -> Self {
        Self(DEFAULT_ENVELOPE_DECAY_VOLUME)
    }
}

impl_identity_value_conversion!(OperatorDecayVolumeValue);
