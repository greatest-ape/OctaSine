use crate::common::*;
use crate::constants::*;


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


pub trait ParameterGetName {
    fn get_parameter_name(&self) -> String;
}


pub trait ParameterGetUnit {
    fn get_parameter_unit_of_measurement(&self) -> String {
        "".to_string()
    }
}


/// Set or get the internal representation of the parameter value.
/// Values often need to go through a conversion first.
/// 
/// Useful for abstracting over interpolatable and simply stored values.
pub trait ParameterInternalValueAccess<T> {
    fn set_converted_parameter_value(&mut self, value: T);
    fn get_unconverted_parameter_value(&self) -> T;
}


/// Convert plugin host float values in the range 0.0 - 1.0 to and from
/// the internal representation
pub trait ParameterValueConversion<T> {
    fn from_parameter_value(&self, value: f64) -> T;
    fn to_parameter_value(&self, value: T) -> f64;
}


pub trait ParameterStringParsing<T> {
    /// Parse a string value coming from the host to the internal
    /// representation
    fn parse_string_value(&self, value: String) -> Option<T>;
}


/// Simple general implementation of Parameter for structs implementing
/// the other Parameter traits and using f64s as values
impl<T> Parameter for T
    where T:
        ParameterGetName +
        ParameterGetUnit +
        ParameterInternalValueAccess<f64> +
        ParameterValueConversion<f64> + 
        ParameterStringParsing<f64>
{
    fn get_parameter_name(&self) -> String {
        ParameterGetName::get_parameter_name(self)
    }
    fn get_parameter_unit_of_measurement(&self) -> String {
        ParameterGetUnit::get_parameter_unit_of_measurement(self)
    }

    fn set_parameter_value_float(&mut self, value: f64){
        self.set_converted_parameter_value(self.from_parameter_value(value));
    }
    fn set_parameter_value_text(&mut self, value: String) -> bool {
        if let Some(value) = self.parse_string_value(value){
            self.set_converted_parameter_value(value);

            true
        } else {
            false
        }
    }
    fn get_parameter_value_float(&self) -> f64 {
        self.to_parameter_value(self.get_unconverted_parameter_value())
    }
    fn get_parameter_value_text(&self) -> String {
        format!("{:.2}", self.get_unconverted_parameter_value())
    }
}


#[derive(Debug, Copy, Clone)]
pub struct TimeInterpolatableValue {
    pub target_value: f64,
    current_value: f64,
    step_size: f64,
    steps_remaining: u8,
    samples_remaining: u8,
    last_time: TimeCounter,
}

impl TimeInterpolatableValue {
    pub fn new(value: f64) -> Self {
        Self {
            target_value: value,
            current_value: value,
            step_size: 0.0,
            steps_remaining: 0,
            last_time: TimeCounter(0.0),
            samples_remaining: INTERPOLATION_SAMPLES_PER_STEP,
        }
    }

    /// Possibly advance interpolation and call callback, return value
    pub fn get_value<F: FnMut(f64)>(
        &mut self,
        time: TimeCounter,
        callback_on_advance: &mut F
    ) -> f64 {
        if self.steps_remaining == 0 || INTERPOLATION_STEPS == 0 {
            return self.current_value;
        }

        if time != self.last_time {
            self.samples_remaining -= 1;

            if self.samples_remaining == 0 {
                self.current_value += self.step_size;

                callback_on_advance(self.current_value);

                self.steps_remaining -= 1;
                self.samples_remaining = INTERPOLATION_SAMPLES_PER_STEP;
            }

            self.last_time = time;
        }

        self.current_value
    }

    // Set target value, possibly restart interpolation
    pub fn set_value(&mut self, value: f64){
        self.target_value = value;

        if INTERPOLATION_STEPS == 0 {
            self.current_value = value;

            return;
        }

        if value == self.current_value {
            self.steps_remaining = 0;
        }
        else {
            // Restart stepping process
            let diff = value - self.current_value;
            self.step_size = diff / INTERPOLATION_STEPS_FLOAT;

            self.steps_remaining = INTERPOLATION_STEPS;
            self.samples_remaining = INTERPOLATION_SAMPLES_PER_STEP;
        }
    }
}