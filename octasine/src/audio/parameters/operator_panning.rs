use std::f64::consts::FRAC_PI_2;

use crate::audio::common::InterpolationDuration;
use crate::common::SampleRate;
use crate::parameter_values::{OperatorPanningValue, ParameterValue};

use super::common::{AudioParameter, InterpolatableAudioValue};

#[derive(Debug, Clone)]
pub struct OperatorPanningAudioParameter {
    value: InterpolatableAudioValue,
    pub left_and_right: [f64; 2],
    pub lfo_active: bool,
}

impl OperatorPanningAudioParameter {
    pub fn calculate_left_and_right(panning: f64) -> [f64; 2] {
        let pan_phase = panning * FRAC_PI_2;

        [pan_phase.cos(), pan_phase.sin()]
    }
}

impl AudioParameter for OperatorPanningAudioParameter {
    type Value = OperatorPanningValue;

    fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        let mut opt_new_left_and_right = None;

        self.value
            .advance_one_sample(sample_rate, &mut |new_panning| {
                opt_new_left_and_right = Some(Self::calculate_left_and_right(new_panning));
            });

        if let Some(new_left_and_right) = opt_new_left_and_right {
            self.left_and_right = new_left_and_right;
        } else if self.lfo_active {
            self.left_and_right = Self::calculate_left_and_right(self.get_value());
        }

        self.lfo_active = false;
    }
    fn get_value(&self) -> <Self::Value as ParameterValue>::Value {
        self.value.get_value()
    }
    fn set_from_patch(&mut self, value: f64) {
        self.value
            .set_value(Self::Value::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(
        &mut self,
        lfo_addition: Option<f64>,
    ) -> <Self::Value as ParameterValue>::Value {
        if let Some(lfo_addition) = lfo_addition {
            let patch_value = Self::Value::new_from_audio(self.get_value()).to_patch();

            let new_panning =
                Self::Value::new_from_patch((patch_value + lfo_addition).min(1.0).max(0.0)).get();

            self.left_and_right = Self::calculate_left_and_right(new_panning);
            self.lfo_active = true;

            new_panning
        } else {
            self.get_value()
        }
    }
}

impl Default for OperatorPanningAudioParameter {
    fn default() -> Self {
        let default = OperatorPanningValue::default().get();

        Self {
            value: InterpolatableAudioValue::new(default, InterpolationDuration::approx_1ms()),
            left_and_right: Self::calculate_left_and_right(default),
            lfo_active: false,
        }
    }
}
