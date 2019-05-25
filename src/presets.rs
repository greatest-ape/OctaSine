use serde::{Serialize, Deserialize};

use crate::crate_version;
use crate::constants::PLUGIN_NAME;
use crate::parameters::Parameters;


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


macro_rules! add_preset_from_file {
    ($preset_vec:ident, $path:expr) => {
        if let Ok(preset) = from_bytes(include_bytes!($path)){
            $preset_vec.push(preset);
        }
    };
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PresetPluginInfo {
    name: String,
    version: String,
}


impl PresetPluginInfo {
    fn new() -> Self {
        Self {
            name: PLUGIN_NAME.to_string(),
            version: crate_version!(),
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Presets {
    presets: Vec<Preset>,

    #[serde(skip, default = Self::default_index)]
    current_index: usize,
}


impl Presets {
    pub fn new() -> Self {
        let mut presets = Vec::new();

        presets.push(Preset::empty());

        // add_preset_from_file!(presets, "../presets/test.fxp");

        Self {
            presets,
            current_index: Self::default_index(),
        }
    }

    pub fn default_index() -> usize {
        0
    }

    pub fn len(&self) -> usize {
        self.presets.len()
    }

    pub fn set_parameters_from_current_preset(
        &self,
        parameters: &mut Parameters
    ){
        self.presets[self.current_index].set_parameters(parameters);
    }

    pub fn set_current_preset_from_parameters(
        &mut self,
        parameters: Parameters
    ){
        self.presets[self.current_index] = Preset::from_parameters(
            parameters,
            self.presets[self.current_index].name.clone()
        );
    }

    // VST methods: preset info and change

    pub fn get_current_index(&self) -> usize {
        self.current_index
    }

    pub fn get_name_by_index(&self, index: usize) -> String {
        self.presets.get(index).map_or("-".to_string(),
            |preset| preset.name.clone())
    }

    pub fn change_preset(&mut self, index: usize){
        if index < self.presets.len(){
            self.current_index = index;
        }
    }

    pub fn set_name_of_current(&mut self, name: String){
        self.presets[self.current_index].name = name
    }

    // VST methods: byte export/import

    pub fn get_current_preset_as_bytes(&self) -> Vec<u8> {
        to_bytes(&self.presets[self.current_index])
    }

    pub fn set_current_preset_from_bytes(&mut self, bytes: &[u8]){
        if let Ok(preset) = from_bytes(bytes) {
            self.presets[self.current_index] = preset;
        }
    }

    pub fn get_preset_bank_as_bytes(&self) -> Vec<u8> {
        to_bytes(self)
    }

    pub fn set_preset_bank_from_bytes(&mut self, bytes: &[u8]){
        if let Ok(bank) = from_bytes::<Presets>(bytes){
            let mut bank = bank;

            if bank.presets.len() == 0 {
                bank.presets.push(Preset::empty());
            }

            *self = bank;
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Preset {
    pub plugin_info: PresetPluginInfo,
    pub name: String,
    pub parameters: Vec<PresetParameter>,
}

impl Preset {
    pub fn from_parameters(parameters: Parameters, name: String) -> Self {
        // Create mutable copy because Parameters.get_index() demands it.
        // Since it is not necessary (or wished for) to make any changes to
        // the parameters, we don't take a mutable reference to them as an
        // argument to this function.
        let mut parameters = parameters;

        let mut preset_parameters = Vec::new();

        for i in 0..parameters.len() {
            let parameter = parameters.get_index(i)
                .expect("Non-existing parameter");

            preset_parameters.push(PresetParameter {
                name: parameter.get_parameter_name(),
                value_float: parameter.get_parameter_value_float(),
                value_text: parameter.get_parameter_value_text(),
            });
        }

        Self {
            plugin_info: PresetPluginInfo::new(),
            name,
            parameters: preset_parameters,
        }
    }

    pub fn empty() -> Self {
        Self::from_parameters(Parameters::new(), "-".to_string())
    }

    pub fn set_parameters(&self, parameters: &mut Parameters) {
        for (i, preset_parameter) in self.parameters.iter().enumerate() {
            if let Some(parameter) = parameters.get_index(i) {
                parameter.set_parameter_value_float(
                    preset_parameter.value_float
                );
            }
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PresetParameter {
    name: String,
    value_float: f64,
    value_text: String,
}


#[cfg(test)]
mod tests {
    use super::*;

    fn gen_random_presets() -> (Presets, Vec<f64>) {
        use rand::{FromEntropy, Rng};
        use rand::rngs::SmallRng;

        let mut rng = SmallRng::from_entropy();

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

        let mut presets = Presets::new();

        presets.set_current_preset_from_parameters(parameters);

        (presets, correct_values)
    }

    fn verify_preset_parameters(
        presets: Presets,
        correct_values: Vec<f64>
    ){
        use assert_approx_eq::assert_approx_eq;

        let mut parameters = Parameters::new();

        presets.set_parameters_from_current_preset(&mut parameters);

        for i in 0..parameters.len(){
            let parameter = parameters.get_index(i)
                .expect("no parameter for index");

            let parameter_value = parameter.get_parameter_value_float();
            
            let correct_value = correct_values[i];

            println!("Parameter name: {}", parameter.get_parameter_name());

            assert_approx_eq!(parameter_value, correct_value);
        }
    }

    #[test]
    fn test_parameter_set_and_get(){
        let (presets, correct_values) = gen_random_presets();

        verify_preset_parameters(presets, correct_values);
    }

    #[test]
    fn test_preset_byte_conversion(){
        let (presets, correct_values) = gen_random_presets();

        let bytes = presets.get_current_preset_as_bytes();

        let mut new_presets = Presets::new();

        new_presets.set_current_preset_from_bytes(&bytes);

        verify_preset_parameters(new_presets, correct_values);
    }
}