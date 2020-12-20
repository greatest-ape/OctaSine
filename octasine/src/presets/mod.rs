use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

use arc_swap::ArcSwap;
use array_init::array_init;

use vst2_helpers::utils::atomic_double::AtomicPositiveDouble;
use vst2_helpers::processing_parameters::utils::*;

use crate::constants::*;

mod change_info;

use change_info::ParameterChangeInfo;


trait ProcessingValueConversion {
    fn from_sync(value: f64) -> Self;
    fn to_sync(self) -> f64;
    fn format(self) -> String;
    fn format_sync(value: f64) -> String;
}


#[derive(Debug, Clone, Copy)]
struct MasterVolume(f64);


impl ProcessingValueConversion for MasterVolume {
    fn from_sync(value: f64) -> Self {
        Self(value * 2.0)
    }
    fn to_sync(self) -> f64 {
        self.0 / 2.0
    }
    fn format(self) -> String {
        format!("{:.2}", 20.0 * self.0.log10())
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
struct MasterFrequency(f64);


impl ProcessingValueConversion for MasterFrequency {
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(
            &MASTER_FREQUENCY_STEPS,
            sync
        ))
    }
    fn to_sync(self) -> f64 {
        map_value_to_parameter_value_with_steps(&MASTER_FREQUENCY_STEPS, self.0)
    }
    fn format(self) -> String {
        format!("{:.02}", self.0)
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}


#[derive(Debug, Clone, Copy)]
enum ProcessingValue {
    MasterVolume(MasterVolume),
    MasterFrequency(MasterFrequency),
}


impl ProcessingValue {
}


struct SyncParameter {
    value: AtomicPositiveDouble,
    name: String,
    unit_from_value: fn(f64) -> String,
    value_from_text: fn(String) -> Option<f64>,
    to_processing: fn(f64) -> ProcessingValue,
    format: fn(f64) -> String,
}


impl SyncParameter {
    fn master_volume() -> Self {
        let value = MasterVolume(DEFAULT_MASTER_VOLUME).to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: "Master volume".to_string(),
            unit_from_value: |_| "dB".to_string(),
            value_from_text: |v| None,
            to_processing: |v| ProcessingValue::MasterVolume(
                MasterVolume::from_sync(v)
            ),
            format: |v| MasterVolume::from_sync(v).format(),
        }
    }

    fn master_frequency() -> Self {
        let value = MasterFrequency(DEFAULT_MASTER_FREQUENCY).to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: "Master frequency".to_string(),
            unit_from_value: |_| "Hz".to_string(),
            value_from_text: |v| None,
            to_processing: |v| ProcessingValue::MasterFrequency(
                MasterFrequency::from_sync(v)
            ),
            format: |v| MasterFrequency::from_sync(v).format(),
        }
    }

    fn set_from_text(&self, text: String) -> bool {
        if let Some(value) = (self.value_from_text)(text){
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

    fn get_parameter(&self, index: usize) -> Option<&SyncParameter> {
        self.parameters.get(index)
    }

    /*
    fn set_parameter_value(&self, index: usize, value: f64){
        self.parameters.get(index).map(|p| p.value.set(value));
    }
    fn get_parameter_value(&self, index: usize) -> Option<f64> {
        self.parameters.get(index).map(|p| p.value.get())
    }
    fn get_parameter_name(&self, index: usize) -> Option<String> {
        self.parameters.get(index).map(|p| p.name.clone())
    }
    fn get_parameter_value_if_changed(&self, index: usize) -> Option<f64> {
        self.parameters.get(index).and_then(|p| p.value.get_if_changed())
    }
    */
}


struct PresetBank {
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

    pub fn get_changed_parameters_from_processing(&self) -> Option<[Option<ProcessingValue>; 64]> {
        self.parameter_change_info_processing.get_changed_parameters(
            &self.get_current_preset().parameters
        )
    }

    pub fn get_changed_parameters_from_gui(&self) -> Option<[Option<ProcessingValue>; 64]> {
        self.parameter_change_info_processing.get_changed_parameters_transient(
            &self.get_current_preset().parameters
        )
    }
}


impl vst::plugin::PluginParameters for PresetBank {
    /// Get parameter label for parameter at `index` (e.g. "db", "sec", "ms", "%").
    fn get_parameter_label(&self, index: i32) -> String {
        self.get_current_preset()
            .get_parameter(index as usize)
            .map(|p| {
                let value = p.value.get();

                (&p.unit_from_value)(value)
            })
            .unwrap_or_else(|| "".to_string())
    }

    /// Get the parameter value for parameter at `index` (e.g. "1.0", "150", "Plate", "Off").
    fn get_parameter_text(&self, index: i32) -> String {
        self.get_current_preset()
            .get_parameter(index as usize)
            .map(|p| (p.format)(p.value.get()))
            .unwrap_or_else(|| "".to_string())
    }

    /// Get the name of parameter at `index`.
    fn get_parameter_name(&self, index: i32) -> String {
        self.get_current_preset()
            .get_parameter(index as usize)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "".to_string())
    }

    /// Get the value of paramater at `index`. Should be value between 0.0 and 1.0.
    fn get_parameter(&self, index: i32) -> f32 {
        self.get_current_preset()
            .get_parameter(index as usize)
            .map(|p| p.value.get() as f32)
            .unwrap_or(0.0)
    }

    /// Set the value of parameter at `index`. `value` is between 0.0 and 1.0.
    fn set_parameter(&self, index: i32, value: f32) {
        let opt_parameter = self.get_current_preset()
            .get_parameter(index as usize);

        if let Some(parameter) = opt_parameter {
            parameter.value.set(value as f64)
        }
    }

    /// Use String as input for parameter value. Used by host to provide an editable field to
    /// adjust a parameter value. E.g. "100" may be interpreted as 100hz for parameter. Returns if
    /// the input string was used.
    fn string_to_parameter(&self, index: i32, text: String) -> bool {
        let opt_parameter = self.get_current_preset()
            .get_parameter(index as usize);

        if let Some(parameter) = opt_parameter {
            parameter.set_from_text(text)
        } else {
            false
        }
    }

    /// Return whether parameter at `index` can be automated.
    fn can_be_automated(&self, index: i32) -> bool {
        self.get_current_preset().parameters.len() < index as usize
    }

    /// Set the current preset to the index specified by `preset`.
    ///
    /// This method can be called on the processing thread for automation.
    fn change_preset(&self, index: i32) {
        self.set_preset_index(index as usize);
    }

    /// Get the current preset index.
    fn get_preset_num(&self) -> i32 {
        self.get_preset_index() as i32
    }

    /// Set the current preset name.
    fn set_preset_name(&self, name: String) {
        self.get_current_preset().name.store(Arc::new(name));
    }

    /// Get the name of the preset at the index specified by `preset`.
    fn get_preset_name(&self, index: i32) -> String {
        self.get_preset(index as usize)
            .map(|p| (*p.name.load_full()).clone())
            .unwrap_or_else(|| "".to_string())
    }

    /*
    /// If `preset_chunks` is set to true in plugin info, this should return the raw chunk data for
    /// the current preset.
    fn get_preset_data(&self) -> Vec<u8> {
        self.export_current_preset_bytes()
    }

    /// If `preset_chunks` is set to true in plugin info, this should return the raw chunk data for
    /// the current plugin bank.
    fn get_bank_data(&self) -> Vec<u8> {
        self.export_bank_as_bytes()
    }

    /// If `preset_chunks` is set to true in plugin info, this should load a preset from the given
    /// chunk data.
    fn load_preset_data(&self, data: &[u8]) {
        self.import_bytes_into_current_preset(data);
    }

    /// If `preset_chunks` is set to true in plugin info, this should load a preset bank from the
    /// given chunk data.
    fn load_bank_data(&self, data: &[u8]) {
        self.import_bank_from_bytes(data);
    }
    */
}