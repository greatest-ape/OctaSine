use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

use arc_swap::ArcSwap;
use array_init::array_init;

use vst2_helpers::utils::atomic_double::AtomicPositiveDouble;


struct Parameter {
    value: AtomicPositiveDouble,
    name: String,
    format_value: fn(f64) -> String,
    unit_from_value: fn(f64) -> String,
    value_from_text: fn(String) -> Option<f64>,
}


impl Parameter {
    fn bla() -> Self {
        Self {
            value: AtomicPositiveDouble::new(0.0),
            name: "Test".to_string(),
            format_value: |v| format!("{}", v),
            unit_from_value: |v| format!("{}", v),
            value_from_text: |v| v.parse().ok(),
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
    parameters: Vec<Parameter>,
}


impl Preset {
    fn get_parameter(&self, index: usize) -> Option<&Parameter> {
        self.parameters.get(index)
    }

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
    fn format_parameter_value(&self, index: usize, value: f64) -> Option<String> {
        self.parameters.get(index).map(|p| (p.format_value)(value))
    }
}


struct PresetBank {
    presets: [Preset; 128],
    preset_index: AtomicUsize,
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
            .map(|p| (p.format_value)(p.value.get()))
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