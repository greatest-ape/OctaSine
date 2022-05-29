mod atomic_double;
mod change_info;
mod parameters;
mod patch_bank;
pub mod serde;

use std::sync::Arc;

use vst::host::Host;
use vst::plugin::HostCallback;

use crate::common::*;
use crate::settings::Settings;

use patch_bank::PatchBank;

/// Thread-safe state used for parameter and preset calls
pub struct SyncState {
    /// Host should always be set when running as real plugin, but having the
    /// option of leaving this field empty is useful when benchmarking.
    host: Option<HostCallback>,
    pub patches: PatchBank,
    settings: Settings,
}

impl SyncState {
    pub fn new(host: Option<HostCallback>) -> Self {
        Self {
            host,
            patches: built_in_patch_bank(),
            settings: Settings::load_or_default(),
        }
    }

    pub fn get_bpm_from_host(&self) -> Option<BeatsPerMinute> {
        // Use TEMPO_VALID constant content as mask directly because
        // of problems with using TimeInfoFlags
        let mask = 1 << 10;

        let time_info = self.host?.get_time_info(mask)?;

        if (time_info.flags & mask) != 0 {
            Some(BeatsPerMinute(time_info.tempo as f64))
        } else {
            None
        }
    }
}

impl vst::plugin::PluginParameters for SyncState {
    /// Get parameter label for parameter at `index` (e.g. "db", "sec", "ms", "%").
    fn get_parameter_label(&self, _: i32) -> String {
        "".to_string()
    }

    /// Get the parameter value for parameter at `index` (e.g. "1.0", "150", "Plate", "Off").
    fn get_parameter_text(&self, index: i32) -> String {
        self.patches
            .get_parameter_value_text(index as usize)
            .unwrap_or_else(|| "".to_string())
    }

    /// Get the name of parameter at `index`.
    fn get_parameter_name(&self, index: i32) -> String {
        self.patches
            .get_parameter_name(index as usize)
            .unwrap_or_else(|| "".to_string())
    }

    /// Get the value of paramater at `index`. Should be value between 0.0 and 1.0.
    fn get_parameter(&self, index: i32) -> f32 {
        self.patches
            .get_parameter_value(index as usize)
            .unwrap_or(0.0) as f32
    }

    /// Set the value of parameter at `index`. `value` is between 0.0 and 1.0.
    fn set_parameter(&self, index: i32, value: f32) {
        self.patches.set_parameter_from_host(index as usize, value);
    }

    /// Use String as input for parameter value. Used by host to provide an editable field to
    /// adjust a parameter value. E.g. "100" may be interpreted as 100hz for parameter. Returns if
    /// the input string was used.
    fn string_to_parameter(&self, index: i32, text: String) -> bool {
        self.patches
            .set_parameter_text_from_host(index as usize, text)
    }

    /// Return whether parameter at `index` can be automated.
    fn can_be_automated(&self, index: i32) -> bool {
        self.patches.num_parameters() < index as usize
    }

    /// Set the current preset to the index specified by `preset`.
    ///
    /// This method can be called on the processing thread for automation.
    fn change_preset(&self, index: i32) {
        self.patches.set_patch_index(index as usize);
    }

    /// Get the current preset index.
    fn get_preset_num(&self) -> i32 {
        self.patches.get_patch_index() as i32
    }

    /// Set the current preset name.
    fn set_preset_name(&self, name: String) {
        self.patches.set_patch_name(name);
    }

    /// Get the name of the preset at the index specified by `preset`.
    fn get_preset_name(&self, index: i32) -> String {
        self.patches
            .get_patch_name(index as usize)
            .unwrap_or_else(|| "".to_string())
    }

    /// If `preset_chunks` is set to true in plugin info, this should return the raw chunk data for
    /// the current preset.
    fn get_preset_data(&self) -> Vec<u8> {
        self.patches.export_current_patch_bytes()
    }

    /// If `preset_chunks` is set to true in plugin info, this should return the raw chunk data for
    /// the current plugin bank.
    fn get_bank_data(&self) -> Vec<u8> {
        self.patches.export_bank_as_bytes()
    }

    /// If `preset_chunks` is set to true in plugin info, this should load a preset from the given
    /// chunk data.
    fn load_preset_data(&self, data: &[u8]) {
        self.patches.import_bytes_into_current_patch(data);
    }

    /// If `preset_chunks` is set to true in plugin info, this should load a preset bank from the
    /// given chunk data.
    fn load_bank_data(&self, data: &[u8]) {
        if let Err(err) = self.patches.import_bank_from_bytes(data) {
            ::log::error!("Couldn't load bank data: {}", err)
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "gui")] {
        use self::change_info::MAX_NUM_PARAMETERS;
        use crate::parameters::Parameter;

        /// Trait passed to GUI code for encapsulation
        pub trait GuiSyncHandle: Clone + Send + Sync + 'static {
            fn begin_edit(&self, parameter: Parameter);
            fn end_edit(&self, parameter: Parameter);
            fn set_parameter(&self, parameter: Parameter, value: f32);
            fn get_parameter(&self, parameter: Parameter) -> f32;
            fn format_parameter_value(&self, parameter: Parameter, value: f32) -> String;
            fn get_patches(&self) -> (usize, Vec<String>);
            fn set_patch_index(&self, index: usize);
            fn get_changed_parameters(&self) -> Option<[Option<f32>; MAX_NUM_PARAMETERS]>;
            fn have_patches_changed(&self) -> bool;
            fn get_gui_settings(&self) -> crate::gui::GuiSettings;
        }

        impl GuiSyncHandle for Arc<SyncState> {
            fn begin_edit(&self, parameter: Parameter) {
                if let Some(host) = self.host {
                    host.begin_edit(parameter.to_index() as i32);
                }
            }
            fn end_edit(&self, parameter: Parameter) {
                if let Some(host) = self.host {
                    host.end_edit(parameter.to_index() as i32);
                }
            }
            fn set_parameter(&self, parameter: Parameter, value: f32){
                let index = parameter.to_index() as usize;

                if let Some(host) = self.host {
                    // Host will occasionally set the value again, but that's
                    // ok
                    host.automate(index as i32, value as f32);
                }

                self.patches.set_parameter_from_gui(index, value);
            }
            fn get_parameter(&self, parameter: Parameter) -> f32 {
                self.patches.get_parameter_value(parameter.to_index() as usize).unwrap() // FIXME: unwrap
            }
            fn format_parameter_value(&self, parameter: Parameter, value: f32) -> String {
                self.patches.format_parameter_value(parameter.to_index() as usize, value).unwrap() // FIXME: unwrap
            }
            fn get_patches(&self) -> (usize, Vec<String>){
                let index = self.patches.get_patch_index();
                let names = self.patches.get_patch_names();

                (index, names)
            }
            fn set_patch_index(&self, index: usize){
                self.patches.set_patch_index(index);

                if let Some(host) = self.host {
                    host.update_display();
                }
            }
            fn get_changed_parameters(&self) -> Option<[Option<f32>; MAX_NUM_PARAMETERS]> {
                self.patches.get_changed_parameters_from_gui()
            }
            fn have_patches_changed(&self) -> bool {
                self.patches.have_patches_changed()
            }
            fn get_gui_settings(&self) -> crate::gui::GuiSettings {
                self.settings.gui.clone()
            }
        }
    }
}

fn built_in_patch_bank() -> PatchBank {
    PatchBank::default()
}
