use std::f32::consts::FRAC_PI_2;

use crate::audio::common::InterpolationDuration;
use crate::common::SampleRate;
use crate::math::{cos, sin};
use crate::parameters::{OperatorPanningValue, ParameterValue};

use super::common::{AudioParameter, Interpolator};

#[derive(Debug, Clone)]
pub struct OperatorPanningAudioParameter {
    value: Interpolator,
    pub left_and_right: [f32; 2],
    pub lfo_active: bool,
}

impl OperatorPanningAudioParameter {
    pub fn calculate_left_and_right(panning: f32) -> [f32; 2] {
        let pan_phase = panning * FRAC_PI_2;

        [cos(pan_phase), sin(pan_phase)]
    }
}

impl AudioParameter for OperatorPanningAudioParameter {
    type ParameterValue = OperatorPanningValue;

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
    fn get_value(&self) -> <Self::ParameterValue as ParameterValue>::Value {
        self.value.get_value()
    }
    fn set_from_patch(&mut self, value: f32) {
        self.value
            .set_value(Self::ParameterValue::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(
        &mut self,
        lfo_addition: Option<f32>,
    ) -> <Self::ParameterValue as ParameterValue>::Value {
        if let Some(lfo_addition) = lfo_addition {
            let patch_value = Self::ParameterValue::new_from_audio(self.get_value()).to_patch();

            let new_panning = Self::ParameterValue::new_from_patch(
                (patch_value + lfo_addition as f32).min(1.0).max(0.0),
            )
            .get();

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
            value: Interpolator::new(default, InterpolationDuration::approx_1ms()),
            left_and_right: Self::calculate_left_and_right(default),
            lfo_active: false,
        }
    }
}
