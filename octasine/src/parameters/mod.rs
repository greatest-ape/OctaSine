pub mod lfo_active;
pub mod lfo_amount;
pub mod lfo_bpm_sync;
pub mod lfo_frequency_free;
pub mod lfo_frequency_ratio;
pub mod lfo_mode;
pub mod lfo_shape;
pub mod lfo_target;
pub mod list;
pub mod master_frequency;
pub mod master_volume;
pub mod operator_active;
pub mod operator_envelope;
pub mod operator_feedback;
pub mod operator_frequency_fine;
pub mod operator_frequency_free;
pub mod operator_frequency_ratio;
pub mod operator_mix_out;
pub mod operator_mod_out;
pub mod operator_mod_target;
pub mod operator_panning;
pub mod operator_volume;
pub mod operator_wave_type;
pub mod utils;

pub use lfo_active::LfoActiveValue;
pub use lfo_amount::LfoAmountValue;
pub use lfo_bpm_sync::LfoBpmSyncValue;
pub use lfo_frequency_free::LfoFrequencyFreeValue;
pub use lfo_frequency_ratio::LfoFrequencyRatioValue;
pub use lfo_mode::LfoModeValue;
pub use lfo_shape::LfoShapeValue;
pub use lfo_target::*;
pub use list::*;
pub use master_frequency::MasterFrequencyValue;
pub use master_volume::MasterVolumeValue;
pub use operator_active::OperatorActiveValue;
pub use operator_envelope::*;
pub use operator_feedback::OperatorFeedbackValue;
pub use operator_frequency_fine::OperatorFrequencyFineValue;
pub use operator_frequency_free::OperatorFrequencyFreeValue;
pub use operator_frequency_ratio::OperatorFrequencyRatioValue;
pub use operator_mix_out::OperatorMixOutValue;
pub use operator_mod_out::OperatorModOutValue;
pub use operator_mod_target::*;
pub use operator_panning::OperatorPanningValue;
pub use operator_volume::OperatorVolumeValue;
pub use operator_wave_type::OperatorWaveTypeValue;
use serde::{Deserialize, Serialize};

use crate::common::{NUM_LFOS, NUM_OPERATORS};

/// Storage of audio parameter values with utilities for conversions
/// to and from patch values.
pub trait ParameterValue: Sized + Default + Copy {
    /// Value as used in audio generation
    type Value: Copy;

    fn new_from_audio(value: Self::Value) -> Self;
    fn new_from_text(_text: &str) -> Option<Self>;
    fn new_from_patch(value: f32) -> Self;

    /// Get inner (audio gen) value
    fn get(self) -> Self::Value;
    fn get_formatted(self) -> String;
    fn to_patch(self) -> f32;

    fn replace_from_patch(&mut self, value: f32) {
        *self = Self::new_from_patch(value);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ParameterKey(pub u32);

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

impl Parameter {
    pub fn name(&self) -> String {
        match self {
            Self::None => "None".into(),
            Self::Master(MasterParameter::Frequency) => "Master frequency".into(),
            Self::Master(MasterParameter::Volume) => "Master volume".into(),
            Self::Operator(index, p) => match p {
                OperatorParameter::Volume => format!("OP {} vol", index + 1),
                OperatorParameter::Active => format!("OP {} active", index + 1),
                OperatorParameter::MixOut => format!("OP {} mix out", index + 1),
                OperatorParameter::Panning => format!("OP {} pan", index + 1),
                OperatorParameter::WaveType => format!("OP {} wave", index + 1),
                OperatorParameter::ModTargets => format!("OP {} target", index + 1),
                OperatorParameter::ModOut => format!("OP {} mod out", index + 1),
                OperatorParameter::Feedback => format!("OP {} feedback", index + 1),
                OperatorParameter::FrequencyRatio => format!("OP {} freq ratio", index + 1),
                OperatorParameter::FrequencyFree => format!("OP {} freq free", index + 1),
                OperatorParameter::FrequencyFine => format!("OP {} freq fine", index + 1),
                OperatorParameter::AttackDuration => format!("OP {} attack time", index + 1),
                OperatorParameter::DecayDuration => format!("OP {} decay time", index + 1),
                OperatorParameter::SustainVolume => format!("OP {} sustain vol", index + 1),
                OperatorParameter::ReleaseDuration => format!("OP {} release time", index + 1),
                OperatorParameter::EnvelopeLockGroup => format!("OP {} lock group", index + 1),
            },
            Self::Lfo(index, p) => match p {
                LfoParameter::Target => format!("LFO {} target", index + 1),
                LfoParameter::BpmSync => format!("LFO {} bpm sync", index + 1),
                LfoParameter::FrequencyRatio => format!("LFO {} freq ratio", index + 1),
                LfoParameter::FrequencyFree => format!("LFO {} freq free", index + 1),
                LfoParameter::Mode => format!("LFO {} oneshot", index + 1),
                LfoParameter::Shape => format!("LFO {} shape", index + 1),
                LfoParameter::Amount => format!("LFO {} amount", index + 1),
                LfoParameter::Active => format!("LFO {} active", index + 1),
            },
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        PARAMETERS.get(index).copied()
    }

    pub const fn to_index(self) -> u8 {
        parameter_to_index(self)
    }

    pub fn clap_path(&self) -> String {
        match self {
            Self::None => "None".into(),
            Self::Master(_) => "Master".into(),
            Self::Operator(index, _) => format!("Operator {}", *index),
            Self::Lfo(index, _) => format!("LFO {}", *index),
        }
    }

    pub fn key(&self) -> ParameterKey {
        let name = match self {
            Self::None => "None".into(),
            Self::Master(MasterParameter::Frequency) => "Master frequency".into(),
            Self::Master(MasterParameter::Volume) => "Master volume".into(),
            Self::Operator(index, p) => match p {
                OperatorParameter::Volume => format!("OP {} vol", index + 1),
                OperatorParameter::Active => format!("OP {} active", index + 1),
                OperatorParameter::MixOut => format!("OP {} mix out", index + 1),
                OperatorParameter::Panning => format!("OP {} pan", index + 1),
                OperatorParameter::WaveType => format!("OP {} wave", index + 1),
                OperatorParameter::ModTargets => format!("OP {} target", index + 1),
                OperatorParameter::ModOut => format!("OP {} mod out", index + 1),
                OperatorParameter::Feedback => format!("OP {} feedback", index + 1),
                OperatorParameter::FrequencyRatio => format!("OP {} freq ratio", index + 1),
                OperatorParameter::FrequencyFree => format!("OP {} freq free", index + 1),
                OperatorParameter::FrequencyFine => format!("OP {} freq fine", index + 1),
                OperatorParameter::AttackDuration => format!("OP {} attack time", index + 1),
                OperatorParameter::DecayDuration => format!("OP {} decay time", index + 1),
                OperatorParameter::SustainVolume => format!("OP {} sustain vol", index + 1),
                OperatorParameter::ReleaseDuration => format!("OP {} release time", index + 1),
                OperatorParameter::EnvelopeLockGroup => format!("OP {} lock group", index + 1),
            },
            Self::Lfo(index, p) => match p {
                LfoParameter::Target => format!("LFO {} target", index + 1),
                LfoParameter::BpmSync => format!("LFO {} bpm sync", index + 1),
                LfoParameter::FrequencyRatio => format!("LFO {} freq ratio", index + 1),
                LfoParameter::FrequencyFree => format!("LFO {} freq free", index + 1),
                LfoParameter::Mode => format!("LFO {} oneshot", index + 1),
                LfoParameter::Shape => format!("LFO {} shape", index + 1),
                LfoParameter::Amount => format!("LFO {} amount", index + 1),
                LfoParameter::Active => format!("LFO {} active", index + 1),
            },
        };

        let hash = seahash::hash(name.as_bytes());
        let first_four_bytes = hash.to_ne_bytes()[..4].try_into().unwrap();

        ParameterKey(u32::from_ne_bytes(first_four_bytes))
    }
}

impl OperatorParameter {
    pub const fn index_array(self) -> [u8; NUM_OPERATORS] {
        let mut arr = [0; NUM_OPERATORS];

        let mut i = 0;

        while i < arr.len() {
            if let (0, Self::ModOut) = (i, self) {
                // There is no mod out parameter for operator 1
                arr[i] = 0;
            } else {
                arr[i] = Parameter::Operator(i as u8, self).to_index();
            }

            i += 1;
        }

        arr
    }
}

impl LfoParameter {
    pub const fn index_array(self) -> [u8; NUM_LFOS] {
        let mut arr = [0; NUM_LFOS];

        let mut i = 0;

        while i < arr.len() {
            arr[i] = Parameter::Lfo(i as u8, self).to_index();

            i += 1;
        }

        arr
    }
}

/// All metadata for a parameter
#[derive(Debug, Clone, Copy)]
pub struct WrappedParameter {
    parameter: Parameter,
    index: u8,
    key: ParameterKey,
}

impl WrappedParameter {
    pub fn parameter(&self) -> Parameter {
        self.parameter
    }
    pub fn index(&self) -> u8 {
        self.index
    }
    pub fn key(&self) -> ParameterKey {
        self.key
    }
}

impl From<Parameter> for WrappedParameter {
    fn from(parameter: Parameter) -> Self {
        Self {
            parameter,
            index: parameter.to_index(),
            key: parameter.key(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{ParameterKey, PARAMETERS};

    #[test]
    fn test_parameter_key_uniqueness() {
        let set: HashSet<ParameterKey> = PARAMETERS.iter().map(|p| p.key()).collect();

        assert_eq!(set.len(), PARAMETERS.len());
    }
}
