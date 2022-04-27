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
pub use operator_mix_out::OperatorMixValue;
pub use operator_mod_out::OperatorModulationIndexValue;
pub use operator_mod_target::*;
pub use operator_panning::OperatorPanningValue;
pub use operator_volume::OperatorVolumeValue;
pub use operator_wave_type::OperatorWaveTypeValue;

pub trait ParameterValue: Sized {
    /// Value as used in audio generation
    type Value: Copy;

    /// Create new ParameterValue
    fn from_audio(value: Self::Value) -> Self;
    /// Create new ParameterValue from text
    fn from_text(_text: String) -> Option<Self> {
        None
    }

    /// Get inner (audio gen) value
    fn get(self) -> Self::Value;
    /// Get inner value, formatted to a String
    fn get_formatted(self) -> String;

    /// Create new ParameterValue from patch value
    fn from_patch(value: f64) -> Self;
    /// Convert ParameterValue to patch value
    fn to_patch(self) -> f64;

    fn convert_patch_to_audio_formatted(value: f64) -> String {
        Self::from_patch(value).get_formatted()
    }
    fn convert_patch_to_audio(value: f64) -> Self::Value {
        Self::from_patch(value).get()
    }
    fn convert_audio_to_patch(value: Self::Value) -> f64 {
        Self::from_audio(value).to_patch()
    }
}
