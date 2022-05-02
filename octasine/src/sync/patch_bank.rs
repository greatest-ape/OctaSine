use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};

use arc_swap::ArcSwap;
use array_init::array_init;

use super::change_info::{ParameterChangeInfo, MAX_NUM_PARAMETERS};
use super::parameters::patch_parameters;
use super::parameters::PatchParameter;
use super::serde::*;

pub struct Patch {
    name: ArcSwap<String>,
    pub parameters: Vec<PatchParameter>,
}

impl Default for Patch {
    fn default() -> Self {
        Self::new("-", patch_parameters())
    }
}

impl Patch {
    pub fn new(name: &str, parameters: Vec<PatchParameter>) -> Self {
        Self {
            name: ArcSwap::new(Arc::new(name.to_owned())),
            parameters,
        }
    }

    pub fn get_name(&self) -> String {
        (*self.name.load_full()).clone()
    }

    pub fn set_name(&self, name: String) {
        self.name.store(Arc::new(name));
    }

    pub fn import_bytes(&self, bytes: &[u8]) -> bool {
        let res_serde_preset: Result<SerdePatch, _> = from_bytes(bytes);

        if let Ok(serde_preset) = res_serde_preset {
            self.import_serde_preset(&serde_preset);

            true
        } else {
            false
        }
    }

    pub fn import_serde_preset(&self, serde_preset: &SerdePatch) {
        self.set_name(serde_preset.name.clone());

        for (index, parameter) in self.parameters.iter().enumerate() {
            if let Some(import_parameter) = serde_preset.parameters.get(index) {
                parameter.set_value(import_parameter.value_float.as_f64())
            }
        }
    }

    pub fn export_bytes(&self) -> Vec<u8> {
        to_bytes(&self.export_serde_preset()).expect("serialize preset")
    }

    pub fn export_serde_preset(&self) -> SerdePatch {
        SerdePatch::new(self)
    }
}

pub struct PatchBank {
    pub patches: [Patch; 128],
    patch_index: AtomicUsize,
    parameter_change_info_audio: ParameterChangeInfo,
    parameter_change_info_gui: ParameterChangeInfo,
    patches_changed: AtomicBool,
}

impl Default for PatchBank {
    fn default() -> Self {
        Self::new(patch_parameters)
    }
}

impl PatchBank {
    pub fn new(parameters: fn() -> Vec<PatchParameter>) -> Self {
        Self {
            patches: array_init(|_| Patch::new("-", parameters())),
            patch_index: AtomicUsize::new(0),
            parameter_change_info_audio: ParameterChangeInfo::default(),
            parameter_change_info_gui: ParameterChangeInfo::default(),
            patches_changed: AtomicBool::new(false),
        }
    }

    // Utils

    fn get_parameter(&self, index: usize) -> Option<&PatchParameter> {
        self.get_current_patch().parameters.get(index)
    }

    fn get_current_patch(&self) -> &Patch {
        &self.patches[self.get_patch_index()]
    }

    fn mark_parameters_as_changed(&self) {
        self.parameter_change_info_audio.mark_all_as_changed();
        self.parameter_change_info_gui.mark_all_as_changed();
    }

    // Number of patches / parameters

    pub fn num_patches(&self) -> usize {
        self.patches.len()
    }

    pub fn num_parameters(&self) -> usize {
        self.get_current_patch().parameters.len()
    }

    // Manage patches

    pub fn get_patch_index(&self) -> usize {
        self.patch_index.load(Ordering::SeqCst)
    }

    pub fn set_patch_index(&self, index: usize) {
        if index >= self.patches.len() {
            return;
        }

        self.patch_index.store(index, Ordering::SeqCst);
        self.patches_changed.store(true, Ordering::SeqCst);
        self.mark_parameters_as_changed();
    }

    pub fn get_patch_name(&self, index: usize) -> Option<String> {
        self.patches
            .get(index as usize)
            .map(|p| format!("{:03}: {}", index + 1, p.name.load_full()))
    }

    pub fn get_patch_names(&self) -> Vec<String> {
        self.patches
            .iter()
            .enumerate()
            .map(|(index, p)| format!("{:03}: {}", index + 1, p.name.load_full()))
            .collect()
    }

    pub fn set_patch_name(&self, name: String) {
        self.get_current_patch().name.store(Arc::new(name));
        self.patches_changed.store(true, Ordering::SeqCst);
    }

    /// Only used from GUI
    pub fn have_patches_changed(&self) -> bool {
        self.patches_changed.fetch_and(false, Ordering::SeqCst)
    }

    // Get parameter changes

    pub fn get_changed_parameters_from_audio(&self) -> Option<[Option<f64>; MAX_NUM_PARAMETERS]> {
        self.parameter_change_info_audio
            .get_changed_parameters(&self.get_current_patch().parameters)
    }

    pub fn get_changed_parameters_from_gui(&self) -> Option<[Option<f64>; MAX_NUM_PARAMETERS]> {
        self.parameter_change_info_gui
            .get_changed_parameters(&self.get_current_patch().parameters)
    }

    // Get parameter values

    pub fn get_parameter_value(&self, index: usize) -> Option<f64> {
        self.get_current_patch()
            .parameters
            .get(index)
            .map(|p| p.get_value())
    }

    pub fn get_parameter_value_text(&self, index: usize) -> Option<String> {
        self.get_current_patch()
            .parameters
            .get(index)
            .map(|p| (p.get_value_text()))
    }

    pub fn get_parameter_name(&self, index: usize) -> Option<String> {
        self.get_current_patch()
            .parameters
            .get(index)
            .map(|p| p.name.clone())
    }

    pub fn format_parameter_value(&self, index: usize, value: f64) -> Option<String> {
        self.get_current_patch()
            .parameters
            .get(index)
            .map(|p| (p.format)(value))
    }

    // Set parameters

    pub fn set_parameter_from_gui(&self, index: usize, value: f64) {
        let opt_parameter = self.get_parameter(index);

        if let Some(parameter) = opt_parameter {
            parameter.set_value(value.min(1.0).max(0.0));

            self.parameter_change_info_audio.mark_as_changed(index);
        }
    }

    pub fn set_parameter_from_host(&self, index: usize, value: f64) {
        let opt_parameter = self.get_parameter(index);

        if let Some(parameter) = opt_parameter {
            parameter.set_value(value as f64);

            self.parameter_change_info_audio.mark_as_changed(index);
            self.parameter_change_info_gui.mark_as_changed(index);
        }
    }

    pub fn set_parameter_text_from_host(&self, index: usize, value: String) -> bool {
        let opt_parameter = self.get_parameter(index);

        if let Some(parameter) = opt_parameter {
            if parameter.set_from_text(value) {
                self.parameter_change_info_audio.mark_as_changed(index);
                self.parameter_change_info_gui.mark_as_changed(index);

                return true;
            }
        }

        false
    }

    // Import / export

    /// Import bytes into current bank, set sync parameters
    pub fn import_bank_from_bytes(&self, bytes: &[u8]) -> Result<(), impl ::std::error::Error> {
        let res_serde_preset_bank: Result<SerdePatchBank, _> = from_bytes(bytes);

        match res_serde_preset_bank {
            Ok(serde_preset_bank) => {
                let default_serde_preset = Patch::default().export_serde_preset();

                for (index, preset) in self.patches.iter().enumerate() {
                    if let Some(serde_preset) = serde_preset_bank.patches.get(index) {
                        preset.import_serde_preset(serde_preset);
                    } else {
                        preset.import_serde_preset(&default_serde_preset);
                        preset.set_name(format!("{:03}", index + 1));
                    }
                }

                self.set_patch_index(0);
                self.mark_parameters_as_changed();
                self.patches_changed.store(true, Ordering::SeqCst);

                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn import_bytes_into_current_patch(&self, bytes: &[u8]) {
        if self.get_current_patch().import_bytes(bytes) {
            self.mark_parameters_as_changed();
            self.patches_changed.store(true, Ordering::SeqCst);
        }
    }

    pub fn export_bank_as_bytes(&self) -> Vec<u8> {
        to_bytes(&SerdePatchBank::new(self)).expect("serialize preset bank")
    }

    pub fn export_current_patch_bytes(&self) -> Vec<u8> {
        self.get_current_patch().export_bytes()
    }

    pub fn new_from_bytes(bytes: &[u8]) -> Self {
        let preset_bank = Self::default();

        preset_bank
            .import_bank_from_bytes(bytes)
            .expect("import bank from bytes");

        preset_bank
    }
}

#[cfg(test)]
pub mod tests {
    use crate::sync::built_in_patch_bank;

    use super::*;

    /// Test importing and exporting, as well as some related functionality
    #[test]
    #[allow(clippy::float_cmp)]
    pub fn test_export_import() {
        for _ in 0..20 {
            let bank_1 = PatchBank::default();

            for preset_index in 0..bank_1.num_patches() {
                bank_1.set_patch_index(preset_index);

                assert_eq!(bank_1.get_patch_index(), preset_index);

                let current_preset = bank_1.get_current_patch();

                for parameter_index in 0..current_preset.parameters.len() {
                    let parameter = current_preset.parameters.get(parameter_index).unwrap();

                    let value = fastrand::f64();

                    parameter.set_value(value);

                    assert_eq!(parameter.get_value(), value);
                }
            }

            let bank_2 = PatchBank::default();

            bank_2
                .import_bank_from_bytes(&bank_1.export_bank_as_bytes())
                .unwrap();

            for preset_index in 0..bank_1.num_patches() {
                bank_1.set_patch_index(preset_index);
                bank_2.set_patch_index(preset_index);

                let current_preset_1 = bank_1.get_current_patch();
                let current_preset_2 = bank_2.get_current_patch();

                for parameter_index in 0..current_preset_1.parameters.len() {
                    let parameter_1 = current_preset_1.parameters.get(parameter_index).unwrap();

                    let parameter_2 = current_preset_2.parameters.get(parameter_index).unwrap();

                    assert_eq!(parameter_1.get_value(), parameter_2.get_value());
                }
            }
        }
    }

    #[test]
    fn test_load_built_in_patches() {
        let preset_bank = built_in_patch_bank();

        // Hopefully prevent compiler from optimizing away code above (if it
        // actually ever did.)
        println!("Dummy info: {:?}", preset_bank.get_parameter_value(0));
    }
}
