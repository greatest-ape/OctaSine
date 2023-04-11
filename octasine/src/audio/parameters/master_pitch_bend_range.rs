use crate::common::SampleRate;
use crate::math::exp2_fast;
use crate::parameters::master_pitch_bend_range::MasterPitchBendRangeValue;
use crate::parameters::ParameterValue;

use super::common::AudioParameter;

pub struct MasterPitchBendRangeUpAudioParameter(MasterPitchBendRangeValue);

impl Default for MasterPitchBendRangeUpAudioParameter {
    fn default() -> Self {
        Self(MasterPitchBendRangeValue::default_up())
    }
}

impl AudioParameter for MasterPitchBendRangeUpAudioParameter {
    type ParameterValue = MasterPitchBendRangeValue;

    fn advance_one_sample(&mut self, _sample_rate: SampleRate) {}
    fn get_value(&self) -> <Self::ParameterValue as ParameterValue>::Value {
        self.0.get()
    }
    fn set_from_patch(&mut self, value: f32) {
        self.0 = Self::ParameterValue::new_from_patch(value);
    }
    fn get_value_with_lfo_addition(
        &mut self,
        lfo_addition: Option<f32>,
    ) -> <Self::ParameterValue as ParameterValue>::Value {
        if let Some(lfo_addition) = lfo_addition {
            // log2(1.5) / 2
            const FACTOR: f32 = 0.584_962_5 / 2.0;

            self.get_value() * exp2_fast(FACTOR * lfo_addition) as f32
        } else {
            self.get_value()
        }
    }
}

pub struct MasterPitchBendRangeDownAudioParameter(MasterPitchBendRangeValue);

impl Default for MasterPitchBendRangeDownAudioParameter {
    fn default() -> Self {
        Self(MasterPitchBendRangeValue::default_down())
    }
}

impl AudioParameter for MasterPitchBendRangeDownAudioParameter {
    type ParameterValue = MasterPitchBendRangeValue;

    fn advance_one_sample(&mut self, _sample_rate: SampleRate) {}
    fn get_value(&self) -> <Self::ParameterValue as ParameterValue>::Value {
        self.0.get()
    }
    fn set_from_patch(&mut self, value: f32) {
        self.0 = Self::ParameterValue::new_from_patch(value);
    }
    fn get_value_with_lfo_addition(
        &mut self,
        lfo_addition: Option<f32>,
    ) -> <Self::ParameterValue as ParameterValue>::Value {
        if let Some(lfo_addition) = lfo_addition {
            // log2(1.5) / 2
            const FACTOR: f32 = 0.584_962_5 / 2.0;

            self.get_value() * exp2_fast(FACTOR * lfo_addition) as f32
        } else {
            self.get_value()
        }
    }
}
