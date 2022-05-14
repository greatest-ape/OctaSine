pub mod lfo_active;
pub mod lfo_amount;
pub mod lfo_bpm_sync;
pub mod lfo_frequency_free;
pub mod lfo_frequency_ratio;
pub mod lfo_mode;
pub mod lfo_shape;
pub mod lfo_target;
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

/// Authoritative list of parameters in order
pub const PARAMETERS: &[Parameter] = &[
    Parameter::Master(MasterParameter::Volume),
    Parameter::Master(MasterParameter::Frequency),
    Parameter::Operator(0, OperatorParameter::Volume),
    Parameter::Operator(0, OperatorParameter::Active),
    Parameter::Operator(0, OperatorParameter::MixOut),
    Parameter::Operator(0, OperatorParameter::Panning),
    Parameter::Operator(0, OperatorParameter::WaveType),
    Parameter::Operator(0, OperatorParameter::Feedback),
    Parameter::Operator(0, OperatorParameter::FrequencyRatio),
    Parameter::Operator(0, OperatorParameter::FrequencyFree),
    Parameter::Operator(0, OperatorParameter::FrequencyFine),
    Parameter::Operator(0, OperatorParameter::AttackDuration),
    Parameter::Operator(0, OperatorParameter::AttackValue),
    Parameter::Operator(0, OperatorParameter::DecayDuration),
    Parameter::Operator(0, OperatorParameter::DecayValue),
    Parameter::Operator(0, OperatorParameter::ReleaseDuration),
    Parameter::Operator(1, OperatorParameter::Volume),
    Parameter::Operator(1, OperatorParameter::Active),
    Parameter::Operator(1, OperatorParameter::MixOut),
    Parameter::Operator(1, OperatorParameter::Panning),
    Parameter::Operator(1, OperatorParameter::WaveType),
    Parameter::Operator(1, OperatorParameter::ModTargets),
    Parameter::Operator(1, OperatorParameter::ModOut),
    Parameter::Operator(1, OperatorParameter::Feedback),
    Parameter::Operator(1, OperatorParameter::FrequencyRatio),
    Parameter::Operator(1, OperatorParameter::FrequencyFree),
    Parameter::Operator(1, OperatorParameter::FrequencyFine),
    Parameter::Operator(1, OperatorParameter::AttackDuration),
    Parameter::Operator(1, OperatorParameter::AttackValue),
    Parameter::Operator(1, OperatorParameter::DecayDuration),
    Parameter::Operator(1, OperatorParameter::DecayValue),
    Parameter::Operator(1, OperatorParameter::ReleaseDuration),
    Parameter::Operator(2, OperatorParameter::Volume),
    Parameter::Operator(2, OperatorParameter::Active),
    Parameter::Operator(2, OperatorParameter::MixOut),
    Parameter::Operator(2, OperatorParameter::Panning),
    Parameter::Operator(2, OperatorParameter::WaveType),
    Parameter::Operator(2, OperatorParameter::ModTargets),
    Parameter::Operator(2, OperatorParameter::ModOut),
    Parameter::Operator(2, OperatorParameter::Feedback),
    Parameter::Operator(2, OperatorParameter::FrequencyRatio),
    Parameter::Operator(2, OperatorParameter::FrequencyFree),
    Parameter::Operator(2, OperatorParameter::FrequencyFine),
    Parameter::Operator(2, OperatorParameter::AttackDuration),
    Parameter::Operator(2, OperatorParameter::AttackValue),
    Parameter::Operator(2, OperatorParameter::DecayDuration),
    Parameter::Operator(2, OperatorParameter::DecayValue),
    Parameter::Operator(2, OperatorParameter::ReleaseDuration),
    Parameter::Operator(3, OperatorParameter::Volume),
    Parameter::Operator(3, OperatorParameter::Active),
    Parameter::Operator(3, OperatorParameter::MixOut),
    Parameter::Operator(3, OperatorParameter::Panning),
    Parameter::Operator(3, OperatorParameter::WaveType),
    Parameter::Operator(3, OperatorParameter::ModTargets),
    Parameter::Operator(3, OperatorParameter::ModOut),
    Parameter::Operator(3, OperatorParameter::Feedback),
    Parameter::Operator(3, OperatorParameter::FrequencyRatio),
    Parameter::Operator(3, OperatorParameter::FrequencyFree),
    Parameter::Operator(3, OperatorParameter::FrequencyFine),
    Parameter::Operator(3, OperatorParameter::AttackDuration),
    Parameter::Operator(3, OperatorParameter::AttackValue),
    Parameter::Operator(3, OperatorParameter::DecayDuration),
    Parameter::Operator(3, OperatorParameter::DecayValue),
    Parameter::Operator(3, OperatorParameter::ReleaseDuration),
    Parameter::Lfo(0, LfoParameter::Target),
    Parameter::Lfo(0, LfoParameter::BpmSync),
    Parameter::Lfo(0, LfoParameter::FrequencyRatio),
    Parameter::Lfo(0, LfoParameter::FrequencyFree),
    Parameter::Lfo(0, LfoParameter::Mode),
    Parameter::Lfo(0, LfoParameter::Shape),
    Parameter::Lfo(0, LfoParameter::Amount),
    Parameter::Lfo(0, LfoParameter::Active),
    Parameter::Lfo(1, LfoParameter::Target),
    Parameter::Lfo(1, LfoParameter::BpmSync),
    Parameter::Lfo(1, LfoParameter::FrequencyRatio),
    Parameter::Lfo(1, LfoParameter::FrequencyFree),
    Parameter::Lfo(1, LfoParameter::Mode),
    Parameter::Lfo(1, LfoParameter::Shape),
    Parameter::Lfo(1, LfoParameter::Amount),
    Parameter::Lfo(1, LfoParameter::Active),
    Parameter::Lfo(2, LfoParameter::Target),
    Parameter::Lfo(2, LfoParameter::BpmSync),
    Parameter::Lfo(2, LfoParameter::FrequencyRatio),
    Parameter::Lfo(2, LfoParameter::FrequencyFree),
    Parameter::Lfo(2, LfoParameter::Mode),
    Parameter::Lfo(2, LfoParameter::Shape),
    Parameter::Lfo(2, LfoParameter::Amount),
    Parameter::Lfo(2, LfoParameter::Active),
    Parameter::Lfo(3, LfoParameter::Target),
    Parameter::Lfo(3, LfoParameter::BpmSync),
    Parameter::Lfo(3, LfoParameter::FrequencyRatio),
    Parameter::Lfo(3, LfoParameter::FrequencyFree),
    Parameter::Lfo(3, LfoParameter::Mode),
    Parameter::Lfo(3, LfoParameter::Shape),
    Parameter::Lfo(3, LfoParameter::Amount),
    Parameter::Lfo(3, LfoParameter::Active),
];

pub trait ParameterValue: Sized + Default {
    /// Value as used in audio generation
    type Value: Copy;

    /// Create new
    fn new_from_audio(value: Self::Value) -> Self;
    /// Create new from String
    fn new_from_text(_text: String) -> Option<Self> {
        None
    }
    /// Create new from patch value
    fn new_from_patch(value: f64) -> Self;

    /// Get inner (audio gen) value
    fn get(self) -> Self::Value;
    fn get_formatted(self) -> String;
    fn to_patch(self) -> f64;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Parameter {
    /// Only used in LFO targetting
    None,
    Master(MasterParameter),
    Operator(usize, OperatorParameter),
    Lfo(usize, LfoParameter),
}

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
                OperatorParameter::AttackValue => format!("OP {} attack vol", index + 1),
                OperatorParameter::DecayDuration => format!("OP {} decay time", index + 1),
                OperatorParameter::DecayValue => format!("OP {} decay vol", index + 1),
                OperatorParameter::ReleaseDuration => format!("OP {} release time", index + 1),
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

    pub fn to_index(&self) -> usize {
        for (index, p) in PARAMETERS.iter().enumerate() {
            if p == self {
                return index;
            }
        }

        panic!("Used parameter not in list")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MasterParameter {
    Volume,
    Frequency,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OperatorParameter {
    Volume,
    Active,
    MixOut,
    Panning,
    WaveType,
    ModTargets,
    ModOut,
    Feedback,
    FrequencyRatio,
    FrequencyFree,
    FrequencyFine,
    AttackDuration,
    AttackValue,
    DecayDuration,
    DecayValue,
    ReleaseDuration,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LfoParameter {
    Target,
    BpmSync,
    FrequencyRatio,
    FrequencyFree,
    Mode,
    Shape,
    Amount,
    Active,
}
