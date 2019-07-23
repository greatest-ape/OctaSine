
use crate::constants::*;
use crate::common::TimeCounter;


/// Convert plugin host float values in the range 0.0 - 1.0 to and from
/// the internal representation
pub trait ParameterValueConversion {
    type ProcessingParameterValue;

    fn to_processing(value: f64) -> Self::ProcessingParameterValue;
    fn to_preset(value: Self::ProcessingParameterValue) -> f64;

    /// Parse a string value coming from the host
    fn parse_string_value(value: String) -> Option<Self::ProcessingParameterValue>;

    fn format_processing(internal_value: Self::ProcessingParameterValue) -> String;

    fn format_value(value: f64) -> String {
        Self::format_processing(Self::to_processing(value))
    }
}


pub trait ProcessingParameter {
    type Value;

    fn get_value(&mut self, time: TimeCounter) -> Self::Value;
    fn get_target_value(&self) -> Self::Value;
    fn set_value(&mut self, value: Self::Value);
}

pub trait ProcessingParameterPresetValueAccess {
    fn set_from_preset_value(&mut self, value: f64);
    fn get_preset_target_value(&self) -> f64;
}

impl<P, T> ProcessingParameterPresetValueAccess for P
    where P:
        ProcessingParameter<Value = T> +
        ParameterValueConversion<ProcessingParameterValue = T>
{
    fn set_from_preset_value(&mut self, value: f64){
        self.set_value(Self::to_processing(value));
    }
    fn get_preset_target_value(&self) -> f64 {
        Self::to_preset(self.get_target_value())
    }
}


#[derive(Debug, Copy, Clone)]
pub struct InterpolatableProcessingValue {
    pub target_value: f64,
    current_value: f64,
    step_size: f64,
    steps_remaining: u8,
    samples_remaining: u8,
    last_time: TimeCounter,
}

impl InterpolatableProcessingValue {
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

    /// Possibly advance interpolation and call callback, return value.
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
