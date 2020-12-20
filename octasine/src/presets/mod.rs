use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

use arc_swap::ArcSwap;
use array_init::array_init;

mod change_info;
mod parameters;

use change_info::ParameterChangeInfo;
use parameters::*;


pub struct Preset {
    pub name: ArcSwap<String>,
    pub parameters: Vec<SyncParameter>,
}


impl Preset {
    fn new(name: String) -> Self {
        let parameters = vec![
            SyncParameter::master_volume(),
            SyncParameter::master_frequency(),
        ];

        Self {
            name: ArcSwap::new(Arc::new(name)),
            parameters
        }
    }

    pub fn get_parameter(&self, index: usize) -> Option<&SyncParameter> {
        self.parameters.get(index)
    }
    pub fn get_parameter_value(&self, index: usize) -> Option<f64> {
        self.parameters.get(index).map(|p| p.value.get())
    }
    pub fn get_parameter_name(&self, index: usize) -> Option<String> {
        self.parameters.get(index).map(|p| p.name.clone())
    }
    pub fn get_parameter_value_if_changed(&self, index: usize) -> Option<f64> {
        self.parameters.get(index).and_then(|p| p.value.get_if_changed())
    }
    pub fn format_parameter_value(&self, index: usize, value: f64) -> Option<String> {
        self.parameters.get(index).map(|p| (p.format)(value))
    }
}


pub struct PresetBank {
    presets: [Preset; 128],
    preset_index: AtomicUsize,
    pub parameter_change_info_processing: ParameterChangeInfo,
    pub parameter_change_info_gui: ParameterChangeInfo,
}


impl Default for PresetBank {
    fn default() -> Self {
        Self {
            presets: array_init(|i| Preset::new(format!("{}", i + 1))),
            preset_index: AtomicUsize::new(0),
            parameter_change_info_processing: ParameterChangeInfo::default(),
            parameter_change_info_gui: ParameterChangeInfo::default(),
        }
    }
}


impl PresetBank {
    pub fn len(&self) -> usize {
        self.presets.len()
    }

    pub fn get_num_parameters(&self) -> usize {
        self.get_current_preset().parameters.len()
    }

    pub fn get_current_preset(&self) -> &Preset {
        &self.presets[self.get_preset_index()]
    }

    pub fn get_preset(&self, index: usize) -> Option<&Preset> {
        self.presets.get(index)
    }

    pub fn get_preset_index(&self) -> usize {
        self.preset_index.load(Ordering::SeqCst)
    }

    pub fn set_preset_index(&self, index: usize){
        if index >= self.presets.len(){
            return;
        }

        self.preset_index.store(index, Ordering::SeqCst);
    }

    pub fn get_changed_parameters_from_processing(&self) -> Option<[Option<f64>; 64]> {
        self.parameter_change_info_processing.get_changed_parameters(
            &self.get_current_preset().parameters
        )
    }

    pub fn get_changed_parameters_from_gui(&self) -> Option<[Option<f64>; 64]> {
        self.parameter_change_info_processing.get_changed_parameters_transient(
            &self.get_current_preset().parameters
        )
    }
}