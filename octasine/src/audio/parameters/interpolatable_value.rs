pub const INTERPOLATION_STEPS: usize = 32;
pub const INTERPOLATION_STEPS_FLOAT: f64 = INTERPOLATION_STEPS as f64;

#[derive(Debug, Copy, Clone)]
pub struct InterpolatableAudioValue {
    value: f64,
    step_size: f64,
    steps_remaining: usize,
}

impl InterpolatableAudioValue {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            step_size: 0.0,
            steps_remaining: 0,
        }
    }

    pub fn advance_one_sample<F: FnMut(f64)>(&mut self, callback_on_advance: &mut F) {
        if self.steps_remaining == 0 || INTERPOLATION_STEPS == 0 {
            return;
        }

        self.steps_remaining -= 1;
        self.value += self.step_size;

        callback_on_advance(self.value);
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    #[allow(clippy::float_cmp)]
    pub fn set_value(&mut self, value: f64) {
        if INTERPOLATION_STEPS == 0 {
            self.value = value;
        } else if value == self.value || (value - self.value).abs() <= ::std::f64::EPSILON {
            self.steps_remaining = 0;
        } else {
            // Restart stepping process
            let diff = value - self.value;

            self.step_size = diff / INTERPOLATION_STEPS_FLOAT;
            self.steps_remaining = INTERPOLATION_STEPS;
        }
    }
}
