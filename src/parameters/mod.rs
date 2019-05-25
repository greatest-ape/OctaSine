use array_init::array_init;

use crate::constants::*;

pub mod common;
pub mod operators;
pub mod other;
pub mod utils;

pub use common::*;
pub use operators::*;
pub use other::*;
pub use utils::*;


pub type Operators = [Operator; NUM_OPERATORS];

/// State that can be changed with parameters. Only accessed through mutex
#[derive(Clone)]
pub struct Parameters {
    pub master_volume: MasterVolume,
    pub master_frequency: MasterFrequency,
    pub operators: Operators,
}

impl Parameters {
    pub fn new() -> Self {
        Self {
            master_volume: MasterVolume::new(),
            master_frequency: MasterFrequency {
                value: 440.0
            },
            operators: array_init(|i| Operator::new(i)),
        }
    }
}

impl Parameters {
    pub fn get_index(&mut self, index: usize) -> Option<&mut Parameter> {
        match index {
            0  => Some(&mut self.master_volume),
            1  => Some(&mut self.master_frequency),
            2  => Some(&mut self.operators[0].volume),
            3  => Some(&mut self.operators[0].panning),
            4  => Some(&mut self.operators[0].wave_type),
            5  => Some(&mut self.operators[0].modulation_index),
            6  => Some(&mut self.operators[0].feedback),
            7  => Some(&mut self.operators[0].frequency_ratio),
            8  => Some(&mut self.operators[0].frequency_free),
            9  => Some(&mut self.operators[0].frequency_fine),
            10 => Some(&mut self.operators[0].volume_envelope.attack_duration),
            11 => Some(&mut self.operators[0].volume_envelope.attack_end_value),
            12 => Some(&mut self.operators[0].volume_envelope.decay_duration),
            13 => Some(&mut self.operators[0].volume_envelope.decay_end_value),
            14 => Some(&mut self.operators[0].volume_envelope.release_duration),
            15 => Some(&mut self.operators[1].volume),
            16 => Some(&mut self.operators[1].panning),
            17 => Some(&mut self.operators[1].wave_type),
            18 => self.operators[1].additive_factor.as_mut()
                .map(|p| p as &mut Parameter),
            19 => Some(&mut self.operators[1].modulation_index),
            20 => Some(&mut self.operators[1].feedback),
            21 => Some(&mut self.operators[1].frequency_ratio),
            22 => Some(&mut self.operators[1].frequency_free),
            23 => Some(&mut self.operators[1].frequency_fine),
            24 => Some(&mut self.operators[1].volume_envelope.attack_duration),
            25 => Some(&mut self.operators[1].volume_envelope.attack_end_value),
            26 => Some(&mut self.operators[1].volume_envelope.decay_duration),
            27 => Some(&mut self.operators[1].volume_envelope.decay_end_value),
            28 => Some(&mut self.operators[1].volume_envelope.release_duration),
            29 => Some(&mut self.operators[2].volume),
            30 => Some(&mut self.operators[2].panning),
            31 => Some(&mut self.operators[2].wave_type),
            32 => self.operators[2].additive_factor.as_mut()
                .map(|p| p as &mut Parameter),
            33 => self.operators[2].output_operator.as_mut()
                .map(|p| p as &mut Parameter),
            34 => Some(&mut self.operators[2].modulation_index),
            35 => Some(&mut self.operators[2].feedback),
            36 => Some(&mut self.operators[2].frequency_ratio),
            37 => Some(&mut self.operators[2].frequency_free),
            38 => Some(&mut self.operators[2].frequency_fine),
            39 => Some(&mut self.operators[2].volume_envelope.attack_duration),
            40 => Some(&mut self.operators[2].volume_envelope.attack_end_value),
            41 => Some(&mut self.operators[2].volume_envelope.decay_duration),
            42 => Some(&mut self.operators[2].volume_envelope.decay_end_value),
            43 => Some(&mut self.operators[2].volume_envelope.release_duration),
            44 => Some(&mut self.operators[3].volume),
            45 => Some(&mut self.operators[3].panning),
            46 => Some(&mut self.operators[3].wave_type),
            47 => self.operators[3].additive_factor.as_mut()
                .map(|p| p as &mut Parameter),
            48 => self.operators[3].output_operator.as_mut()
                .map(|p| p as &mut Parameter),
            49 => Some(&mut self.operators[3].modulation_index),
            50 => Some(&mut self.operators[3].feedback),
            51 => Some(&mut self.operators[3].frequency_ratio),
            52 => Some(&mut self.operators[3].frequency_free),
            53 => Some(&mut self.operators[3].frequency_fine),
            54 => Some(&mut self.operators[3].volume_envelope.attack_duration),
            55 => Some(&mut self.operators[3].volume_envelope.attack_end_value),
            56 => Some(&mut self.operators[3].volume_envelope.decay_duration),
            57 => Some(&mut self.operators[3].volume_envelope.decay_end_value),
            58 => Some(&mut self.operators[3].volume_envelope.release_duration),

            _  => None
        }
    }

    pub fn len(&self) -> usize {
        59
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_value_nan_and_bounds() {
        use rand::{FromEntropy, Rng};
        use rand::rngs::SmallRng;

        let mut rng = SmallRng::from_entropy();
        let mut parameters = Parameters::new();

        for i in 0..parameters.len(){
            let parameter = parameters.get_index(i)
                .expect("no parameter for index");

            let random_value = rng.gen();
            
            parameter.set_parameter_value_float(random_value);

            let value = parameter.get_parameter_value_float();

            println!(
                "parameter name: {}, value: {}, input value: {}",
                parameter.get_parameter_name(),
                value,
                random_value,
            );

            assert!(!value.is_nan());
            assert!(value <= 1.0);
            assert!(value >= 0.0);
        }
    }
}