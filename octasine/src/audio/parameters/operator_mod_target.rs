use arrayvec::ArrayVec;

use crate::common::SampleRate;
use crate::parameter_values::{
    Operator2ModulationTargetValue, Operator3ModulationTargetValue, Operator4ModulationTargetValue,
};

use super::common::{AudioParameter, SimpleAudioParameter};

pub enum OperatorModulationTargetAudioParameter {
    Two(SimpleAudioParameter<Operator2ModulationTargetValue>),
    Three(SimpleAudioParameter<Operator3ModulationTargetValue>),
    Four(SimpleAudioParameter<Operator4ModulationTargetValue>),
}

impl OperatorModulationTargetAudioParameter {
    pub fn opt_new(operator_index: usize) -> Option<Self> {
        match operator_index {
            1 => Some(OperatorModulationTargetAudioParameter::Two(
                Default::default(),
            )),
            2 => Some(OperatorModulationTargetAudioParameter::Three(
                Default::default(),
            )),
            3 => Some(OperatorModulationTargetAudioParameter::Four(
                Default::default(),
            )),
            _ => None,
        }
    }

    pub fn get_active_indices(&self) -> ArrayVec<usize, 3> {
        let mut indices = ArrayVec::default();

        match self {
            Self::Two(p) => indices.extend(p.get_value().active_indices()),
            Self::Three(p) => indices.extend(p.get_value().active_indices()),
            Self::Four(p) => indices.extend(p.get_value().active_indices()),
        }

        indices
    }

    pub fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        match self {
            Self::Two(p) => p.advance_one_sample(sample_rate),
            Self::Three(p) => p.advance_one_sample(sample_rate),
            Self::Four(p) => p.advance_one_sample(sample_rate),
        }
    }
}
