use crate::common::*;


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


pub trait ParameterValueConversion<T> {
    fn from_parameter_value(&self, value: f64) -> T;
    fn to_parameter_value(&self, value: T) -> f64;
}


pub trait ParameterStringParsing<T> {
    fn parse_string_value(&self, value: String) -> Option<T>;
}



#[derive(Debug, Copy, Clone)]
pub struct TimeInterpolatableValue {
    pub current_value: f64,
    pub target_value: f64,
    pub step_size: f64,
    pub steps_remaining: usize,
    pub last_time: TimeCounter,
    pub total_num_steps: usize
}

impl TimeInterpolatableValue {
    pub fn new(value: f64) -> Self {
        Self {
            current_value: value,
            target_value: value,
            step_size: 0.0,
            steps_remaining: 0,
            last_time: TimeCounter(0.0),
            total_num_steps: 32,
        }
    }

    pub fn get_value(&mut self, time: TimeCounter) -> f64 {
        if self.total_num_steps == 0 {
            return self.current_value;
        }

        if time != self.last_time && self.steps_remaining > 0 {
            self.current_value += self.step_size;
            self.steps_remaining -= 1;
            self.last_time = time;
        }

        self.current_value
    }

    pub fn set_value(&mut self, value: f64){
        self.target_value = value;

        if self.total_num_steps == 0 {
            self.current_value = value;

            return;
        }

        if value == self.current_value {
            self.steps_remaining = 0;
        }
        else {
            // Restart stepping process
            let diff = value - self.current_value;
            self.step_size = diff / self.total_num_steps as f64;
            self.steps_remaining = self.total_num_steps;
        }
    }
}