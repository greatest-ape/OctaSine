use std::sync::atomic::{AtomicUsize, Ordering};

use array_init::array_init;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};

pub mod parameters;
pub mod change_info;

pub use parameters::PresetParameters;
pub use change_info::ParameterChangeInfo;


#[allow(unused_macros)]
macro_rules! preset_from_file {
    ($path:expr) => {
        Preset::new_from_bytes(
            include_bytes!($path)
        ).expect(&format!("Couldn't load preset file: {}", $path))
    };
}


#[allow(clippy::let_and_return)]
#[allow(unused_mut)]
fn built_in_presets() -> Vec<Preset> {
    let mut presets = Vec::new();

    // presets.push(preset_from_file!("../presets/test.fxp"));

    presets
}


fn to_bytes<T: Serialize>(t: &T) -> Vec<u8> {
    let mut out = "\n\n".to_string().as_bytes().to_vec();

    out.append(&mut serde_json::to_vec_pretty(t)
        .expect("presets module: couldn't serialize"));
    
    out
}


fn from_bytes<'a, T: Deserialize<'a>>(
    bytes: &'a [u8]
) -> Result<T, impl ::std::error::Error> {
    serde_json::from_slice(bytes)
}


pub struct Preset {
    /// Preset name wrapped in a RwLock. Hopefully, the audio processing
    /// thread won't access it
    name: RwLock<String>,
    parameters: PresetParameters,
}


impl Preset {
    #[allow(dead_code)]
    fn new_from_bytes(bytes: &[u8]) -> Result<Self, impl ::std::error::Error> {
        let res_serde_preset: Result<SerdePreset, _> = from_bytes(bytes);

        res_serde_preset.map(|serde_preset| {
            let preset = Self::default();

            preset.import_serde_preset(&serde_preset);

            preset
        })
    }

    fn get_name(&self) -> String {
        self.name.read().clone()
    }

    fn set_name(&self, name: String) {
        *self.name.write() = name;
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

    fn import_serde_preset(&self, serde_preset: &SerdePreset){
        for index in 0..self.parameters.len() {
            if let Some(import_parameter) = serde_preset.parameters.get(index){
                if let Some(parameter) = self.parameters.get(index){
                    parameter.set_parameter_value_float(
                        import_parameter.value_float
                    );
                }
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


impl Default for Preset {
    fn default() -> Self {
        Self {
            name: RwLock::new("-".to_string()),
            parameters: PresetParameters::new(),
        }
    }
}


pub struct PresetBank {
    presets: [Preset; 128],
    preset_index: AtomicUsize,
    pub parameter_change_info: ParameterChangeInfo,
}


impl PresetBank {
    pub fn new() -> Self {
        let mut presets: [Preset; 128] = array_init(|_| Preset::default());
        let built_in_presets = built_in_presets();

        for (preset, built_in_preset) in presets.iter_mut()
            .zip(built_in_presets.into_iter())
        {
            *preset = built_in_preset;
        }

        Self {
            presets,
            preset_index: AtomicUsize::new(0),
            parameter_change_info: ParameterChangeInfo::new(),
        }
    }
    
    pub fn len(&self) -> usize {
        self.presets.len()
    }

    pub fn get_changed_parameters(&self) -> Option<[Option<f32>; 64]> {
        self.parameter_change_info.get_changed_parameters(
            &self.get_current_preset().parameters
        )
    }

    fn mark_parameters_as_changed(&self){
        self.parameter_change_info.mark_all_as_changed();
    }

    fn get_current_preset(&self) -> &Preset {
        &self.presets[self.get_preset_index()]
    }
}

/// PresetBank preset call handling
impl PresetBank {
    pub fn set_preset_index(&self, index: usize){
        if index >= self.len(){
            return;
        }

        self.preset_index.store(index, Ordering::SeqCst);
    }

    pub fn get_preset_index(&self) -> usize {
        self.preset_index.load(Ordering::SeqCst)
    }

    pub fn get_preset_name_by_index(&self, index: usize) -> String {
        self.presets.get(index)
            .map_or("".to_string(), |preset| preset.get_name())
    }

    pub fn set_current_preset_name(&self, name: String){
        self.get_current_preset().set_name(name)
    }
}


/// PresetBank parameter call handling
impl PresetBank {
    pub fn get_num_parameters(&self) -> usize {
        self.get_current_preset().parameters.len()
    }

    pub fn get_parameter_name(&self, index: usize) -> String {
        self.get_current_preset().parameters.get(index as usize).map_or(
            "".to_string(),
            |parameter| parameter.get_parameter_name()
        )
    }

    pub fn get_parameter_unit(&self, index: usize) -> String {
        self.get_current_preset().parameters.get(index as usize).map_or(
            "".to_string(),
            |parameter| parameter.get_parameter_unit_of_measurement()
        )
    }

    pub fn can_parameter_be_automated(&self, index: usize) -> bool {
        self.get_current_preset().parameters.get(index as usize).is_some()
    }

    pub fn get_parameter_value_text(&self, index: usize) -> String {
        self.get_current_preset().parameters.get(index as usize).map_or(
            "".to_string(),
            |parameter| parameter.get_parameter_value_text()
        )
    }

    pub fn get_parameter_value_float(&self, index: usize) -> f32 {
        self.get_current_preset().parameters.get(index as usize)
            .map_or(0.0, |p| p.get_parameter_value_float())
    }

    pub fn set_parameter_value_float(&self, index: usize, value: f32){
        let index = index as usize;
        let opt_parameter = self.get_current_preset().parameters.get(index);

        if let Some(parameter) = opt_parameter {
            parameter.set_parameter_value_float(value.min(1.0).max(0.0));

            self.parameter_change_info.mark_as_changed(index);
        }
    }

    pub fn set_parameter_value_text(&self, index: usize, value: String) -> bool {
        let index = index as usize;
        let opt_parameter = self.get_current_preset().parameters.get(index);

        if let Some(parameter) = opt_parameter {
            if parameter.set_parameter_value_text(value) {
                self.parameter_change_info.mark_as_changed(index);

                return true;
            }
        }

        false
    }
}


/// PresetBank import/export handling
impl PresetBank {
    /// Import bytes into current bank, set sync parameters
    pub fn import_bank_from_bytes(&self, bytes: &[u8]){
        let res_serde_preset_bank: Result<SerdePresetBank, _> =
            from_bytes(bytes);

        if let Ok(serde_preset_bank) = res_serde_preset_bank {
            let empty_serde_preset = Preset::default().export_serde_preset();

            for (index, preset) in self.presets.iter().enumerate(){
                if let Some(serde_preset) = serde_preset_bank.presets.get(index){
                    preset.import_serde_preset(serde_preset);
                } else {
                    preset.import_serde_preset(&empty_serde_preset);
                }
            }

            self.set_preset_index(0);
            self.mark_parameters_as_changed();
        }
    }

    pub fn import_bytes_into_current_preset(&self, bytes: &[u8]){
        if self.get_current_preset().import_bytes(bytes){
            self.mark_parameters_as_changed();
        }
    }

    pub fn export_bank_as_bytes(&self) -> Vec<u8> {
        to_bytes(&SerdePresetBank::new(self))
    }

    pub fn export_current_preset_bytes(&self) -> Vec<u8> {
        self.get_current_preset().export_bytes()
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct SerdePresetParameter {
    name: String,
    value_float: f32,
    value_text: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct SerdePreset {
    name: String,
    parameters: Vec<SerdePresetParameter>,
}


impl SerdePreset {
    fn new(preset: &Preset) -> Self {
        let mut parameters = Vec::new();

        for i in 0..preset.parameters.len(){
            if let Some(parameter) = preset.parameters.get(i){
                parameters.push(SerdePresetParameter {
                    name: parameter.get_parameter_name(),
                    value_float: parameter.get_parameter_value_float(),
                    value_text: parameter.get_parameter_value_text(),
                });
            }
        }

        Self {
            name: preset.get_name(),
            parameters,
        }
    }
}


#[derive(Serialize, Deserialize)]
struct SerdePresetBank {
    presets: Vec<SerdePreset>,
}


impl SerdePresetBank {
    fn new(preset_bank: &PresetBank) -> Self {
        Self {
            presets: preset_bank.presets.iter()
                .map(|p| p.export_serde_preset())
                .collect()
        }
    }
}


#[cfg(test)]
mod tests {
    use quickcheck::{TestResult, quickcheck};
    use rand::{FromEntropy, Rng};
    use rand::rngs::SmallRng;

    use super::*;

    #[test]
    fn test_load_built_in_presets(){
        built_in_presets();
    }

    /// Test importing and exporting, as well as some related functionality
    #[test]
    fn test_export_import(){
        let mut rng = SmallRng::from_entropy();

        for _ in 0..20 {
            let bank_1 = PresetBank::new();

            for preset_index in 0..bank_1.len(){
                bank_1.set_preset_index(preset_index);

                assert_eq!(bank_1.get_preset_index(), preset_index);

                let current_preset = bank_1.get_current_preset();

                for parameter_index in 0..current_preset.parameters.len(){
                    let parameter = current_preset.parameters
                        .get(parameter_index)
                        .unwrap();
                    
                    let value: f32 = rng.gen();

                    parameter.set_parameter_value_float(value);

                    assert_eq!(parameter.get_parameter_value_float(), value);
                }
            }

            let bank_2 = PresetBank::new();

            bank_2.import_bank_from_bytes(&bank_1.export_bank_as_bytes());

            for preset_index in 0..bank_1.len(){
                bank_1.set_preset_index(preset_index);
                bank_2.set_preset_index(preset_index);

                let current_preset_1 = bank_1.get_current_preset();
                let current_preset_2 = bank_2.get_current_preset();

                for parameter_index in 0..current_preset_1.parameters.len(){
                    let parameter_1 = current_preset_1.parameters
                        .get(parameter_index)
                        .unwrap();

                    let parameter_2 = current_preset_2.parameters
                        .get(parameter_index).
                        unwrap();

                    assert_eq!(
                        parameter_1.get_parameter_value_float(),
                        parameter_2.get_parameter_value_float(),
                    );
                }
            }
        }
    }

    /// Don't crash when importing garbage
    /// 
    /// Not really a necessary test, Serde should manage fine.
    #[test]
    fn test_import_garbage(){
        fn prop(garbage: Vec<u8>) -> TestResult {
            let bank = PresetBank::new();

            bank.import_bank_from_bytes(&garbage);

            TestResult::from_bool(true)
        }

        quickcheck(prop as fn(Vec<u8>) -> TestResult);
    }
}