use crate::common::SampleRate;
use crate::parameters::*;

use super::{
    common::{AudioParameter, SimpleAudioParameter},
    AudioParameterPatchInteraction,
};

pub enum LfoTargetAudioParameter {
    One(SimpleAudioParameter<Lfo1TargetParameterValue>),
    Two(SimpleAudioParameter<Lfo2TargetParameterValue>),
    Three(SimpleAudioParameter<Lfo3TargetParameterValue>),
    Four(SimpleAudioParameter<Lfo4TargetParameterValue>),
}

impl LfoTargetAudioParameter {
    pub fn new(lfo_index: usize) -> Self {
        match lfo_index {
            0 => Self::One(Default::default()),
            1 => Self::Two(Default::default()),
            2 => Self::Three(Default::default()),
            3 => Self::Four(Default::default()),
            _ => unreachable!(),
        }
    }

    pub fn set_from_patch(&mut self, value: f32) {
        match self {
            Self::One(p) => p.set_from_patch(value),
            Self::Two(p) => p.set_from_patch(value),
            Self::Three(p) => p.set_from_patch(value),
            Self::Four(p) => p.set_from_patch(value),
        }
    }

    pub fn get_value(&self) -> LfoTargetParameter {
        match self {
            Self::One(p) => p.get_value(),
            Self::Two(p) => p.get_value(),
            Self::Three(p) => p.get_value(),
            Self::Four(p) => p.get_value(),
        }
    }

    pub fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        match self {
            Self::One(p) => p.advance_one_sample(sample_rate),
            Self::Two(p) => p.advance_one_sample(sample_rate),
            Self::Three(p) => p.advance_one_sample(sample_rate),
            Self::Four(p) => p.advance_one_sample(sample_rate),
        }
    }
}

impl AudioParameterPatchInteraction for LfoTargetAudioParameter {
    fn set_patch_value(&mut self, value: f32) {
        self.set_from_patch(value)
    }

    #[cfg(test)]
    fn compare_patch_value(&mut self, value: f32) -> bool {
        let a = match self {
            Self::One(_) => Lfo1TargetParameterValue::new_from_patch(value).to_patch(),
            Self::Two(_) => Lfo2TargetParameterValue::new_from_patch(value).to_patch(),
            Self::Three(_) => Lfo3TargetParameterValue::new_from_patch(value).to_patch(),
            Self::Four(_) => Lfo4TargetParameterValue::new_from_patch(value).to_patch(),
        };

        let b = match self {
            Self::One(p) => p.get_parameter_value().to_patch(),
            Self::Two(p) => p.get_parameter_value().to_patch(),
            Self::Three(p) => p.get_parameter_value().to_patch(),
            Self::Four(p) => p.get_parameter_value().to_patch(),
        };

        a == b
    }
}
