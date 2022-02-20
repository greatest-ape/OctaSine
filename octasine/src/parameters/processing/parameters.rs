use std::{f64::consts::FRAC_PI_2, marker::PhantomData};

use crate::common::*;
use crate::constants::*;
use crate::parameters::values::*;

use super::interpolatable_value::*;
use super::ProcessingParameter;

#[derive(Debug, Clone)]
pub struct InterpolatableProcessingParameter<P: ParameterValue> {
    value: InterpolatableProcessingValue,
    phantom_data: PhantomData<P>,
}

impl<P> Default for InterpolatableProcessingParameter<P>
where
    P: ParameterValue<Value = f64> + Default,
{
    fn default() -> Self {
        let default = P::default().get();

        Self {
            value: InterpolatableProcessingValue::new(default),
            phantom_data: PhantomData::default(),
        }
    }
}

impl<P> ProcessingParameter for InterpolatableProcessingParameter<P>
where
    P: ParameterValue<Value = f64>,
{
    type Value = f64;

    fn advance_one_sample(&mut self) {
        self.value.advance_one_sample(&mut |_| ())
    }
    fn get_value(&self) -> Self::Value {
        self.value.get_value()
    }
    fn set_from_sync(&mut self, value: f64) {
        self.value.set_value(P::from_sync(value).get())
    }
    fn get_value_with_lfo_addition(&mut self, lfo_addition: Option<f64>) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            let sync_value = P::from_processing(self.get_value()).to_sync();

            P::from_sync((sync_value + lfo_addition).min(1.0).max(0.0)).get()
        } else {
            self.get_value()
        }
    }
}

pub struct SimpleProcessingParameter<P: ParameterValue> {
    pub value: <P as ParameterValue>::Value,
    sync_cache: f64,
}

impl<P: ParameterValue + Default> Default for SimpleProcessingParameter<P> {
    fn default() -> Self {
        Self {
            value: P::default().get(),
            sync_cache: P::default().to_sync(),
        }
    }
}

impl<P> ProcessingParameter for SimpleProcessingParameter<P>
where
    P: ParameterValue,
{
    type Value = <P as ParameterValue>::Value;

    fn advance_one_sample(&mut self) {}
    fn get_value(&self) -> Self::Value {
        self.value
    }
    fn set_from_sync(&mut self, value: f64) {
        self.sync_cache = value;
        self.value = P::from_sync(value).get();
    }
    fn get_value_with_lfo_addition(&mut self, lfo_addition: Option<f64>) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            P::from_sync((self.sync_cache + lfo_addition).min(1.0).max(0.0)).get()
        } else {
            self.get_value()
        }
    }
}

// Master volume

#[derive(Debug, Clone)]
pub struct MasterVolumeProcessingParameter {
    value: InterpolatableProcessingValue,
}

impl Default for MasterVolumeProcessingParameter {
    fn default() -> Self {
        let default = MasterVolumeValue::default().get();

        Self {
            value: InterpolatableProcessingValue::new(default),
        }
    }
}

impl ProcessingParameter for MasterVolumeProcessingParameter {
    type Value = f64;

    fn advance_one_sample(&mut self) {
        self.value.advance_one_sample(&mut |_| ())
    }
    fn get_value(&self) -> Self::Value {
        self.value.get_value()
    }
    fn set_from_sync(&mut self, value: f64) {
        self.value
            .set_value(MasterVolumeValue::from_sync(value).get())
    }
    fn get_value_with_lfo_addition(&mut self, lfo_addition: Option<f64>) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            self.get_value() * 2.0f64.powf(lfo_addition)
        } else {
            self.get_value()
        }
    }
}

// Operator volume

#[derive(Debug, Clone)]
pub struct OperatorVolumeProcessingParameter {
    value: InterpolatableProcessingValue,
}

impl OperatorVolumeProcessingParameter {
    pub fn new(operator_index: usize) -> Self {
        let value = OperatorVolumeValue::new(operator_index).get();

        Self {
            value: InterpolatableProcessingValue::new(value),
        }
    }
}

impl ProcessingParameter for OperatorVolumeProcessingParameter {
    type Value = f64;

    fn advance_one_sample(&mut self) {
        self.value.advance_one_sample(&mut |_| ())
    }
    fn get_value(&self) -> Self::Value {
        self.value.get_value()
    }
    fn set_from_sync(&mut self, value: f64) {
        self.value
            .set_value(OperatorVolumeValue::from_sync(value).get())
    }
    fn get_value_with_lfo_addition(&mut self, lfo_addition: Option<f64>) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            self.get_value() * 2.0f64.powf(lfo_addition)
        } else {
            self.get_value()
        }
    }
}

// Master frequency / operator free frequency parameters with special lfo value handling

pub struct FreeFrequencyProcessingParameter<P: ParameterValue<Value = f64>> {
    pub value: <P as ParameterValue>::Value,
}

impl<P: ParameterValue<Value = f64> + Default> Default for FreeFrequencyProcessingParameter<P> {
    fn default() -> Self {
        Self {
            value: P::default().get(),
        }
    }
}

impl<P> ProcessingParameter for FreeFrequencyProcessingParameter<P>
where
    P: ParameterValue<Value = f64>,
{
    type Value = <P as ParameterValue>::Value;

    fn advance_one_sample(&mut self) {}
    fn get_value(&self) -> Self::Value {
        self.value
    }
    fn set_from_sync(&mut self, value: f64) {
        self.value = P::from_sync(value).get();
    }
    fn get_value_with_lfo_addition(&mut self, lfo_addition: Option<f64>) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            self.get_value() * 2.0f64.powf(lfo_addition)
        } else {
            self.get_value()
        }
    }
}

// Modulation target

pub enum OperatorModulationTargetProcessingParameter {
    Three(SimpleProcessingParameter<Operator3ModulationTargetValue>),
    Four(SimpleProcessingParameter<Operator4ModulationTargetValue>),
}

impl OperatorModulationTargetProcessingParameter {
    pub fn opt_new(operator_index: usize) -> Option<Self> {
        match operator_index {
            2 => Some(OperatorModulationTargetProcessingParameter::Three(
                Default::default(),
            )),
            3 => Some(OperatorModulationTargetProcessingParameter::Four(
                Default::default(),
            )),
            _ => None,
        }
    }

    pub fn get_value(&mut self) -> usize {
        match self {
            Self::Three(p) => p.get_value(),
            Self::Four(p) => p.get_value(),
        }
    }

    pub fn advance_one_sample(&mut self) {
        match self {
            Self::Three(p) => p.advance_one_sample(),
            Self::Four(p) => p.advance_one_sample(),
        }
    }
}

// Panning

#[derive(Debug, Clone)]
pub struct OperatorPanningProcessingParameter {
    value: InterpolatableProcessingValue,
    pub left_and_right: [f64; 2],
    pub lfo_active: bool,
}

impl OperatorPanningProcessingParameter {
    pub fn calculate_left_and_right(panning: f64) -> [f64; 2] {
        let pan_phase = panning * FRAC_PI_2;

        [pan_phase.cos(), pan_phase.sin()]
    }
}

impl ProcessingParameter for OperatorPanningProcessingParameter {
    type Value = f64;

    fn advance_one_sample(&mut self) {
        let mut opt_new_left_and_right = None;

        self.value.advance_one_sample(&mut |new_panning| {
            opt_new_left_and_right = Some(Self::calculate_left_and_right(new_panning));
        });

        if let Some(new_left_and_right) = opt_new_left_and_right {
            self.left_and_right = new_left_and_right;
        } else if self.lfo_active {
            self.left_and_right = Self::calculate_left_and_right(self.get_value());
        }

        self.lfo_active = false;
    }
    fn get_value(&self) -> Self::Value {
        self.value.get_value()
    }
    fn set_from_sync(&mut self, value: f64) {
        self.value
            .set_value(OperatorPanningValue::from_sync(value).get())
    }
    fn get_value_with_lfo_addition(&mut self, lfo_addition: Option<f64>) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            let sync_value = OperatorPanningValue::from_processing(self.get_value()).to_sync();

            let new_panning =
                OperatorPanningValue::from_sync((sync_value + lfo_addition).min(1.0).max(0.0))
                    .get();

            self.left_and_right = Self::calculate_left_and_right(new_panning);
            self.lfo_active = true;

            new_panning
        } else {
            self.get_value()
        }
    }
}

impl Default for OperatorPanningProcessingParameter {
    fn default() -> Self {
        let default = DEFAULT_OPERATOR_PANNING;

        Self {
            value: InterpolatableProcessingValue::new(default),
            left_and_right: Self::calculate_left_and_right(default),
            lfo_active: false,
        }
    }
}

// LFO target parameter

pub enum LfoTargetProcessingParameter {
    One(SimpleProcessingParameter<Lfo1TargetParameterValue>),
    Two(SimpleProcessingParameter<Lfo2TargetParameterValue>),
    Three(SimpleProcessingParameter<Lfo3TargetParameterValue>),
    Four(SimpleProcessingParameter<Lfo4TargetParameterValue>),
}

impl LfoTargetProcessingParameter {
    pub fn new(lfo_index: usize) -> Self {
        match lfo_index {
            0 => Self::One(Default::default()),
            1 => Self::Two(Default::default()),
            2 => Self::Three(Default::default()),
            3 => Self::Four(Default::default()),
            _ => unreachable!(),
        }
    }

    pub fn set_from_sync(&mut self, value: f64) {
        match self {
            Self::One(p) => p.set_from_sync(value),
            Self::Two(p) => p.set_from_sync(value),
            Self::Three(p) => p.set_from_sync(value),
            Self::Four(p) => p.set_from_sync(value),
        }
    }

    pub fn get_value(&self) -> LfoTargetParameter {
        match self {
            Self::One(p) => p.value,
            Self::Two(p) => p.value,
            Self::Three(p) => p.value,
            Self::Four(p) => p.value,
        }
    }

    pub fn advance_one_sample(&mut self) {
        match self {
            Self::One(p) => p.advance_one_sample(),
            Self::Two(p) => p.advance_one_sample(),
            Self::Three(p) => p.advance_one_sample(),
            Self::Four(p) => p.advance_one_sample(),
        }
    }
}
