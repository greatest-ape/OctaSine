mod atomic_double;
mod change_info;
mod import_export;
mod import_export_utils;

use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};

use arc_swap::ArcSwap;
use array_init::array_init;

use crate::parameters::sync::create_parameters;
use crate::parameters::values::ParameterValue;

use atomic_double::AtomicPositiveDouble;
use change_info::ParameterChangeInfo;
use import_export::*;
use import_export_utils::*;

pub use change_info::MAX_NUM_PARAMETERS;

pub struct SyncParameter {
    value: AtomicPositiveDouble,
    name: String,
    sync_from_text: fn(String) -> Option<f64>,
    format_sync: fn(f64) -> String,
}

impl SyncParameter {
    pub fn new<V: ParameterValue>(name: &str, default: V) -> Self {
        Self {
            name: name.to_string(),
            value: AtomicPositiveDouble::new(default.to_sync()),
            sync_from_text: |v| V::from_text(v).map(|v| v.to_sync()),
            format_sync: |v| V::from_sync(v).format(),
        }
    }

    pub fn get_value(&self) -> f64 {
        self.value.get()
    }

    pub fn get_value_text(&self) -> String {
        (self.format_sync)(self.value.get())
    }

    pub fn set_from_text(&self, text: String) -> bool {
        if let Some(value) = (self.sync_from_text)(text) {
            self.value.set(value);

            true
        } else {
            false
        }
    }
}

struct Preset {
    name: ArcSwap<String>,
    parameters: Vec<SyncParameter>,
}

impl Default for Preset {
    fn default() -> Self {
        Self::new("-".to_string(), create_parameters())
    }
}

impl Preset {
    fn new(name: String, parameters: Vec<SyncParameter>) -> Self {
        Self {
            name: ArcSwap::new(Arc::new(name)),
            parameters,
        }
    }

    fn get_name(&self) -> String {
        (*self.name.load_full()).clone()
    }

    fn set_name(&self, name: String) {
        self.name.store(Arc::new(name));
    }

    fn import_bytes(&self, bytes: &[u8]) -> bool {
        let res_serde_preset: Result<SerdePreset, _> = from_bytes(bytes);

        if let Ok(serde_preset) = res_serde_preset {
            self.import_serde_preset(&serde_preset);

            true
        } else {
            false
        }
    }

    fn import_serde_preset(&self, serde_preset: &SerdePreset) {
        self.set_name(serde_preset.name.clone());

        for (index, parameter) in self.parameters.iter().enumerate() {
            if let Some(import_parameter) = serde_preset.parameters.get(index) {
                parameter.value.set(import_parameter.value_float.as_f64())
            }
        }
    }

    fn export_bytes(&self) -> Vec<u8> {
        to_bytes(&self.export_serde_preset())
    }

    fn export_serde_preset(&self) -> SerdePreset {
        SerdePreset::new(self)
    }
}

pub struct PresetBank {
    presets: [Preset; 128],
    preset_index: AtomicUsize,
    parameter_change_info_processing: ParameterChangeInfo,
    parameter_change_info_gui: ParameterChangeInfo,
    presets_changed: AtomicBool,
}

impl Default for PresetBank {
    fn default() -> Self {
        Self::new(create_parameters)
    }
}

impl PresetBank {
    pub fn new(parameters: fn() -> Vec<SyncParameter>) -> Self {
        Self {
            presets: array_init(|i| Preset::new(format!("{:03}", i + 1), parameters())),
            preset_index: AtomicUsize::new(0),
            parameter_change_info_processing: ParameterChangeInfo::default(),
            parameter_change_info_gui: ParameterChangeInfo::default(),
            presets_changed: AtomicBool::new(false),
        }
    }

    // Utils

    fn get_parameter(&self, index: usize) -> Option<&SyncParameter> {
        self.get_current_preset().parameters.get(index)
    }

    fn get_current_preset(&self) -> &Preset {
        &self.presets[self.get_preset_index()]
    }

    fn mark_parameters_as_changed(&self) {
        self.parameter_change_info_processing.mark_all_as_changed();
        self.parameter_change_info_gui.mark_all_as_changed();
    }

    // Number of presets / parameters

    pub fn num_presets(&self) -> usize {
        self.presets.len()
    }

    pub fn num_parameters(&self) -> usize {
        self.get_current_preset().parameters.len()
    }

    // Manage presets

    pub fn get_preset_index(&self) -> usize {
        self.preset_index.load(Ordering::SeqCst)
    }

    pub fn set_preset_index(&self, index: usize) {
        if index >= self.presets.len() {
            return;
        }

        self.preset_index.store(index, Ordering::SeqCst);
        self.presets_changed.store(true, Ordering::SeqCst);
        self.mark_parameters_as_changed();
    }

    pub fn get_preset_name(&self, index: usize) -> Option<String> {
        self.presets
            .get(index as usize)
            .map(|p| (*p.name.load_full()).clone())
    }

    pub fn get_preset_names(&self) -> Vec<String> {
        self.presets
            .iter()
            .map(|p| (*p.name.load_full()).clone())
            .collect()
    }

    pub fn set_preset_name(&self, name: String) {
        self.get_current_preset().name.store(Arc::new(name));
        self.presets_changed.store(true, Ordering::SeqCst);
    }

    /// Only used from GUI
    pub fn have_presets_changed(&self) -> bool {
        self.presets_changed.fetch_and(false, Ordering::SeqCst)
    }

    // Get parameter changes

    pub fn get_changed_parameters_from_processing(
        &self,
    ) -> Option<[Option<f64>; MAX_NUM_PARAMETERS]> {
        self.parameter_change_info_processing
            .get_changed_parameters(&self.get_current_preset().parameters)
    }

    pub fn get_changed_parameters_from_gui(&self) -> Option<[Option<f64>; MAX_NUM_PARAMETERS]> {
        self.parameter_change_info_gui
            .get_changed_parameters(&self.get_current_preset().parameters)
    }

    // Get parameter values

    pub fn get_parameter_value(&self, index: usize) -> Option<f64> {
        self.get_current_preset()
            .parameters
            .get(index)
            .map(|p| p.value.get())
    }

    pub fn get_parameter_value_text(&self, index: usize) -> Option<String> {
        self.get_current_preset()
            .parameters
            .get(index)
            .map(|p| (p.format_sync)(p.value.get()))
    }

    pub fn get_parameter_name(&self, index: usize) -> Option<String> {
        self.get_current_preset()
            .parameters
            .get(index)
            .map(|p| p.name.clone())
    }

    pub fn format_parameter_value(&self, index: usize, value: f64) -> Option<String> {
        self.get_current_preset()
            .parameters
            .get(index)
            .map(|p| (p.format_sync)(value))
    }

    // Set parameters

    pub fn set_parameter_from_gui(&self, index: usize, value: f64) {
        let opt_parameter = self.get_parameter(index);

        if let Some(parameter) = opt_parameter {
            parameter.value.set(value.min(1.0).max(0.0));

            self.parameter_change_info_processing.mark_as_changed(index);
        }
    }

    pub fn set_parameter_from_host(&self, index: usize, value: f64) {
        let opt_parameter = self.get_parameter(index);

        if let Some(parameter) = opt_parameter {
            parameter.value.set(value as f64);

            self.parameter_change_info_processing.mark_as_changed(index);
            self.parameter_change_info_gui.mark_as_changed(index);
        }
    }

    pub fn set_parameter_text_from_host(&self, index: usize, value: String) -> bool {
        let opt_parameter = self.get_parameter(index);

        if let Some(parameter) = opt_parameter {
            if parameter.set_from_text(value) {
                self.parameter_change_info_processing.mark_as_changed(index);
                self.parameter_change_info_gui.mark_as_changed(index);

                return true;
            }
        }

        false
    }

    // Import / export

    /// Import bytes into current bank, set sync parameters
    pub fn import_bank_from_bytes(&self, bytes: &[u8]) -> Result<(), impl ::std::error::Error> {
        let res_serde_preset_bank: Result<SerdePresetBank, _> = from_bytes(bytes);

        match res_serde_preset_bank {
            Ok(serde_preset_bank) => {
                let default_serde_preset = Preset::default().export_serde_preset();

                for (index, preset) in self.presets.iter().enumerate() {
                    if let Some(serde_preset) = serde_preset_bank.presets.get(index) {
                        preset.import_serde_preset(serde_preset);
                    } else {
                        preset.import_serde_preset(&default_serde_preset);
                        preset.set_name(format!("{:03}", index + 1));
                    }
                }

                self.set_preset_index(0);
                self.mark_parameters_as_changed();
                self.presets_changed.store(true, Ordering::SeqCst);

                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn import_bytes_into_current_preset(&self, bytes: &[u8]) {
        if self.get_current_preset().import_bytes(bytes) {
            self.mark_parameters_as_changed();
            self.presets_changed.store(true, Ordering::SeqCst);
        }
    }

    pub fn export_bank_as_bytes(&self) -> Vec<u8> {
        to_bytes(&SerdePresetBank::new(self))
    }

    pub fn export_current_preset_bytes(&self) -> Vec<u8> {
        self.get_current_preset().export_bytes()
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
    use assert_approx_eq::assert_approx_eq;

    use crate::built_in_preset_bank;

    use super::*;

    /// Test importing and exporting, as well as some related functionality
    #[test]
    #[allow(clippy::float_cmp)]
    pub fn test_export_import() {
        for _ in 0..20 {
            let bank_1 = PresetBank::default();

            for preset_index in 0..bank_1.num_presets() {
                bank_1.set_preset_index(preset_index);

                assert_eq!(bank_1.get_preset_index(), preset_index);

                let current_preset = bank_1.get_current_preset();

                for parameter_index in 0..current_preset.parameters.len() {
                    let parameter = current_preset.parameters.get(parameter_index).unwrap();

                    let value = fastrand::f64();

                    parameter.value.set(value);

                    assert_eq!(parameter.value.get(), value);
                }
            }

            let bank_2 = PresetBank::default();

            bank_2
                .import_bank_from_bytes(&bank_1.export_bank_as_bytes())
                .unwrap();

            for preset_index in 0..bank_1.num_presets() {
                bank_1.set_preset_index(preset_index);
                bank_2.set_preset_index(preset_index);

                let current_preset_1 = bank_1.get_current_preset();
                let current_preset_2 = bank_2.get_current_preset();

                for parameter_index in 0..current_preset_1.parameters.len() {
                    let parameter_1 = current_preset_1.parameters.get(parameter_index).unwrap();

                    let parameter_2 = current_preset_2.parameters.get(parameter_index).unwrap();

                    assert_eq!(parameter_1.value.get(), parameter_2.value.get(),);
                }
            }
        }
    }

    #[test]
    fn test_load_built_in_presets() {
        let preset_bank = built_in_preset_bank();

        // Hopefully prevent compiler from optimizing away code above (if it
        // actually ever did.)
        println!("Dummy info: {:?}", preset_bank.get_parameter_value(0));
    }

    /// Previous format used plain floats for value_float, so we need to check
    /// that (almost) the same values are deserialized no matter the format
    #[test]
    fn test_compare_preset_format_versions() {
        use assert_approx_eq::assert_approx_eq;

        let bank_1: PresetBank = PresetBank::new_from_bytes(include_bytes!(
            "../../presets/test-preset-bank-format-1.json"
        ));
        let bank_2: PresetBank = PresetBank::new_from_bytes(include_bytes!(
            "../../presets/test-preset-bank-format-2.json"
        ));

        assert_eq!(bank_1.num_presets(), bank_2.num_presets());

        for preset_index in 0..bank_1.num_presets() {
            bank_1.set_preset_index(preset_index);
            bank_2.set_preset_index(preset_index);

            assert_eq!(bank_1.num_parameters(), bank_2.num_parameters());

            for parameter_index in 0..bank_1.num_parameters() {
                assert_approx_eq!(
                    bank_1.get_parameter_value(parameter_index).unwrap(),
                    bank_2.get_parameter_value(parameter_index).unwrap(),
                    // Accept precision loss (probably due to
                    // JSON/javascript shenanigans)
                    0.0000000000000002
                );
            }
        }
    }
}
