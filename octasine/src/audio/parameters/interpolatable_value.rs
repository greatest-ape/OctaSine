/// Does not support STEPS = 0
#[derive(Debug, Copy, Clone)]
pub struct InterpolatableAudioValue<const STEPS: u16 = 32> {
    value: f64,
    step_size: f64,
    steps_remaining: u16,
    steps_f64: f64,
}

impl<const STEPS: u16> InterpolatableAudioValue<STEPS> {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            step_size: 0.0,
            steps_remaining: 0,
            steps_f64: f64::from(STEPS),
        }
    }

    pub fn advance_one_sample<F: FnMut(f64)>(&mut self, callback_on_advance: &mut F) {
        if self.steps_remaining == 0 {
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
        if value == self.value || (value - self.value).abs() <= f64::EPSILON {
            self.steps_remaining = 0;
        } else {
            // Restart stepping process
            let diff = value - self.value;

            self.step_size = diff / self.steps_f64;
            self.steps_remaining = STEPS;
        }
    }
}
