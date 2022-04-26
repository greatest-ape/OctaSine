use crate::common::*;
use crate::constants::*;

use crate::parameters::utils::*;

macro_rules! impl_envelope_duration_value_conversion {
    ($struct_name:ident) => {
        impl ParameterValue for $struct_name {
            type Value = f64;

            fn from_processing(value: Self::Value) -> Self {
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

            fn from_processing(value: Self::Value) -> Self {
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

pub trait ParameterValue: Sized {
    type Value: Copy;

    fn from_processing(value: Self::Value) -> Self;
    /// Get inner (processing) value
    fn get(self) -> Self::Value;
    fn from_sync(value: f64) -> Self;
    fn to_sync(self) -> f64;
    fn format(self) -> String;
    fn format_sync(value: f64) -> String;
    fn from_text(_text: String) -> Option<Self> {
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MasterVolumeValue(f64);

impl Default for MasterVolumeValue {
    fn default() -> Self {
        Self(DEFAULT_MASTER_VOLUME)
    }
}

impl ParameterValue for MasterVolumeValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(value: f64) -> Self {
        Self(value * 2.0)
    }
    fn to_sync(self) -> f64 {
        self.0 / 2.0
    }
    fn format(self) -> String {
        format!("{:.2} dB", 20.0 * self.0.log10())
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MasterFrequencyValue(f64);

impl Default for MasterFrequencyValue {
    fn default() -> Self {
        Self(DEFAULT_MASTER_FREQUENCY)
    }
}

impl ParameterValue for MasterFrequencyValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(
            &MASTER_FREQUENCY_STEPS,
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_value_to_parameter_value_with_steps(&MASTER_FREQUENCY_STEPS, self.0)
    }
    fn format(self) -> String {
        if self.0 < 10000.0 {
            format!("{:.02} Hz", self.0)
        } else {
            format!("{:.02}", self.0)
        }
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OperatorVolumeValue(f64);

impl Default for OperatorVolumeValue {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_VOLUME)
    }
}

impl ParameterValue for OperatorVolumeValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(value: f64) -> Self {
        Self(value * 2.0)
    }
    fn to_sync(self) -> f64 {
        self.0 / 2.0
    }
    fn format(self) -> String {
        format!("{:.04}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OperatorVolumeToggleValue(f64);

impl Default for OperatorVolumeToggleValue {
    fn default() -> Self {
        Self(1.0)
    }
}

impl ParameterValue for OperatorVolumeToggleValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value.round())
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(value: f64) -> Self {
        Self(value.round())
    }
    fn to_sync(self) -> f64 {
        self.0
    }
    fn format(self) -> String {
        if self.0 < 0.5 {
            "Off".into()
        } else {
            "On".into()
        }
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OperatorMixValue(f64);

impl OperatorMixValue {
    pub fn new(index: usize) -> Self {
        if index == 0 {
            Self(DEFAULT_OPERATOR_VOLUME)
        } else {
            Self(0.0)
        }
    }
}

impl ParameterValue for OperatorMixValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(sync * 2.0)
    }
    fn to_sync(self) -> f64 {
        self.0 / 2.0
    }
    fn format(self) -> String {
        format!("{:.04}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
    fn from_text(text: String) -> Option<Self> {
        text.parse::<f64>().map(|v| Self(v.max(0.0).min(2.0))).ok()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OperatorFrequencyRatioValue(f64);

impl Default for OperatorFrequencyRatioValue {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_FREQUENCY_RATIO)
    }
}

impl ParameterValue for OperatorFrequencyRatioValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(&OPERATOR_RATIO_STEPS[..], sync))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(&OPERATOR_RATIO_STEPS[..], self.0)
    }
    fn format(self) -> String {
        format!("{:.04}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
    fn from_text(text: String) -> Option<Self> {
        text.parse::<f64>()
            .ok()
            .map(|value| Self(round_to_step(&OPERATOR_RATIO_STEPS[..], value)))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OperatorFrequencyFreeValue(f64);

impl Default for OperatorFrequencyFreeValue {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_FREQUENCY_FREE)
    }
}

impl ParameterValue for OperatorFrequencyFreeValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(
            &OPERATOR_FREE_STEPS,
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_value_to_parameter_value_with_steps(&OPERATOR_FREE_STEPS, self.0)
    }
    fn format(self) -> String {
        format!("{:.04}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OperatorFrequencyFineValue(f64);

impl Default for OperatorFrequencyFineValue {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_FREQUENCY_FINE)
    }
}

impl ParameterValue for OperatorFrequencyFineValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(
            &OPERATOR_FINE_STEPS,
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_value_to_parameter_value_with_steps(&OPERATOR_FINE_STEPS, self.0)
    }
    fn format(self) -> String {
        format!("{:.04}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OperatorFeedbackValue(f64);

impl Default for OperatorFeedbackValue {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_FEEDBACK)
    }
}

impl ParameterValue for OperatorFeedbackValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(
            &OPERATOR_BETA_STEPS[..],
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_value_to_parameter_value_with_steps(&OPERATOR_BETA_STEPS[..], self.0)
    }
    fn format(self) -> String {
        format!("{:.04}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OperatorModulationIndexValue(f64);

impl Default for OperatorModulationIndexValue {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_MODULATION_INDEX)
    }
}

impl ParameterValue for OperatorModulationIndexValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(
            &OPERATOR_BETA_STEPS[..],
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_value_to_parameter_value_with_steps(&OPERATOR_BETA_STEPS[..], self.0)
    }
    fn format(self) -> String {
        format!("{:.04}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OperatorWaveTypeValue(pub WaveType);

impl Default for OperatorWaveTypeValue {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_WAVE_TYPE)
    }
}

impl ParameterValue for OperatorWaveTypeValue {
    type Value = WaveType;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
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
            WaveType::Sine => "SINE".to_string(),
            WaveType::WhiteNoise => "NOISE".to_string(),
        }
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
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

#[derive(Debug, Clone, Copy)]
pub struct OperatorPanningValue(f64);

impl Default for OperatorPanningValue {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_PANNING)
    }
}

impl ParameterValue for OperatorPanningValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(sync)
    }
    fn to_sync(self) -> f64 {
        self.0
    }
    fn format(self) -> String {
        let pan = ((self.0 - 0.5) * 100.0) as isize;

        match pan.cmp(&0) {
            std::cmp::Ordering::Greater => format!("{}R", pan),
            std::cmp::Ordering::Less => format!("{}L", pan),
            std::cmp::Ordering::Equal => "C".to_string(),
        }
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Operator2ModulationTargetValue(ModTargetStorage<1>);

impl ParameterValue for Operator2ModulationTargetValue {
    type Value = ModTargetStorage<1>;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(
            Self::Value::permutations(),
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(Self::Value::permutations(), self.0)
    }
    fn format(self) -> String {
        self.0.to_string()
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Operator3ModulationTargetValue(ModTargetStorage<2>);

impl ParameterValue for Operator3ModulationTargetValue {
    type Value = ModTargetStorage<2>;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(
            Self::Value::permutations(),
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(Self::Value::permutations(), self.0)
    }
    fn format(self) -> String {
        self.0.to_string()
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Operator4ModulationTargetValue(ModTargetStorage<3>);

impl ParameterValue for Operator4ModulationTargetValue {
    type Value = ModTargetStorage<3>;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(
            Self::Value::permutations(),
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(Self::Value::permutations(), self.0)
    }
    fn format(self) -> String {
        self.0.to_string()
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lfo1TargetParameterValue(pub LfoTargetParameter);

impl Default for Lfo1TargetParameterValue {
    fn default() -> Self {
        Self(LfoTargetParameter::Master(LfoTargetMasterParameter::Volume))
    }
}

impl ParameterValue for Lfo1TargetParameterValue {
    type Value = LfoTargetParameter;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(
            get_lfo_target_parameters(0),
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(get_lfo_target_parameters(0), self.0)
    }
    fn format(self) -> String {
        self.0.to_string()
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lfo2TargetParameterValue(pub LfoTargetParameter);

impl Default for Lfo2TargetParameterValue {
    fn default() -> Self {
        Self(LfoTargetParameter::Master(LfoTargetMasterParameter::Volume))
    }
}

impl ParameterValue for Lfo2TargetParameterValue {
    type Value = LfoTargetParameter;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(
            get_lfo_target_parameters(1),
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(get_lfo_target_parameters(1), self.0)
    }
    fn format(self) -> String {
        self.0.to_string()
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lfo3TargetParameterValue(pub LfoTargetParameter);

impl Default for Lfo3TargetParameterValue {
    fn default() -> Self {
        Self(LfoTargetParameter::Master(LfoTargetMasterParameter::Volume))
    }
}

impl ParameterValue for Lfo3TargetParameterValue {
    type Value = LfoTargetParameter;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(
            get_lfo_target_parameters(2),
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(get_lfo_target_parameters(2), self.0)
    }
    fn format(self) -> String {
        self.0.to_string()
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lfo4TargetParameterValue(pub LfoTargetParameter);

impl Default for Lfo4TargetParameterValue {
    fn default() -> Self {
        Self(LfoTargetParameter::Master(LfoTargetMasterParameter::Volume))
    }
}

impl ParameterValue for Lfo4TargetParameterValue {
    type Value = LfoTargetParameter;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(
            get_lfo_target_parameters(3),
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(get_lfo_target_parameters(3), self.0)
    }
    fn format(self) -> String {
        self.0.to_string()
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

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

#[derive(Debug, Clone, Copy)]
pub struct LfoModeValue(pub LfoMode);

impl Default for LfoModeValue {
    fn default() -> Self {
        Self(DEFAULT_LFO_MODE)
    }
}

impl ParameterValue for LfoModeValue {
    type Value = LfoMode;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(&LFO_MODE_STEPS[..], sync))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(&LFO_MODE_STEPS[..], self.0)
    }
    fn format(self) -> String {
        match self.0 {
            LfoMode::Once => "ONCE".to_string(),
            LfoMode::Forever => "LOOP".to_string(),
        }
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
    fn from_text(text: String) -> Option<Self> {
        match text.to_lowercase().as_ref() {
            "once" => Some(Self(LfoMode::Once)),
            "forever" => Some(Self(LfoMode::Forever)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LfoBpmSyncValue(pub bool);

impl Default for LfoBpmSyncValue {
    fn default() -> Self {
        Self(true)
    }
}

impl ParameterValue for LfoBpmSyncValue {
    type Value = bool;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(sync <= 0.5)
    }
    fn to_sync(self) -> f64 {
        if self.0 {
            0.0
        } else {
            1.0
        }
    }
    fn format(self) -> String {
        if self.0 {
            "On".to_string()
        } else {
            "Off".to_string()
        }
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
    fn from_text(text: String) -> Option<Self> {
        match text.to_lowercase().as_ref() {
            "true" | "on" => Some(Self(true)),
            "false" | "off" => Some(Self(false)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LfoFrequencyRatioValue(pub f64);

impl Default for LfoFrequencyRatioValue {
    fn default() -> Self {
        Self(1.0)
    }
}

impl ParameterValue for LfoFrequencyRatioValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(
            &LFO_FREQUENCY_RATIO_STEPS,
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(&LFO_FREQUENCY_RATIO_STEPS, self.0)
    }
    fn format(self) -> String {
        format!("{:.04}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
    fn from_text(_: String) -> Option<Self> {
        None // FIXME
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LfoFrequencyFreeValue(pub f64);

impl Default for LfoFrequencyFreeValue {
    fn default() -> Self {
        Self(1.0)
    }
}

impl ParameterValue for LfoFrequencyFreeValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(
            &LFO_FREQUENCY_FREE_STEPS,
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_value_to_parameter_value_with_steps(&LFO_FREQUENCY_FREE_STEPS, self.0)
    }
    fn format(self) -> String {
        format!("{:.04}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
    fn from_text(_: String) -> Option<Self> {
        None // FIXME
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LfoAmountValue(pub f64);

impl Default for LfoAmountValue {
    fn default() -> Self {
        Self(0.0)
    }
}

impl ParameterValue for LfoAmountValue {
    type Value = f64;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(sync * 2.0)
    }
    fn to_sync(self) -> f64 {
        self.0 * 0.5
    }
    fn format(self) -> String {
        format!("{:.04}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
    fn from_text(_: String) -> Option<Self> {
        None // FIXME
    }
}
