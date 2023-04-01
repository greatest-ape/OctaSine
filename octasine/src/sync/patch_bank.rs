use std::{
    collections::VecDeque,
    io::Read,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
};

use arc_swap::ArcSwap;
use array_init::array_init;
use compact_str::{format_compact, CompactString};

use crate::{common::IndexMap, parameters::ParameterKey};

use super::change_info::{ParameterChangeInfo, MAX_NUM_PARAMETERS};
use super::parameters::PatchParameter;
use super::serde::*;

pub struct Patch {
    name: ArcSwap<String>,
    pub parameters: IndexMap<ParameterKey, PatchParameter>,
}

impl Default for Patch {
    fn default() -> Self {
        Self::new("-", PatchParameter::all())
    }
}

impl Patch {
    pub fn new(name: &str, parameters: IndexMap<ParameterKey, PatchParameter>) -> Self {
        Self {
            name: ArcSwap::new(Arc::new(Self::process_name(name))),
            parameters,
        }
    }

    pub fn get_fxp_filename(&self) -> CompactString {
        match self.name.load_full().as_str() {
            "" => "-.fxp".into(),
            name => format_compact!("{}.fxp", name),
        }
    }

    pub fn export_fxp_bytes(&self) -> Vec<u8> {
        serialize_patch_fxp_bytes(self).expect("serialize patch")
    }

    pub fn get_name(&self) -> String {
        (*self.name.load_full()).clone()
    }

    pub fn set_name(&self, name: &str) {
        self.name.store(Arc::new(Self::process_name(name)));
    }

    fn process_name(name: &str) -> String {
        name.chars().into_iter().filter(|c| c.is_ascii()).collect()
    }

    fn update_from_bytes(&self, bytes: &[u8]) -> anyhow::Result<()> {
        update_patch_from_bytes(self, bytes)
    }

    fn set_from_patch_parameters(&self, parameters: &IndexMap<ParameterKey, PatchParameter>) {
        self.set_name("".into());

        for (parameter, default_value) in self
            .parameters
            .values()
            .zip(parameters.values().map(PatchParameter::get_value))
        {
            parameter.set_value(default_value);
        }
    }
}

pub struct PatchBank {
    pub patches: [Patch; 128],
    patch_index: AtomicUsize,
    parameter_change_info_audio: ParameterChangeInfo,
    pub parameter_change_info_gui: ParameterChangeInfo,
    patches_changed: AtomicBool,
    envelope_viewports_changed: AtomicBool,
}

impl Default for PatchBank {
    fn default() -> Self {
        Self::new(PatchParameter::all)
    }
}

impl PatchBank {
    pub fn new(parameters: fn() -> IndexMap<ParameterKey, PatchParameter>) -> Self {
        Self {
            patches: array_init(|_| Patch::new("-", parameters())),
            patch_index: AtomicUsize::new(0),
            parameter_change_info_audio: ParameterChangeInfo::default(),
            parameter_change_info_gui: ParameterChangeInfo::default(),
            patches_changed: AtomicBool::new(false),
            envelope_viewports_changed: AtomicBool::new(false),
        }
    }

    // Utils

    pub fn get_parameter_by_index(&self, index: usize) -> Option<&PatchParameter> {
        self.get_current_patch()
            .parameters
            .get_index(index)
            .map(|(_, v)| v)
    }

    pub fn get_parameter_by_key(&self, key: &ParameterKey) -> Option<&PatchParameter> {
        self.get_current_patch().parameters.get(key)
    }

    pub fn get_index_and_parameter_by_key(
        &self,
        key: &ParameterKey,
    ) -> Option<(usize, &PatchParameter)> {
        self.get_current_patch()
            .parameters
            .get_full(key)
            .map(|(i, _, p)| (i, p))
    }

    pub fn get_current_patch(&self) -> &Patch {
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
}

// Manage patches
impl PatchBank {
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
        self.envelope_viewports_changed
            .store(true, Ordering::SeqCst);
    }

    pub fn get_patch_name(&self, index: usize) -> Option<CompactString> {
        self.patches
            .get(index as usize)
            .map(|p| format_compact!("{:03}: {}", index + 1, p.name.load_full()))
    }

    pub fn get_current_patch_name(&self) -> CompactString {
        self.get_current_patch().name.load_full().as_str().into()
    }

    pub fn get_patch_names(&self) -> Vec<CompactString> {
        self.patches
            .iter()
            .enumerate()
            .map(|(index, p)| format_compact!("{:03}: {}", index + 1, p.name.load_full()))
            .collect()
    }

    pub fn set_patch_name(&self, name: &str) {
        self.get_current_patch().set_name(name);
        self.patches_changed.store(true, Ordering::SeqCst);
    }

    /// Only used from GUI
    pub fn have_patches_changed(&self) -> bool {
        self.patches_changed.fetch_and(false, Ordering::SeqCst)
    }
}

// Get parameter changes
impl PatchBank {
    pub fn get_changed_parameters_from_audio(&self) -> Option<[Option<f32>; MAX_NUM_PARAMETERS]> {
        self.parameter_change_info_audio
            .get_changed_parameters(&self.get_current_patch().parameters)
    }

    pub fn get_changed_parameters_from_gui(&self) -> Option<[Option<f32>; MAX_NUM_PARAMETERS]> {
        self.parameter_change_info_gui
            .get_changed_parameters(&self.get_current_patch().parameters)
    }
}

// Get parameter values
impl PatchBank {
    pub fn get_parameter_value(&self, index: usize) -> Option<f32> {
        self.get_current_patch()
            .parameters
            .get_index(index)
            .map(|(_, p)| p.get_value())
    }

    pub fn get_parameter_value_text(&self, index: usize) -> Option<CompactString> {
        self.get_current_patch()
            .parameters
            .get_index(index)
            .map(|(_, p)| (p.get_value_text()))
    }

    pub fn get_parameter_name(&self, index: usize) -> Option<CompactString> {
        self.get_current_patch()
            .parameters
            .get_index(index)
            .map(|(_, p)| p.name.clone())
    }

    pub fn format_parameter_value(&self, index: usize, value: f32) -> Option<CompactString> {
        self.get_current_patch()
            .parameters
            .get_index(index)
            .map(|(_, p)| (p.format)(value))
    }
}

// Set parameters
impl PatchBank {
    pub fn set_parameter_from_gui(&self, index: usize, value: f32) {
        let opt_parameter = self.get_parameter_by_index(index);

        if let Some(parameter) = opt_parameter {
            parameter.set_value(value.min(1.0).max(0.0));

            self.parameter_change_info_audio.mark_as_changed(index);
        }
    }

    pub fn set_parameter_from_host(&self, index: usize, value: f32) {
        let opt_parameter = self.get_parameter_by_index(index);

        if let Some(parameter) = opt_parameter {
            parameter.set_value(value as f32);

            self.parameter_change_info_audio.mark_as_changed(index);
            self.parameter_change_info_gui.mark_as_changed(index);
        }
    }

    pub fn set_parameter_text_from_host(&self, index: usize, value: &str) -> bool {
        let opt_parameter = self.get_parameter_by_index(index);

        if let Some(parameter) = opt_parameter {
            if parameter.set_from_text(value) {
                self.parameter_change_info_audio.mark_as_changed(index);
                self.parameter_change_info_gui.mark_as_changed(index);

                return true;
            }
        }

        false
    }

    pub fn set_parameter_text_from_gui(&self, index: usize, value: &str) -> bool {
        let opt_parameter = self.get_parameter_by_index(index);

        if let Some(parameter) = opt_parameter {
            if parameter.set_from_text(value) {
                self.parameter_change_info_audio.mark_as_changed(index);

                return true;
            }
        }

        false
    }
}

// Import / export
impl PatchBank {
    pub fn import_bank_or_patches_from_paths(&self, paths: &[PathBuf]) {
        let mut bank_file_bytes = Vec::new();
        let mut patch_file_bytes = VecDeque::new();

        for path in paths {
            match read_file(&path) {
                Ok(bytes) => match path.extension().and_then(|s| s.to_str()) {
                    Some("fxb") => {
                        bank_file_bytes.push(bytes);
                    }
                    Some("fxp") => {
                        patch_file_bytes.push_back(bytes);
                    }
                    _ => {
                        ::log::warn!("Ignored file without fxp or fxb file extension");
                    }
                },
                Err(err) => ::log::warn!(
                    "Failed loading bank / patch bank from file {}: {:#}",
                    path.display(),
                    err
                ),
            };
        }

        match bank_file_bytes.pop() {
            Some(bank_bytes) => {
                if let Err(err) = self.import_bank_from_bytes(&bank_bytes) {
                    ::log::error!("failed importing patch bank: {:#}", err);
                }
            }
            None => {
                // Import serde patches into current and following patches
                let mut patch_iterator = self.patches[self.get_patch_index()..].iter().peekable();

                for patch_bytes in patch_file_bytes {
                    if patch_iterator.peek().is_none() {
                        break;
                    }

                    patch_iterator.next_if(|patch| {
                        if let Err(err) = patch.update_from_bytes(&patch_bytes) {
                            ::log::error!("failed importing patch: {:#}", err);

                            false
                        } else {
                            true
                        }
                    });
                }

                self.mark_parameters_as_changed();
                self.patches_changed.store(true, Ordering::SeqCst);
                self.envelope_viewports_changed
                    .store(true, Ordering::SeqCst);
            }
        }
    }

    /// Import bytes into current bank, set sync parameters
    pub fn import_bank_from_bytes(&self, bytes: &[u8]) -> anyhow::Result<()> {
        match update_bank_from_bytes(self, bytes) {
            Ok(()) => {
                self.set_patch_index(0);
                self.mark_parameters_as_changed();
                self.patches_changed.store(true, Ordering::SeqCst);
                self.envelope_viewports_changed
                    .store(true, Ordering::SeqCst);

                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn import_bytes_into_current_patch(&self, bytes: &[u8]) {
        match self.get_current_patch().update_from_bytes(bytes) {
            Ok(()) => {
                self.mark_parameters_as_changed();
                self.patches_changed.store(true, Ordering::SeqCst);
                self.envelope_viewports_changed
                    .store(true, Ordering::SeqCst);
            }
            Err(err) => {
                ::log::warn!("failed importing bytes into current patch: {:#}", err);
            }
        }
    }

    pub fn export_plain_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        serialize_bank_plain_bytes(&mut buffer, self).expect("serialize preset bank");

        buffer
    }

    pub fn export_fxb_bytes(&self) -> Vec<u8> {
        serialize_bank_fxb_bytes(self).expect("serialize preset bank")
    }

    pub fn new_from_bytes(bytes: &[u8]) -> Self {
        let preset_bank = Self::default();

        preset_bank
            .import_bank_from_bytes(bytes)
            .expect("import bank from bytes");

        preset_bank
    }
}

// Clear data
impl PatchBank {
    pub fn clear_current_patch(&self) {
        self.get_current_patch()
            .set_from_patch_parameters(&PatchParameter::all());

        self.mark_parameters_as_changed();
        self.patches_changed.store(true, Ordering::SeqCst);
        self.envelope_viewports_changed
            .store(true, Ordering::SeqCst);
    }

    pub fn clear_bank(&self) {
        let default_parameters = PatchParameter::all();

        for patch in self.patches.iter() {
            patch.set_from_patch_parameters(&default_parameters);
        }

        self.set_patch_index(0);

        self.mark_parameters_as_changed();
        self.patches_changed.store(true, Ordering::SeqCst);
        self.envelope_viewports_changed
            .store(true, Ordering::SeqCst);
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
        fastrand::seed(123);

        for _ in 0..50 {
            let bank_1 = PatchBank::default();

            for (patch_index, patch) in bank_1.patches.iter().enumerate() {
                bank_1.set_patch_index(patch_index);

                assert_eq!(bank_1.get_patch_index(), patch_index);
                assert_eq!(bank_1.get_current_patch().get_name(), patch.get_name());

                for parameter in patch.parameters.values() {
                    let value = fastrand::f32();

                    parameter.set_value(value);

                    assert_eq!(parameter.get_value(), value);
                }
            }

            let bank_2 = PatchBank::new_from_bytes(&bank_1.export_fxb_bytes());
            let bank_3 = PatchBank::new_from_bytes(&bank_1.export_plain_bytes());

            for ((patch_1, patch_2), patch_3) in bank_1
                .patches
                .iter()
                .zip(bank_2.patches.iter())
                .zip(bank_3.patches.iter())
            {
                for ((p1, p2), p3) in patch_1
                    .parameters
                    .values()
                    .zip(patch_2.parameters.values())
                    .zip(patch_3.parameters.values())
                {
                    let values = [p1, p2, p3]
                        .into_iter()
                        .map(|p| (p.get_value(), p.get_value_text()))
                        .collect::<Vec<_>>();

                    assert_eq!(values[0], values[1]);
                    assert_eq!(values[0], values[2]);
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

fn read_file(path: &::std::path::Path) -> anyhow::Result<Vec<u8>> {
    let mut file = ::std::fs::File::open(path)?;
    let mut bytes = Vec::new();

    file.read_to_end(&mut bytes)?;

    Ok(bytes)
}
