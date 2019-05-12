use crate::common::*;
use crate::constants::*;
use crate::operators::Operator;


pub trait Parameter {
    fn get_parameter_name(&self) -> String;
    fn get_parameter_unit_of_measurement(&self) -> String {
        "".to_string()
    }

    fn get_parameter_value_float(&self) -> f64;
    fn set_parameter_value_float(&mut self, value: f64);

    fn get_parameter_value_text(&self) -> String;
    fn set_parameter_value_text(&mut self, _value: String) -> bool;
}


pub type Operators = [Operator; NUM_OPERATORS];

/// State that can be changed with parameters. Only accessed through mutex
pub struct Parameters {
    pub master_frequency: MasterFrequency,
    pub operators: Operators,
}

impl Parameters {
    pub fn get_index(&mut self, index: usize) -> Option<&mut Parameter> {
        match index {
            0  => Some(&mut self.operators[0].volume),
            1  => Some(&mut self.operators[0].panning),
            2  => Some(&mut self.operators[0].wave_type),
            3  => Some(&mut self.operators[0].modulation_index),
            4  => Some(&mut self.operators[0].feedback),
            5  => Some(&mut self.operators[0].frequency_ratio),
            6  => Some(&mut self.operators[0].frequency_free),
            7  => Some(&mut self.operators[0].frequency_fine),
            8  => Some(&mut self.operators[0].volume_envelope.attack_duration),
            9  => Some(&mut self.operators[0].volume_envelope.attack_end_value),
            10 => Some(&mut self.operators[0].volume_envelope.decay_duration),
            11 => Some(&mut self.operators[0].volume_envelope.decay_end_value),
            12 => Some(&mut self.operators[0].volume_envelope.release_duration),
            13 => Some(&mut self.operators[1].volume),
            14 => Some(&mut self.operators[1].panning),
            15 => Some(&mut self.operators[1].wave_type),
            16 => self.operators[1].additive_factor.as_mut()
                .map(|p| p as &mut Parameter),
            17 => Some(&mut self.operators[1].modulation_index),
            18 => Some(&mut self.operators[1].feedback),
            19 => Some(&mut self.operators[1].frequency_ratio),
            20 => Some(&mut self.operators[1].frequency_free),
            21 => Some(&mut self.operators[1].frequency_fine),
            22 => Some(&mut self.operators[1].volume_envelope.attack_duration),
            23 => Some(&mut self.operators[1].volume_envelope.attack_end_value),
            24 => Some(&mut self.operators[1].volume_envelope.decay_duration),
            25 => Some(&mut self.operators[1].volume_envelope.decay_end_value),
            26 => Some(&mut self.operators[1].volume_envelope.release_duration),
            27 => Some(&mut self.operators[2].volume),
            28 => Some(&mut self.operators[2].panning),
            29 => Some(&mut self.operators[2].wave_type),
            30 => self.operators[2].additive_factor.as_mut()
                .map(|p| p as &mut Parameter),
            31 => self.operators[2].output_operator.as_mut()
                .map(|p| p as &mut Parameter),
            32 => Some(&mut self.operators[2].modulation_index),
            33 => Some(&mut self.operators[2].feedback),
            34 => Some(&mut self.operators[2].frequency_ratio),
            35 => Some(&mut self.operators[2].frequency_free),
            36 => Some(&mut self.operators[2].frequency_fine),
            37 => Some(&mut self.operators[2].volume_envelope.attack_duration),
            38 => Some(&mut self.operators[2].volume_envelope.attack_end_value),
            39 => Some(&mut self.operators[2].volume_envelope.decay_duration),
            40 => Some(&mut self.operators[2].volume_envelope.decay_end_value),
            41 => Some(&mut self.operators[2].volume_envelope.release_duration),
            42 => Some(&mut self.operators[3].volume),
            43 => Some(&mut self.operators[3].panning),
            44 => Some(&mut self.operators[3].wave_type),
            45 => self.operators[3].additive_factor.as_mut()
                .map(|p| p as &mut Parameter),
            46 => self.operators[3].output_operator.as_mut()
                .map(|p| p as &mut Parameter),
            47 => Some(&mut self.operators[3].modulation_index),
            48 => Some(&mut self.operators[3].feedback),
            49 => Some(&mut self.operators[3].frequency_ratio),
            50 => Some(&mut self.operators[3].frequency_free),
            51 => Some(&mut self.operators[3].frequency_fine),
            52 => Some(&mut self.operators[3].volume_envelope.attack_duration),
            53 => Some(&mut self.operators[3].volume_envelope.attack_end_value),
            54 => Some(&mut self.operators[3].volume_envelope.decay_duration),
            55 => Some(&mut self.operators[3].volume_envelope.decay_end_value),
            56 => Some(&mut self.operators[3].volume_envelope.release_duration),

            _  => None
        }
    }

    pub fn len(&self) -> usize {
        57
    }
}