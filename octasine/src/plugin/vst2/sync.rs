use std::path::PathBuf;
#[cfg(feature = "gui")]
use std::sync::Arc;

use compact_str::CompactString;
#[cfg(feature = "gui")]
use vst::host::Host;

use crate::{parameters::WrappedParameter, sync::SyncState};
#[cfg(feature = "gui")]
use crate::{settings::Settings, sync::change_info::MAX_NUM_PARAMETERS};

impl vst::plugin::PluginParameters for SyncState<vst::plugin::HostCallback> {
    /// Get parameter label for parameter at `index` (e.g. "db", "sec", "ms", "%").
    fn get_parameter_label(&self, _: i32) -> String {
        "".to_string()
    }

    /// Get the parameter value for parameter at `index` (e.g. "1.0", "150", "Plate", "Off").
    fn get_parameter_text(&self, index: i32) -> String {
        self.patches
            .get_parameter_value_text(index as usize)
            .map(String::from)
            .unwrap_or_else(|| "".to_string())
    }

    /// Get the name of parameter at `index`.
    fn get_parameter_name(&self, index: i32) -> String {
        self.patches
            .get_parameter_name(index as usize)
            .map(String::from)
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
            .set_parameter_text_from_host(index as usize, &text)
    }

    /// Return whether parameter at `index` can be automated.
    fn can_be_automated(&self, index: i32) -> bool {
        (index as usize) < self.patches.num_parameters()
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
        self.patches.set_patch_name(&name);
    }

    /// Get the name of the preset at the index specified by `preset`.
    fn get_preset_name(&self, index: i32) -> String {
        self.patches
            .get_patch_name(index as usize)
            .map(String::from)
            .unwrap_or_else(|| "".to_string())
    }

    /// If `preset_chunks` is set to true in plugin info, this should return the raw chunk data for
    /// the current preset.
    fn get_preset_data(&self) -> Vec<u8> {
        self.patches.get_current_patch().export_fxp_bytes()
    }

    /// If `preset_chunks` is set to true in plugin info, this should return the raw chunk data for
    /// the current plugin bank.
    fn get_bank_data(&self) -> Vec<u8> {
        self.patches.export_fxb_bytes()
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

#[cfg(feature = "gui")]
impl crate::sync::GuiSyncHandle for Arc<SyncState<vst::plugin::HostCallback>> {
    fn begin_edit(&self, parameter: WrappedParameter) {
        if let Some(host) = self.host {
            host.begin_edit(parameter.index() as i32);
        }
    }
    fn end_edit(&self, parameter: WrappedParameter) {
        if let Some(host) = self.host {
            host.end_edit(parameter.index() as i32);
        }
    }
    fn set_parameter(&self, parameter: WrappedParameter, value: f32) {
        let index = parameter.index() as usize;

        if let Some(host) = self.host {
            // Host will occasionally set the value again, but that's
            // ok
            host.automate(index as i32, value as f32);
        }

        self.patches.set_parameter_from_gui(index, value);
    }
    fn set_parameter_immediate(&self, parameter: WrappedParameter, value: f32) {
        if let Some(host) = self.host {
            let index = parameter.index() as i32;

            // Always wrapped in begin_edit and end_edit
            host.begin_edit(index);
            host.automate(index, value);
            host.end_edit(index);
        }

        self.patches
            .set_parameter_from_gui(parameter.index() as usize, value);
    }
    fn set_parameter_from_text(&self, parameter: WrappedParameter, text: &str) -> Option<f32> {
        let index = parameter.index() as usize;

        if self.patches.set_parameter_text_from_gui(index, text) {
            let value = self.patches.get_parameter_value(index).unwrap();

            if let Some(host) = self.host {
                host.begin_edit(index as i32);
                host.automate(index as i32, value);
                host.end_edit(index as i32);
            }

            Some(value)
        } else {
            None
        }
    }
    fn set_parameter_audio_only(&self, parameter: WrappedParameter, value: f32) {
        self.patches
            .set_parameter_from_gui(parameter.index() as usize, value);
    }
    fn get_parameter(&self, parameter: WrappedParameter) -> f32 {
        self.patches
            .get_parameter_value(parameter.index() as usize)
            .unwrap() // FIXME: unwrap
    }
    fn format_parameter_value(&self, parameter: WrappedParameter, value: f32) -> CompactString {
        self.patches
            .format_parameter_value(parameter.index() as usize, value)
            .unwrap() // FIXME: unwrap
    }
    fn get_patches(&self) -> (usize, Vec<CompactString>) {
        let index = self.patches.get_patch_index();
        let names = self.patches.get_patch_names();

        (index, names)
    }
    fn set_patch_index(&self, index: usize) {
        self.patches.set_patch_index(index);

        if let Some(host) = self.host {
            host.update_display();
        }
    }
    fn get_current_patch_name(&self) -> CompactString {
        self.patches.get_current_patch_name()
    }
    fn set_current_patch_name(&self, name: &str) {
        self.patches.set_patch_name(name);

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
        Settings::load_or_default().gui
    }
    fn export_patch(&self) -> (CompactString, Vec<u8>) {
        let name = self.patches.get_current_patch_filename_for_export();
        let data = self.patches.get_current_patch().export_fxp_bytes();

        (name, data)
    }
    fn export_bank(&self) -> Vec<u8> {
        self.patches.export_fxb_bytes()
    }
    fn import_bank_or_patches_from_paths(&self, paths: &[PathBuf]) {
        self.patches.import_bank_or_patches_from_paths(paths);

        if let Some(host) = self.host {
            host.update_display();
        }
    }
    fn clear_patch(&self) {
        self.patches.clear_current_patch();
    }
    fn clear_bank(&self) {
        self.patches.clear_bank();
    }
}
