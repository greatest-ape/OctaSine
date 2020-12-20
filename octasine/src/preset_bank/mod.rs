use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

use arc_swap::ArcSwap;
use array_init::array_init;

use vst2_helpers::utils::atomic_double::AtomicPositiveDouble;

use crate::parameters::processing::values::ProcessingValue;

mod change_info;

use change_info::ParameterChangeInfo;


pub struct PresetParameter {
    pub value: AtomicPositiveDouble,
    pub name: String,
    pub unit_from_value: fn(f64) -> String,
    pub value_from_text: fn(String) -> Option<f64>,
    pub to_processing: fn(f64) -> ProcessingValue,
    pub format: fn(f64) -> String,
}


impl PresetParameter {
    pub fn set_from_text(&self, text: String) -> bool {
        if let Some(value) = (self.value_from_text)(text){
            self.value.set(value);

            true
        } else {
            false
        }
    }
}


struct Preset {
    pub name: ArcSwap<String>,
    pub parameters: Vec<PresetParameter>,
}


impl Preset {
    fn new(name: String) -> Self {
        let parameters = vec![
            PresetParameter::master_volume(),
            PresetParameter::master_frequency(),
            PresetParameter::operator_volume(0),
        ];

        Self {
            name: ArcSwap::new(Arc::new(name)),
            parameters
        }
    }
}


pub struct PresetBank {
    presets: [Preset; 128],
    preset_index: AtomicUsize,
    parameter_change_info_processing: ParameterChangeInfo,
    parameter_change_info_gui: ParameterChangeInfo,
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
    // Utils

    fn get_parameter(&self, index: usize) -> Option<&PresetParameter> {
        self.get_current_preset().parameters.get(index)
    }

    fn get_current_preset(&self) -> &Preset {
        &self.presets[self.get_preset_index()]
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

    pub fn set_preset_index(&self, index: usize){
        if index >= self.presets.len(){
            return;
        }

        self.preset_index.store(index, Ordering::SeqCst);
    }

    pub fn get_preset_name(&self, index: usize) -> Option<String> {
        self.presets.get(index as usize)
            .map(|p| (*p.name.load_full()).clone())
    }

    pub fn set_preset_name(&self, name: String){
        self.get_current_preset().name.store(Arc::new(name));
    }

    // Get parameter changes

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

    // Get parameter values

    pub fn get_parameter_value(&self, index: usize) -> Option<f64> {
        self.get_current_preset().parameters.get(index).map(|p| p.value.get())
    }

    pub fn get_parameter_value_text(&self, index: usize) -> Option<String> {
        self.get_current_preset().parameters.get(index)
            .map(|p| (p.format)(p.value.get()))
    }

    pub fn get_parameter_name(&self, index: usize) -> Option<String> {
        self.get_current_preset().parameters.get(index).map(|p| p.name.clone())
    }

    pub fn get_parameter_unit(&self, index: usize) -> Option<String> {
        self.get_current_preset().parameters.get(index)
            .map(|p| {
                let value = p.value.get();

                (&p.unit_from_value)(value)
            })
    }

    pub fn get_parameter_value_if_changed(&self, index: usize) -> Option<f64> {
        self.get_current_preset().parameters.get(index).and_then(|p| p.value.get_if_changed())
    }

    pub fn format_parameter_value(&self, index: usize, value: f64) -> Option<String> {
        self.get_current_preset().parameters.get(index).map(|p| (p.format)(value))
    }

    // Set parameters

    pub fn set_parameter_from_gui(&self, index: usize, value: f64){
        let opt_parameter = self.get_parameter(index);

        if let Some(parameter) = opt_parameter {
            parameter.value.set(value.min(1.0).max(0.0));

            self.parameter_change_info_processing
                .mark_as_changed(index);
        }
    }

    pub fn set_parameter_from_host(&self, index: usize, value: f64){
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
            if parameter.set_from_text(value){
                self.parameter_change_info_processing.mark_as_changed(index);
                self.parameter_change_info_gui.mark_as_changed(index);

                return true;
            }
        }

        false
    }
}