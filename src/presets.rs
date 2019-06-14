use std::sync::atomic::{AtomicU64, Ordering};

use array_init::array_init;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};

use crate::atomics::atomic_float::AtomicFloat;
use crate::parameters::processing::ProcessingParameters;
use crate::parameters::sync::SyncParameters;


#[allow(unused_macros)]
macro_rules! preset_from_file {
    ($path:expr) => {
        Preset::new_from_bytes(
            include_bytes!($path)
        ).expect(&format!("Couldn't load preset file: {}", $path))
    };
}


fn built_in_presets() -> Vec<Preset> {
    #[allow(unused_mut)]
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
    /// Parameters. Limited to 64 parameters here because
    /// ChangedParametersInfo is used in the project
    parameters: [AtomicFloat; 64],
}


impl Preset {
    pub fn new_from_bytes(bytes: &[u8]) -> Result<Self, impl ::std::error::Error> {
        let res_serde_preset: Result<SerdePreset, _> = from_bytes(bytes);

        res_serde_preset.map(|serde_preset| {
            let preset = Self::default();

            preset.import_serde_preset(&serde_preset);

            preset
        })
    }

    pub fn get_name(&self) -> String {
        self.name.read().clone()
    }

    pub fn set_name(&self, name: String) {
        *self.name.write() = name;
    }

    pub fn import_bytes(&self, bytes: &[u8]){
        let res_serde_preset: Result<SerdePreset, _> = from_bytes(bytes);

        if let Ok(serde_preset) = res_serde_preset {
            self.import_serde_preset(&serde_preset);
        }
    }

    fn import_serde_preset(&self, serde_preset: &SerdePreset){
        for (index, parameter) in self.parameters.iter().enumerate(){
            if let Some(value) = serde_preset.parameters.get(index){
                parameter.set(*value);
            }
        }
    }

    pub fn export_bytes(&self) -> Vec<u8> {
        to_bytes(&self.export_serde_preset())
    }

    fn export_serde_preset(&self) -> SerdePreset {
        SerdePreset::new(self)
    }

    pub fn set_processing_parameters(
        &self,
        processing: &mut ProcessingParameters
    ){
        for (index, parameter) in self.parameters.iter().enumerate() {
            if let Some(p) = processing.get(index){
                p.set_from_sync_value(parameter.get());
            }
        }
    }

    pub fn set_sync_parameters(
        &self,
        sync: &SyncParameters,
    ){
        for (index, parameter) in self.parameters.iter().enumerate() {
            if let Some(p) = sync.get(index){
                p.set_parameter_value_float(parameter.get());
            }
        }
    }

    pub fn load_sync_parameters(
        &self,
        sync: &SyncParameters,
    ){
        for (index, parameter) in self.parameters.iter().enumerate() {
            if let Some(p) = sync.get(index){
                parameter.set(p.get_parameter_value_float());
            }
        }
    }
}


impl Default for Preset {
    fn default() -> Self {
        let preset = Self {
            name: RwLock::new("-".to_string()),
            parameters: array_init(|_| AtomicFloat::new(0.0)),
        };

        let mut processing_parameters = ProcessingParameters::new();

        for (index, parameter) in preset.parameters.iter().enumerate() {
            if let Some(p) = processing_parameters.get(index){
                parameter.set(p.get_sync_target_value());
            }
        }

        preset
    }
}


/// Mask for use with PresetBank.index_and_changed
/// 
/// OR with this to set changed bit to one
const CHANGED_MASK: u64 = 1 << 63;

/// Mask for use with PresetBank.index_and_changed
/// 
/// AND with this to set changed bit to zero
const NOT_CHANGED_MASK: u64 = !CHANGED_MASK;


pub struct PresetBank {
    presets: [Preset; 128],
    index_and_changed: AtomicU64,
}


impl PresetBank {
    fn index_in_bounds(&self, index: usize) -> bool {
        index < self.len()
    }

    fn extract_index(index_and_changed: u64) -> usize {
        (index_and_changed & NOT_CHANGED_MASK) as usize
    }

    fn get_current_preset(&self) -> &Preset {
        &self.presets[self.get_index()]
    }

    fn set_index(&self, index: usize){
        if !self.index_in_bounds(index){
            return;
        }

        let index_and_changed = (index as u64) | CHANGED_MASK;

        self.index_and_changed.store(index_and_changed, Ordering::SeqCst);
    }

    fn import_bank_bytes(&self, bytes: &[u8]){
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
        }

        self.set_index(0);
    }

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
            index_and_changed: AtomicU64::new(0),
        }
    }
    
    pub fn len(&self) -> usize {
        self.presets.len()
    }

    pub fn mark_as_changed(&self){
        self.index_and_changed.fetch_or(CHANGED_MASK, Ordering::SeqCst);
    }

    /// Retrieve change status and set it to unchanged. If anything had
    /// changed, set corresponding processing parameters.
    pub fn set_processing_if_changed(
        &self,
        processing_parameters: &mut ProcessingParameters,
    ){
        let index_and_changed = self.index_and_changed.fetch_and(
            NOT_CHANGED_MASK,
            Ordering::SeqCst
        );

        if (index_and_changed >> 63) & 1 == 1 {
            let index = Self::extract_index(index_and_changed);

            self.presets[index].set_processing_parameters(
                processing_parameters
            );
        }
    }

    pub fn set_index_and_set_sync_parameters(
        &self,
        index: usize,
        sync_parameters: &SyncParameters,
    ){
        if !self.index_in_bounds(index){
            return;
        }

        self.presets[index].set_sync_parameters(sync_parameters);

        self.set_index(index);
    }

    pub fn get_index(&self) -> usize {
        Self::extract_index(self.index_and_changed.load(Ordering::SeqCst))
    }

    pub fn get_preset_name_by_index(&self, index: usize) -> String {
        self.presets.get(index).map_or("".to_string(), |p| p.get_name())
    }

    pub fn set_current_preset_name(&self, name: String){
        self.get_current_preset().set_name(name)
    }

    /// Import bytes into current bank, set sync parameters
    pub fn import_bank_from_bytes(
        &self,
        sync_parameters: &SyncParameters,
        bytes: &[u8]
    ){
        self.import_bank_bytes(bytes);

        self.get_current_preset().set_sync_parameters(&sync_parameters);
    }

    /// Load sync parameters into current preset, then export bank as bytes
    pub fn export_bank_as_bytes(
        &self,
        sync_parameters: &SyncParameters
    ) -> Vec<u8> {
        self.get_current_preset()
            .load_sync_parameters(&sync_parameters);

        to_bytes(&SerdePresetBank::new(self))
    }

    /// Set current preset values to sync parameter values, export as bytes
    pub fn export_current_preset_bytes(
        &self,
        sync_parameters: &SyncParameters
    ) -> Vec<u8> {
        let current = self.get_current_preset();

        current.load_sync_parameters(&sync_parameters);
        current.export_bytes()
    }

    /// Import bytes into current preset, set sync parameters
    pub fn import_bytes_into_current_preset(
        &self,
        sync_parameters: &SyncParameters,
        bytes: &[u8]
    ){
        let current_preset = self.get_current_preset();

        current_preset.import_bytes(bytes);
        current_preset.set_sync_parameters(sync_parameters);

        self.mark_as_changed();
    }

    pub fn set_processing_and_sync_from_current(
        &self,
        processing_parameters: &mut ProcessingParameters,
        sync_parameters: &SyncParameters,
    ){
        let current_preset = self.get_current_preset();
        
        current_preset.set_processing_parameters(processing_parameters);
        current_preset.set_sync_parameters(&sync_parameters);
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct SerdePreset {
    name: String,
    parameters: Vec<f32>,
}


impl SerdePreset {
    fn new(preset: &Preset) -> Self {
        // FIXME: will export unused parameters
        let parameters = preset.parameters.iter().map(|p| p.get()).collect();

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
    use super::*;

    #[test]
    fn test_load_built_in_presets(){
        built_in_presets();
    }
}


/* Tests from old presets. Still need to be implemented here

#[cfg(test)]
mod tests {
    use rand::{FromEntropy, Rng};
    use rand::rngs::SmallRng;

    use super::*;

    fn gen_random_parameters(rng: &mut impl Rng) -> (Parameters, Vec<f32>) {
        let mut parameters = Parameters::new();
        let mut correct_values = Vec::new();

        for i in 0..parameters.len(){
            let parameter = parameters.get_index(i)
                .expect("no parameter for index");
            
            parameter.set_parameter_value_float(rng.gen());

            correct_values.push(
                parameter.get_parameter_value_float()
            );
        }

        (parameters, correct_values)
    }

    fn gen_random_presets() -> (Presets, Vec<Vec<f32>>) {
        let mut rng = SmallRng::from_entropy();

        let mut presets = Presets::new_with_many_presets(127);
        let mut all_correct_values = Vec::new();

        for i in 0..presets.len(){
            presets.change_preset(i);

            let (parameters, correct_values) = gen_random_parameters(&mut rng);

            presets.set_current_preset_from_parameters(parameters);
            all_correct_values.push(correct_values);
        }

        presets.change_preset(0);

        (presets, all_correct_values)
    }

    fn verify_preset_parameters(
        presets: Presets,
        all_correct_values: Vec<Vec<f32>>,
        verify_only_current: bool,
    ){
        use assert_approx_eq::assert_approx_eq;

        let mut presets = presets;

        let current_index = presets.get_current_index();

        for (i, correct_values) in all_correct_values.iter().enumerate(){
            if verify_only_current && i != current_index {
                continue;
            }

            let mut parameters = Parameters::new();

            presets.change_preset(i);
            presets.set_parameters_from_current_preset(&mut parameters);

            for j in 0..parameters.len(){
                let parameter = parameters.get_index(j)
                    .expect("no parameter for index");

                let parameter_value = parameter.get_parameter_value_float();
                
                let correct_value = correct_values[j];

                println!("SyncParameter name: {}", parameter.get_parameter_name());

                assert_approx_eq!(parameter_value, correct_value);
            }
        }
    }

    #[test]
    fn test_parameter_set_and_get(){
        let (presets, correct_values) = gen_random_presets();

        verify_preset_parameters(presets, correct_values, false);
    }

    #[test]
    fn test_preset_byte_conversion(){
        let (presets, correct_values) = gen_random_presets();

        let bytes = presets.get_current_preset_as_bytes();

        let mut new_presets = Presets::new_with_many_presets(127);

        new_presets.set_current_preset_from_bytes(&bytes);

        verify_preset_parameters(new_presets, correct_values, true);
    }

    #[test]
    fn test_preset_bank_byte_conversion(){
        let (presets, correct_values) = gen_random_presets();

        let bytes = presets.get_preset_bank_as_bytes();

        let mut new_presets = Presets::new();

        new_presets.set_preset_bank_from_bytes(&bytes);

        verify_preset_parameters(new_presets, correct_values, false);
    }
}

*/