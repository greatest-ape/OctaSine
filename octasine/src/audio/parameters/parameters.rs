use std::{f64::consts::FRAC_PI_2, marker::PhantomData};

use arrayvec::ArrayVec;

use crate::parameter_values::*;

use super::interpolatable_value::*;
use super::AudioParameter;

#[derive(Debug, Clone)]
pub struct InterpolatableAudioParameter<P: ParameterValue> {
    value: InterpolatableAudioValue,
    phantom_data: PhantomData<P>,
}

impl<P> Default for InterpolatableAudioParameter<P>
where
    P: ParameterValue<Value = f64> + Default,
{
    fn default() -> Self {
        let default = P::default().get();

        Self {
            value: InterpolatableAudioValue::new(default),
            phantom_data: PhantomData::default(),
        }
    }
}

impl<P> AudioParameter for InterpolatableAudioParameter<P>
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
    fn set_from_patch(&mut self, value: f64) {
        self.value.set_value(P::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(&mut self, lfo_addition: Option<f64>) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            let patch_value = P::new_from_audio(self.get_value()).to_patch();

            P::new_from_patch((patch_value + lfo_addition).min(1.0).max(0.0)).get()
        } else {
            self.get_value()
        }
    }
}

pub struct SimpleAudioParameter<P: ParameterValue> {
    pub value: <P as ParameterValue>::Value,
    sync_cache: f64,
}

impl<P: ParameterValue + Default> Default for SimpleAudioParameter<P> {
    fn default() -> Self {
        Self {
            value: P::default().get(),
            sync_cache: P::default().to_patch(),
        }
    }
}

impl<P> AudioParameter for SimpleAudioParameter<P>
where
    P: ParameterValue,
{
    type Value = <P as ParameterValue>::Value;

    fn advance_one_sample(&mut self) {}
    fn get_value(&self) -> Self::Value {
        self.value
    }
    fn set_from_patch(&mut self, value: f64) {
        self.sync_cache = value;
        self.value = P::new_from_patch(value).get();
    }
    fn get_value_with_lfo_addition(&mut self, lfo_addition: Option<f64>) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            P::new_from_patch((self.sync_cache + lfo_addition).min(1.0).max(0.0)).get()
        } else {
            self.get_value()
        }
    }
}

// Master volume

#[derive(Debug, Clone)]
pub struct MasterVolumeAudioParameter {
    value: InterpolatableAudioValue,
}

impl Default for MasterVolumeAudioParameter {
    fn default() -> Self {
        let default = MasterVolumeValue::default().get();

        Self {
            value: InterpolatableAudioValue::new(default),
        }
    }
}

impl AudioParameter for MasterVolumeAudioParameter {
    type Value = f64;

    fn advance_one_sample(&mut self) {
        self.value.advance_one_sample(&mut |_| ())
    }
    fn get_value(&self) -> Self::Value {
        self.value.get_value()
    }
    fn set_from_patch(&mut self, value: f64) {
        self.value
            .set_value(MasterVolumeValue::new_from_patch(value).get())
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
pub struct OperatorVolumeAudioParameter {
    value: InterpolatableAudioValue,
}

impl Default for OperatorVolumeAudioParameter {
    fn default() -> Self {
        let default = OperatorVolumeValue::default().get();

        Self {
            value: InterpolatableAudioValue::new(default),
        }
    }
}

impl AudioParameter for OperatorVolumeAudioParameter {
    type Value = f64;

    fn advance_one_sample(&mut self) {
        self.value.advance_one_sample(&mut |_| ())
    }
    fn get_value(&self) -> Self::Value {
        self.value.get_value()
    }
    fn set_from_patch(&mut self, value: f64) {
        self.value
            .set_value(OperatorVolumeValue::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(&mut self, lfo_addition: Option<f64>) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            self.get_value() * 2.0f64.powf(lfo_addition)
        } else {
            self.get_value()
        }
    }
}

#[derive(Debug, Clone)]
pub struct OperatorVolumeToggleAudioParameter {
    value: InterpolatableAudioValue,
}

impl Default for OperatorVolumeToggleAudioParameter {
    fn default() -> Self {
        Self {
            value: InterpolatableAudioValue::new(1.0),
        }
    }
}

impl AudioParameter for OperatorVolumeToggleAudioParameter {
    type Value = f64;

    fn advance_one_sample(&mut self) {
        self.value.advance_one_sample(&mut |_| ())
    }
    fn get_value(&self) -> Self::Value {
        self.value.get_value()
    }
    fn set_from_patch(&mut self, value: f64) {
        self.value
            .set_value(OperatorVolumeValue::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(&mut self, _lfo_addition: Option<f64>) -> Self::Value {
        self.get_value()
    }
}

#[derive(Debug, Clone)]
pub struct OperatorMixAudioParameter {
    value: InterpolatableAudioValue,
}

impl OperatorMixAudioParameter {
    pub fn new(operator_index: usize) -> Self {
        let value = OperatorMixValue::new(operator_index).get();

        Self {
            value: InterpolatableAudioValue::new(value),
        }
    }
}

impl AudioParameter for OperatorMixAudioParameter {
    type Value = f64;

    fn advance_one_sample(&mut self) {
        self.value.advance_one_sample(&mut |_| ())
    }
    fn get_value(&self) -> Self::Value {
        self.value.get_value()
    }
    fn set_from_patch(&mut self, value: f64) {
        self.value
            .set_value(OperatorMixValue::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(&mut self, lfo_addition: Option<f64>) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            let patch_value = OperatorMixValue::new_from_audio(self.get_value()).to_patch();

            OperatorMixValue::new_from_patch((patch_value + lfo_addition).min(1.0).max(0.0)).get()
        } else {
            self.get_value()
        }
    }
}

// Master / operator / lfo free frequency parameters with special lfo value handling

pub struct FreeFrequencyAudioParameter<P: ParameterValue<Value = f64>> {
    pub value: <P as ParameterValue>::Value,
}

impl<P: ParameterValue<Value = f64> + Default> Default for FreeFrequencyAudioParameter<P> {
    fn default() -> Self {
        Self {
            value: P::default().get(),
        }
    }
}

impl<P> AudioParameter for FreeFrequencyAudioParameter<P>
where
    P: ParameterValue<Value = f64>,
{
    type Value = <P as ParameterValue>::Value;

    fn advance_one_sample(&mut self) {}
    fn get_value(&self) -> Self::Value {
        self.value
    }
    fn set_from_patch(&mut self, value: f64) {
        self.value = P::new_from_patch(value).get();
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

    pub fn advance_one_sample(&mut self) {
        match self {
            Self::Two(p) => p.advance_one_sample(),
            Self::Three(p) => p.advance_one_sample(),
            Self::Four(p) => p.advance_one_sample(),
        }
    }
}

// Panning

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
    fn set_from_patch(&mut self, value: f64) {
        self.value
            .set_value(OperatorPanningValue::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(&mut self, lfo_addition: Option<f64>) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            let patch_value = OperatorPanningValue::new_from_audio(self.get_value()).to_patch();

            let new_panning = OperatorPanningValue::new_from_patch(
                (patch_value + lfo_addition).min(1.0).max(0.0),
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
            value: InterpolatableAudioValue::new(default),
            left_and_right: Self::calculate_left_and_right(default),
            lfo_active: false,
        }
    }
}

// LFO target parameter

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

    pub fn set_from_sync(&mut self, value: f64) {
        match self {
            Self::One(p) => p.set_from_patch(value),
            Self::Two(p) => p.set_from_patch(value),
            Self::Three(p) => p.set_from_patch(value),
            Self::Four(p) => p.set_from_patch(value),
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

// LFO amount

#[derive(Debug, Clone)]
pub struct LfoAmountAudioParameter {
    value: InterpolatableAudioValue,
}

impl Default for LfoAmountAudioParameter {
    fn default() -> Self {
        let default = LfoAmountValue::default().get();

        Self {
            value: InterpolatableAudioValue::new(default),
        }
    }
}

impl AudioParameter for LfoAmountAudioParameter {
    type Value = f64;

    fn advance_one_sample(&mut self) {
        self.value.advance_one_sample(&mut |_| ())
    }
    fn get_value(&self) -> Self::Value {
        self.value.get_value()
    }
    fn set_from_patch(&mut self, value: f64) {
        self.value
            .set_value(LfoAmountValue::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(&mut self, lfo_addition: Option<f64>) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            self.get_value() * 2.0f64.powf(lfo_addition)
        } else {
            self.get_value()
        }
    }
}
