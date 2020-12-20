use crate::common::*;
use crate::constants::*;

use super::utils::*;


macro_rules! impl_envelope_duration_value_conversion {
    ($struct_name:ident) => {
        impl ProcessingValueConversion for $struct_name {
            fn from_sync(value: f64) -> Self {
                // Force some decay to avoid clicks
                Self((value * ENVELOPE_MAX_DURATION)
                    .max(ENVELOPE_MIN_DURATION))
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
                    .map(|v| Self(v.max(ENVELOPE_MAX_DURATION).max(ENVELOPE_MAX_DURATION)))
                    .ok()
            }
        }
    };
}


macro_rules! impl_identity_value_conversion {
    ($struct_name:ident) => {
        impl ProcessingValueConversion for $struct_name {
            fn from_sync(value: f64) -> Self {
                Self(value)
            }
            fn to_sync(self) -> f64 {
                self.0
            }
            fn format(self) -> String {
                format!("{:.02}", self.0)
            }
            fn format_sync(value: f64) -> String {
                Self::from_sync(value).format()
            }
        }
    };
}


pub trait ProcessingValueConversion: Sized {
    fn from_sync(value: f64) -> Self;
    fn to_sync(self) -> f64;
    fn format(self) -> String;
    fn format_sync(value: f64) -> String;
    fn from_text(_text: String) -> Option<Self> {
        None
    }
    fn unit(&self) -> String {
        "".to_string()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct MasterVolume(pub f64);


impl Default for MasterVolume {
    fn default() -> Self {
        Self(DEFAULT_MASTER_VOLUME)
    }
}


impl ProcessingValueConversion for MasterVolume {
    fn from_sync(value: f64) -> Self {
        Self(value * 2.0)
    }
    fn to_sync(self) -> f64 {
        self.0 / 2.0
    }
    fn format(self) -> String {
        format!("{:.2}", 20.0 * self.0.log10())
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct MasterFrequency(pub f64);


impl Default for MasterFrequency {
    fn default() -> Self {
        Self(DEFAULT_MASTER_FREQUENCY)
    }
}


impl ProcessingValueConversion for MasterFrequency {
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(
            &MASTER_FREQUENCY_STEPS,
            sync
        ))
    }
    fn to_sync(self) -> f64 {
        map_value_to_parameter_value_with_steps(&MASTER_FREQUENCY_STEPS, self.0)
    }
    fn format(self) -> String {
        format!("{:.02}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct OperatorVolume(pub f64);


impl OperatorVolume {
    pub fn new(index: usize) -> Self {
        if index == 0 {
            Self(DEFAULT_OPERATOR_VOLUME)
        } else {
            Self(0.0)
        }
    }
}


impl ProcessingValueConversion for OperatorVolume {
    fn from_sync(sync: f64) -> Self {
        Self(sync * 2.0)
    }
    fn to_sync(self) -> f64 {
        self.0 / 2.0
    }
    fn format(self) -> String {
        format!("{:.02}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
    fn from_text(text: String) -> Option<Self> {
        text.parse::<f64>()
            .map(|v| Self(v.max(0.0).min(2.0)))
            .ok()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct OperatorAdditive(pub f64);


impl Default for OperatorAdditive {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_ADDITIVE_FACTOR)
    }
}


impl ProcessingValueConversion for OperatorAdditive {
    fn from_sync(sync: f64) -> Self {
        Self(sync)
    }
    fn to_sync(self) -> f64 {
        self.0
    }
    fn format(self) -> String {
        format!("{:.02}%", (self.0 * 100.0))
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct OperatorFrequencyRatio(pub f64);


impl Default for OperatorFrequencyRatio {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_FREQUENCY_RATIO)
    }
}


impl ProcessingValueConversion for OperatorFrequencyRatio {
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(&OPERATOR_RATIO_STEPS[..], sync))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(&OPERATOR_RATIO_STEPS[..], self.0)
    }
    fn format(self) -> String {
        format!("{:.02}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
    fn from_text(text: String) -> Option<Self> {
        text.parse::<f64>().ok().map(|value|
            Self(round_to_step(&OPERATOR_RATIO_STEPS[..], value))
        )
    }
}


#[derive(Debug, Clone, Copy)]
pub struct OperatorFrequencyFree(pub f64);


impl Default for OperatorFrequencyFree {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_FREQUENCY_FREE)
    }
}


impl ProcessingValueConversion for OperatorFrequencyFree {
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(&OPERATOR_FREE_STEPS, sync))
    }
    fn to_sync(self) -> f64 {
        map_value_to_parameter_value_with_steps(&OPERATOR_FREE_STEPS, self.0)
    }
    fn format(self) -> String {
        format!("{:.02}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct OperatorFrequencyFine(pub f64);


impl Default for OperatorFrequencyFine {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_FREQUENCY_FINE)
    }
}


impl ProcessingValueConversion for OperatorFrequencyFine {
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(&OPERATOR_FINE_STEPS, sync))
    }
    fn to_sync(self) -> f64 {
        map_value_to_parameter_value_with_steps(&OPERATOR_FINE_STEPS, self.0)
    }
    fn format(self) -> String {
        format!("{:.02}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct OperatorFeedback(pub f64);


impl Default for OperatorFeedback {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_FEEDBACK)
    }
}


impl ProcessingValueConversion for OperatorFeedback {
    fn from_sync(sync: f64) -> Self {
        Self(sync)
    }
    fn to_sync(self) -> f64 {
        self.0
    }
    fn format(self) -> String {
        format!("{:.02}%", (self.0 * 100.0))
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct OperatorModulationIndex(pub f64);


impl Default for OperatorModulationIndex {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_MODULATION_INDEX)
    }
}


impl ProcessingValueConversion for OperatorModulationIndex {
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(&OPERATOR_BETA_STEPS[..], sync))
    }
    fn to_sync(self) -> f64 {
        map_value_to_parameter_value_with_steps(&OPERATOR_BETA_STEPS[..], self.0)
    }
    fn format(self) -> String {
        format!("{:.02}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct OperatorWaveType(pub WaveType);


impl Default for OperatorWaveType {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_WAVE_TYPE)
    }
}


impl ProcessingValueConversion for OperatorWaveType {
    fn from_sync(sync: f64) -> Self {
        if sync <= 0.5 {
            Self(WaveType::Sine)
        } else {
            Self(WaveType::WhiteNoise)
        }
    }
    fn to_sync(self) -> f64 {
        match self.0 {
            WaveType::Sine => 0.0,
            WaveType::WhiteNoise => 1.0,
        }
    }
    fn format(self) -> String {
        match self.0 {
            WaveType::Sine => "Sine".to_string(),
            WaveType::WhiteNoise => "White noise".to_string(),
        }
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
    fn from_text(text: String) -> Option<Self> {
        let value = text.to_lowercase();

        if value.contains("sin"){
            Some(OperatorWaveType(WaveType::Sine))
        } else if value.contains("noise") {
            Some(OperatorWaveType(WaveType::WhiteNoise))
        } else {
            None
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct OperatorAttackDuration(pub f64);


impl Default for OperatorAttackDuration {
    fn default() -> Self {
        Self(DEFAULT_ENVELOPE_ATTACK_DURATION)
    }
}


impl_envelope_duration_value_conversion!(OperatorAttackDuration);


#[derive(Debug, Clone, Copy)]
pub struct OperatorDecayDuration(pub f64);


impl Default for OperatorDecayDuration {
    fn default() -> Self {
        Self(DEFAULT_ENVELOPE_DECAY_DURATION)
    }
}


impl_envelope_duration_value_conversion!(OperatorDecayDuration);


#[derive(Debug, Clone, Copy)]
pub struct OperatorReleaseDuration(pub f64);


impl Default for OperatorReleaseDuration {
    fn default() -> Self {
        Self(DEFAULT_ENVELOPE_RELEASE_DURATION)
    }
}


impl_envelope_duration_value_conversion!(OperatorReleaseDuration);


#[derive(Debug, Clone, Copy)]
pub struct OperatorAttackVolume(pub f64);


impl Default for OperatorAttackVolume {
    fn default() -> Self {
        Self(DEFAULT_ENVELOPE_ATTACK_VOLUME)
    }
}


impl_identity_value_conversion!(OperatorAttackVolume);


#[derive(Debug, Clone, Copy)]
pub struct OperatorDecayVolume(pub f64);


impl Default for OperatorDecayVolume {
    fn default() -> Self {
        Self(DEFAULT_ENVELOPE_DECAY_VOLUME)
    }
}


impl_identity_value_conversion!(OperatorDecayVolume);


#[derive(Debug, Clone, Copy)]
pub struct OperatorPanning(pub f64);


impl Default for OperatorPanning {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_PANNING)
    }
}


impl ProcessingValueConversion for OperatorPanning {
    fn from_sync(sync: f64) -> Self {
        Self(sync)
    }
    fn to_sync(self) -> f64 {
        self.0
    }
    fn format(self) -> String {
        let tmp = ((self.0 - 0.5) * 100.0) as isize;

        if tmp > 0 {
            format!("{}R", tmp)
        } else if tmp < 0 {
            format!("{}L", tmp)
        } else {
            "C".to_string()
        }
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct OperatorModulationTarget2(pub usize);


impl Default for OperatorModulationTarget2 {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_3_MOD_TARGET)
    }
}


impl ProcessingValueConversion for OperatorModulationTarget2 {
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(&[0, 1], sync))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(&[0, 1], self.0)
    }
    fn format(self) -> String {
        format!("Operator {}", self.0 + 1)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
    fn from_text(text: String) -> Option<Self> {
        if let Ok(value) = text.parse::<usize>(){
            if value == 1 || value == 2 {
                return Some(Self(value - 1));
            }
        }

        None
    }
}


#[derive(Debug, Clone, Copy)]
pub struct OperatorModulationTarget3(pub usize);


impl Default for OperatorModulationTarget3 {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_4_MOD_TARGET)
    }
}


impl ProcessingValueConversion for OperatorModulationTarget3 {
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(&[0, 1, 2], sync))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(&[0, 1, 2], self.0)
    }
    fn format(self) -> String {
        format!("Operator {}", self.0 + 1)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
    fn from_text(text: String) -> Option<Self> {
        if let Ok(value) = text.parse::<usize>(){
            if value == 1 || value == 2 || value == 3 {
                return Some(Self(value - 1));
            }
        }

        None
    }
}