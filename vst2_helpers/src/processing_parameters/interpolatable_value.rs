use super::TimeCounter;

pub const INTERPOLATION_SAMPLES_PER_STEP: u8 = 4;
pub const INTERPOLATION_STEPS: u8 = 8;
pub const INTERPOLATION_STEPS_FLOAT: f64 = INTERPOLATION_STEPS as f64;


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
    /// Needs to be called for every sample!
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