use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

use arc_swap::ArcSwap;
use array_init::array_init;
use serde::{Serialize, Deserialize};

pub mod parameters;
pub mod change_info;

pub use parameters::{PresetParameters, PresetParameter};
pub use change_info::ParameterChangeInfo;


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


pub struct Preset<P> where P: PresetParameters {
    name: ArcSwap<String>,
    parameters: P,
}


impl<P> Preset<P> where P: PresetParameters {
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


impl<P> Default for Preset<P> where P: PresetParameters {
    fn default() -> Self {
        Self {
            name: ArcSwap::new(Arc::new("-".to_string())),
            parameters: P::default(),
        }
    }
}


pub struct PresetBank<P> where P: PresetParameters {
    presets: [Preset<P>; 128],
    preset_index: AtomicUsize,
    pub parameter_change_info: ParameterChangeInfo,
}


impl<P> Default for PresetBank<P> where P: PresetParameters {
    fn default() -> Self {
        Self {
            presets: array_init(|_| Preset::default()),
            preset_index: AtomicUsize::new(0),
            parameter_change_info: ParameterChangeInfo::default(),
        }
    }
}

impl<P> PresetBank<P> where P: PresetParameters {
    pub fn new_from_presets(presets: Vec<Preset<P>>) -> Self {
        let mut bank = Self::default();

        for (bank_preset, preset) in bank.presets.iter_mut()
            .zip(presets.into_iter())
        {
            *bank_preset = preset;
        }

        bank
    }

    pub fn len(&self) -> usize {
        self.presets.len()
    }

    pub fn get_changed_parameters(&self) -> Option<[Option<f64>; 64]> {
        self.parameter_change_info.get_changed_parameters(
            &self.get_current_preset().parameters
        )
    }

    fn mark_parameters_as_changed(&self){
        self.parameter_change_info.mark_all_as_changed();
    }

    fn get_current_preset(&self) -> &Preset<P> {
        &self.presets[self.get_preset_index()]
    }
}

/// PresetBank preset call handling
impl<P> PresetBank<P> where P: PresetParameters {
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
            .map_or("".to_string(), Preset::get_name)
    }

    pub fn set_current_preset_name(&self, name: String){
        self.get_current_preset().set_name(name)
    }
}


/// PresetBank parameter call handling
impl<P> PresetBank<P> where P: PresetParameters {
    pub fn get_num_parameters(&self) -> usize {
        self.get_current_preset().parameters.len()
    }

    pub fn get_parameter_name(&self, index: usize) -> String {
        self.get_current_preset().parameters.get(index as usize).map_or(
            "".to_string(),
            PresetParameter::get_parameter_name
        )
    }

    pub fn get_parameter_unit(&self, index: usize) -> String {
        self.get_current_preset().parameters.get(index as usize).map_or(
            "".to_string(),
            PresetParameter::get_parameter_unit_of_measurement
        )
    }

    pub fn can_parameter_be_automated(&self, index: usize) -> bool {
        self.get_current_preset().parameters.get(index as usize).is_some()
    }

    pub fn get_parameter_value_text(&self, index: usize) -> String {
        self.get_current_preset().parameters.get(index as usize).map_or(
            "".to_string(),
            PresetParameter::get_parameter_value_text
        )
    }

    pub fn get_parameter_value_float(&self, index: usize) -> f64 {
        self.get_current_preset().parameters.get(index as usize)
            .map_or(0.0, PresetParameter::get_parameter_value_float)
    }

    pub fn set_parameter_value_float(&self, index: usize, value: f64){
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
impl<P> PresetBank<P> where P: PresetParameters {
    /// Import bytes into current bank, set sync parameters
    pub fn import_bank_from_bytes(&self, bytes: &[u8]){
        let res_serde_preset_bank: Result<SerdePresetBank, _> =
            from_bytes(bytes);

        if let Ok(serde_preset_bank) = res_serde_preset_bank {
            let preset: Preset<P> = Preset::default();
            let empty_serde_preset = preset.export_serde_preset();

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
    value_float: f64,
    value_text: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct SerdePreset {
    name: String,
    parameters: Vec<SerdePresetParameter>,
}


impl SerdePreset {
    fn new(preset: &Preset<impl PresetParameters>) -> Self {
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
    fn new(preset_bank: &PresetBank<impl PresetParameters>) -> Self {
        Self {
            presets: preset_bank.presets.iter()
                .map(Preset::export_serde_preset)
                .collect()
        }
    }
}


/// Code to be included in tests, including from other crates
pub mod test_helpers {
    use assert_approx_eq::assert_approx_eq;

    use crate::presets::parameters::*;

    use super::*;

    /// Test importing and exporting, as well as some related functionality
    /// 
    /// Use this in other crates with your own preset parameter type!
    pub fn export_import<P>() where P: PresetParameters {
        for _ in 0..20 {
            let bank_1: PresetBank<P> = PresetBank::default();

            for preset_index in 0..bank_1.len(){
                bank_1.set_preset_index(preset_index);

                assert_eq!(bank_1.get_preset_index(), preset_index);

                let current_preset = bank_1.get_current_preset();

                for parameter_index in 0..current_preset.parameters.len(){
                    let parameter = current_preset.parameters
                        .get(parameter_index)
                        .unwrap();
                    
                    let value = fastrand::f64();

                    parameter.set_parameter_value_float(value);

                    assert_eq!(parameter.get_parameter_value_float(), value);
                }
            }

            let bank_2: PresetBank<P> = PresetBank::default();

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

                    assert_approx_eq!(
                        parameter_1.get_parameter_value_float(),
                        parameter_2.get_parameter_value_float(),
                        // Accept precision loss (probably due to
                        // JSON/javascript shenanigans)
                        0.0000000000000002
                    );
                }
            }
        }
    }
}


#[cfg(test)]
pub mod tests {
    use crate::presets::parameters::*;
    use crate::processing_parameters::*;
    use crate::utils::atomic_double::AtomicPositiveDouble;

    use super::*;

    pub(crate) struct TestProcessingParameter {
        value: f64
    }

    impl ProcessingParameter for TestProcessingParameter {
        type Value = f64;
        type ExtraData = ();

        fn get_value(&mut self, _extra_data: Self::ExtraData) -> Self::Value {
            self.value
        }
        fn get_target_value(&self) -> Self::Value {
            self.value
        }
        fn set_value(&mut self, value: Self::Value){
            self.value = value;
        }
    }

    impl ParameterValueConversion for TestProcessingParameter {
        type ProcessingParameterValue = f64;

        fn to_processing(value: f64) -> Self::ProcessingParameterValue {
            value
        }
        fn to_preset(value: Self::ProcessingParameterValue) -> f64 {
            value
        }

        fn parse_string_value(value: String) -> Option<Self::ProcessingParameterValue> {
            value.parse().ok()
        }

        fn format_processing(internal_value: Self::ProcessingParameterValue) -> String {
            format!("{}", internal_value)
        }
    }

    pub(crate) struct TestPresetParameter {
        name: String,
        value: AtomicPositiveDouble,
    }

    impl Default for TestPresetParameter {
        fn default() -> Self {
            Self {
                name: "test".to_string(),
                value: AtomicPositiveDouble::new(fastrand::f64()),
            }
        }
    }

    impl PresetParameterValueAccess for TestPresetParameter {
        fn set_value(&self, value: f64) {
            self.value.set(value);
        }
        fn get_value(&self) -> f64 {
            self.value.get()
        }
        fn get_value_if_changed(&self) -> Option<f64> {
            self.value.get_if_changed()
        }
    }


    impl PresetParameterGetName for TestPresetParameter {
        fn get_parameter_name(&self) -> String {
            self.name.clone()
        }
    }


    impl PresetParameterGetUnit for TestPresetParameter {
        fn get_parameter_unit_of_measurement(&self) -> String {
            "kHz".to_string()
        }
    }

    impl ParameterValueConversion for TestPresetParameter {
        type ProcessingParameterValue = f64;

        fn to_processing(value: f64) -> Self::ProcessingParameterValue {
            value
        }
        fn to_preset(value: Self::ProcessingParameterValue) -> f64 {
            value
        }

        fn parse_string_value(value: String) -> Option<Self::ProcessingParameterValue> {
            value.parse().ok()
        }

        fn format_processing(internal_value: Self::ProcessingParameterValue) -> String {
            format!("{}", internal_value)
        }
    }

    pub(crate) struct TestPresetParameters([TestPresetParameter; 60]);

    impl Default for TestPresetParameters {
        fn default() -> Self {
            Self(
                array_init(|_| TestPresetParameter::default()),
            )
        }
    }

    impl PresetParameters for TestPresetParameters {
        fn get(&self, index: usize) -> Option<&dyn PresetParameter> {
            self.0.get(index).map(|d| d as &dyn PresetParameter)
        }
        fn len(&self) -> usize {
            self.0.len()
        }
    }

    #[test]
    fn test_export_import(){
        use super::test_helpers::*;

        export_import::<TestPresetParameters>();
    }
}