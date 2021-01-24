use std::{f64::consts::FRAC_PI_2, marker::PhantomData};

use crate::common::*;
use crate::constants::*;
use crate::parameters::values::*;

use super::interpolatable_value::*;


pub trait ProcessingParameter {
    type Value;
    type ExtraData;

    fn get_value(&mut self, extra_data: Self::ExtraData) -> Self::Value;
    fn set_from_sync(&mut self, value: f64);
    fn get_value_with_lfo_addition(
        &mut self,
        extra_data: Self::ExtraData,
        lfo_addition: Option<f64>
    ) -> Self::Value;
}


#[derive(Debug, Clone)]
pub struct InterpolatableProcessingParameter<P: ParameterValue> {
    value: InterpolatableProcessingValue,
    phantom_data: PhantomData<P>,
}

impl <P> Default for InterpolatableProcessingParameter<P>
    where P: ParameterValue<Value=f64> + Default
{
    fn default() -> Self {
        let default = P::default().get();

        Self {
            value: InterpolatableProcessingValue::new(default),
            phantom_data: PhantomData::default(),
        }
    }
}

impl <P>ProcessingParameter for InterpolatableProcessingParameter<P>
    where P: ParameterValue<Value=f64>
{
    type Value = f64;
    type ExtraData = TimeCounter;

    fn get_value(&mut self, extra_data: Self::ExtraData) -> Self::Value {
        self.value.get_value(extra_data, &mut |_| ())
    }
    fn set_from_sync(&mut self, value: f64){
        self.value.set_value(P::from_sync(value).get())
    }
    fn get_value_with_lfo_addition(
        &mut self,
        extra_data: Self::ExtraData,
        lfo_addition: Option<f64>
    ) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            let sync_value = P::from_processing(
                self.get_value(extra_data)
            ).to_sync();

            P::from_sync(
                (sync_value + lfo_addition).min(1.0).max(0.0)
            ).get()
        } else {
            self.get_value(extra_data)
        }   
    }   
}


pub struct SimpleProcessingParameter<P: ParameterValue> {
    pub value: <P as ParameterValue>::Value,
    sync_cache: f64,
}

impl <P: ParameterValue + Default>Default for SimpleProcessingParameter<P> {
    fn default() -> Self {
        Self {
            value: P::default().get(),
            sync_cache: P::default().to_sync(),
        }
    }
}

impl <P>ProcessingParameter for SimpleProcessingParameter<P>
    where P: ParameterValue
{
    type Value = <P as ParameterValue>::Value;
    type ExtraData = ();

    fn get_value(&mut self, _: Self::ExtraData) -> Self::Value {
        self.value
    }
    fn set_from_sync(&mut self, value: f64){
        self.sync_cache = value;
        self.value = P::from_sync(value).get();
    }
    fn get_value_with_lfo_addition(
        &mut self,
        extra_data: Self::ExtraData,
        lfo_addition: Option<f64>
    ) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            P::from_sync(
                (self.sync_cache + lfo_addition).min(1.0).max(0.0)
            ).get()
        } else {
            self.get_value(extra_data)
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
            value: InterpolatableProcessingValue::new(value)
        }
    }
}

impl ProcessingParameter for OperatorVolumeProcessingParameter {
    type Value = f64;
    type ExtraData = TimeCounter;

    fn get_value(&mut self, extra_data: Self::ExtraData) -> Self::Value {
        self.value.get_value(extra_data, &mut |_| ())
    }
    fn set_from_sync(&mut self, value: f64){
        self.value.set_value(OperatorVolumeValue::from_sync(value).get())
    }
    fn get_value_with_lfo_addition(
        &mut self,
        extra_data: Self::ExtraData,
        lfo_addition: Option<f64>
    ) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            let sync_value = OperatorVolumeValue::from_processing(
                self.get_value(extra_data)
            ).to_sync();

            OperatorVolumeValue::from_sync(
                (sync_value + lfo_addition).min(1.0).max(0.0)
            ).get()
        } else {
            self.get_value(extra_data)
        }   
    }   
}


// Modulation target

pub enum OperatorModulationTargetProcessingParameter {
    OperatorIndex2(SimpleProcessingParameter<OperatorModulationTarget2Value>),
    OperatorIndex3(SimpleProcessingParameter<OperatorModulationTarget3Value>),
}


impl OperatorModulationTargetProcessingParameter {
    pub fn opt_new(operator_index: usize) -> Option<Self> {
        match operator_index {
            2 => Some(OperatorModulationTargetProcessingParameter::OperatorIndex2(
                Default::default()
            )),
            3 => Some(OperatorModulationTargetProcessingParameter::OperatorIndex3(
                Default::default()
            )),
            _ => None
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
    type ExtraData = TimeCounter;

    fn get_value(&mut self, time: Self::ExtraData) -> Self::Value {
        let mut opt_new_left_and_right = None;

        let value = self.value.get_value(time, &mut |new_panning| {
            opt_new_left_and_right =
                Some(Self::calculate_left_and_right(new_panning));
        });

        if let Some(new_left_and_right) = opt_new_left_and_right {
            self.left_and_right = new_left_and_right;
        } else if self.lfo_active {
            self.left_and_right = Self::calculate_left_and_right(value);
        }

        self.lfo_active = false;

        value
    }
    fn set_from_sync(&mut self, value: f64) {
        self.value.set_value(OperatorPanningValue::from_sync(value).get())
    }
    fn get_value_with_lfo_addition(
        &mut self,
        extra_data: Self::ExtraData,
        lfo_addition: Option<f64>
    ) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            let sync_value = OperatorPanningValue::from_processing(
                self.get_value(extra_data)
            ).to_sync();

            let new_panning = OperatorPanningValue::from_sync(
                (sync_value + lfo_addition).min(1.0).max(0.0)
            ).get();

            self.left_and_right = Self::calculate_left_and_right(new_panning);
            self.lfo_active = true;

            new_panning
        } else {
            self.get_value(extra_data)
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
    Two(SimpleProcessingParameter<Lfo1TargetParameterValue>),
    Three(SimpleProcessingParameter<Lfo1TargetParameterValue>),
    Four(SimpleProcessingParameter<Lfo1TargetParameterValue>),
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

    pub fn set_from_sync(&mut self, value: f64){
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
}
