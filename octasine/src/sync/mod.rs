mod atomic_float;
pub mod change_info;
mod parameters;
mod patch_bank;
mod serde;

use std::path::PathBuf;

use compact_str::CompactString;
use patch_bank::PatchBank;

/// Thread-safe state used for parameter and preset calls
pub struct SyncState<H> {
    /// Host should always be set when running as real plugin, but having the
    /// option of leaving this field empty is useful when benchmarking.
    pub host: Option<H>,
    pub patches: PatchBank,
}

impl<H> SyncState<H> {
    pub fn new(host: Option<H>) -> Self {
        Self {
            host,
            patches: built_in_patch_bank(),
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "gui")] {
        use crate::parameters::WrappedParameter;
        use self::change_info::MAX_NUM_PARAMETERS;

        #[derive(Clone, Copy)]
        pub struct EnvelopeViewport {
            pub x_offset: f32,
            pub viewport_factor: f32,
        }

        /// Trait passed to GUI code for encapsulation
        pub trait GuiSyncHandle: Clone + Send + Sync + 'static {
            fn begin_edit(&self, parameter: WrappedParameter);
            fn end_edit(&self, parameter: WrappedParameter);
            fn set_parameter(&self, parameter: WrappedParameter, value: f32);
            /// Set parameter immediately. Wrap in begin and end edit commands if necessary
            fn set_parameter_immediate(&self, parameter: WrappedParameter, value: f32);
            fn set_parameter_from_text(&self, parameter: WrappedParameter, text: &str) -> Option<f32>;
            /// Set parameter without telling host
            fn set_parameter_audio_only(&self, parameter: WrappedParameter, value: f32);
            fn set_envelope_viewport(&self, operator_index: usize, viewport: EnvelopeViewport);
            fn get_parameter(&self, parameter: WrappedParameter) -> f32;
            fn format_parameter_value(&self, parameter: WrappedParameter, value: f32) -> CompactString;
            fn get_patches(&self) -> (usize, Vec<CompactString>);
            fn set_patch_index(&self, index: usize);
            fn get_current_patch_name(&self) -> CompactString;
            fn set_current_patch_name(&self, name: &str);
            fn get_changed_parameters(&self) -> Option<[Option<f32>; MAX_NUM_PARAMETERS]>;
            fn get_viewports_if_changed(&self) -> Option<[EnvelopeViewport; 4]>;
            fn have_patches_changed(&self) -> bool;
            fn get_gui_settings(&self) -> crate::gui::GuiSettings;
            fn export_patch(&self) -> (CompactString, Vec<u8>);
            fn export_bank(&self) -> Vec<u8>;
            fn import_bank_or_patches_from_paths(&self, paths: &[PathBuf]);
            fn clear_patch(&self);
            fn clear_bank(&self);
        }
    }
}

fn built_in_patch_bank() -> PatchBank {
    PatchBank::default()
}
