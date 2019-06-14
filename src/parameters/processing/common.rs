use crate::constants::*;
use crate::common::TimeCounter;

use crate::parameters::common::ParameterValueConversion;


pub trait ProcessingParameter {
    type Value;

    fn get_value(&mut self, time: TimeCounter) -> Self::Value;
    fn get_target_value(&self) -> Self::Value;
    fn set_value(&mut self, value: Self::Value);
}

pub trait ProcessingParameterSyncValueAccess {
    fn set_from_sync_value(&mut self, value: f32);
    fn get_sync_target_value(&self) -> f32;
}

impl<P, T> ProcessingParameterSyncValueAccess for P
    where P:
        ProcessingParameter<Value = T> +
        ParameterValueConversion<ProcessingValue = T>
{
    fn set_from_sync_value(&mut self, value: f32){
        self.set_value(Self::to_processing(value));
    }
    fn get_sync_target_value(&self) -> f32 {
        Self::to_sync(self.get_target_value())
    }
}


#[derive(Debug, Copy, Clone)]
pub struct TimeInterpolatableValue {
    pub target_value: f32,
    current_value: f32,
    step_size: f32,
    steps_remaining: u8,
    samples_remaining: u8,
    last_time: TimeCounter,
}

impl TimeInterpolatableValue {
    pub fn new(value: f32) -> Self {
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
    pub fn get_value<F: FnMut(f32)>(
        &mut self,
        time: TimeCounter,
        callback_on_advance: &mut F
    ) -> f32 {
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
    pub fn set_value(&mut self, value: f32){
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

