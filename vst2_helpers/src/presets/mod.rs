use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

use arc_swap::ArcSwap;
use array_init::array_init;
use serde::{Serialize, Deserialize};

pub mod parameters;
pub mod serde_utils;

pub use parameters::{PresetParameters, PresetParameter};
pub use serde_utils::*;


pub struct Preset<P> where P: PresetParameters {
    name: ArcSwap<String>,
    parameters: P,
}


impl<P> Default for Preset<P> where P: PresetParameters {
    fn default() -> Self {
        Self {
            name: ArcSwap::new(Arc::new("-".to_string())),
            parameters: P::default(),
        }
    }
}


impl<P> Preset<P> where P: PresetParameters {
    fn new_with_number_name(number: usize) -> Self {
        Self {
            name: ArcSwap::new(Arc::new(Self::format_number(number))),
            parameters: P::default(),
        }
    }

    fn format_number(number: usize) -> String {
        format!("{:03}", number)
    }

    fn get_name(&self) -> String {
        (*self.name.load_full()).clone()
    }

    fn set_name(&self, name: String) {
        self.name.store(Arc::new(name));
    }

    fn set_name_from_number(&self, number: usize){
        self.set_name(Self::format_number(number))
    }

}


pub struct PresetBank<P> where P: PresetParameters {
    presets: [Preset<P>; 128],
    preset_index: AtomicUsize,
    pub parameter_change_info_processing: ParameterChangeInfo,
    pub parameter_change_info_gui: ParameterChangeInfo,
}


impl<P> Default for PresetBank<P> where P: PresetParameters {
    fn default() -> Self {
        Self {
            presets: array_init(|i| Preset::new_with_number_name(i + 1)),
            preset_index: AtomicUsize::new(0),
            parameter_change_info_processing: ParameterChangeInfo::default(),
            parameter_change_info_gui: ParameterChangeInfo::default(),
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

    pub fn get_changed_parameters_from_processing(&self) -> Option<[Option<f64>; 64]> {
        self.parameter_change_info_processing.get_changed_parameters(
            &self.get_current_preset().parameters
        )
    }

    pub fn get_changed_parameters_from_gui(&self) -> Option<[Option<f64>; 64]> {
        self.parameter_change_info_gui.get_changed_parameters_transient(
            &self.get_current_preset().parameters
        )
    }

    pub fn gui_did_parameters_change(&self) -> bool {
        self.parameter_change_info_gui.changes_exist()
    }

    fn mark_parameters_as_changed(&self){
        self.parameter_change_info_processing.mark_all_as_changed();
        self.parameter_change_info_gui.mark_all_as_changed();
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

    pub fn set_parameter_value_float_from_host(&self, index: usize, value: f64){
        let index = index as usize;
        let opt_parameter = self.get_current_preset().parameters.get(index);

        if let Some(parameter) = opt_parameter {
            parameter.set_parameter_value_float(value.min(1.0).max(0.0));

            self.parameter_change_info_processing.mark_as_changed(index);
            self.parameter_change_info_gui.mark_as_changed(index);
        }
    }

    pub fn set_parameter_value_float_from_gui(&self, index: usize, value: f64){
        let index = index as usize;
        let opt_parameter = self.get_current_preset().parameters.get(index);

        if let Some(parameter) = opt_parameter {
            parameter.set_parameter_value_float(value.min(1.0).max(0.0));

            self.parameter_change_info_processing.mark_as_changed(index);
        }
    }

    pub fn set_parameter_value_text_from_host(&self, index: usize, value: String) -> bool {
        let index = index as usize;
        let opt_parameter = self.get_current_preset().parameters.get(index);

        if let Some(parameter) = opt_parameter {
            if parameter.set_parameter_value_text(value) {
                self.parameter_change_info_processing.mark_as_changed(index);
                self.parameter_change_info_gui.mark_as_changed(index);

                return true;
            }
        }

        false
    }

    pub fn format_parameter_value(&self, index: usize, value: f64) -> String {
        self.get_current_preset().parameters.get(index as usize).map_or(
            "".to_string(),
            |p| p.format_value(value)
        )
    }
}


/// PresetBank import/export handling
impl<P> PresetBank<P> where P: PresetParameters {
}
