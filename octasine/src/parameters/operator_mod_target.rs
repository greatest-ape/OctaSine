use std::borrow::Borrow;

use super::utils::*;
use super::ParameterValue;

const OPERATOR_2_PERMUTATIONS: &[ModTargetStorage] = &[
    ModTargetStorage::new(&[true]),
    ModTargetStorage::new(&[false]),
];
const OPERATOR_3_PERMUTATIONS: &[ModTargetStorage] = &[
    ModTargetStorage::new(&[false, false]),
    ModTargetStorage::new(&[true, false]),
    ModTargetStorage::new(&[false, true]),
    ModTargetStorage::new(&[true, true]),
];
const OPERATOR_4_PERMUTATIONS: &[ModTargetStorage] = &[
    ModTargetStorage::new(&[true, false, false]),
    ModTargetStorage::new(&[true, true, false]),
    ModTargetStorage::new(&[true, false, true]),
    ModTargetStorage::new(&[true, true, true]),
    ModTargetStorage::new(&[false, true, false]),
    ModTargetStorage::new(&[false, false, true]),
    ModTargetStorage::new(&[false, true, true]),
    ModTargetStorage::new(&[false, false, true]),
    ModTargetStorage::new(&[false, false, false]),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModTargetStorage {
    indices: u8,
    iter_range: u8,
}

impl ModTargetStorage {
    pub const fn new(slice: &[bool]) -> Self {
        let mut storage = Self {
            indices: 0,
            iter_range: slice.len() as u8,
        };

        let mut i = 0;

        while i < slice.len() {
            storage.indices = Self::set_bit(storage.indices, i, slice[i]);

            i += 1;
        }

        storage
    }

    pub fn set_index(&mut self, index: usize, value: bool) {
        self.indices = Self::set_bit(self.indices, index, value);
    }

    pub fn index_active(&self, index: usize) -> bool {
        (self.indices & (1 << index)) != 0
    }

    pub fn active_indices(&self) -> impl Iterator<Item = usize> + '_ {
        (0..self.iter_range).filter_map(|i| {
            if self.index_active(i.into()) {
                Some(i.into())
            } else {
                None
            }
        })
    }

    const fn set_bit(mut data: u8, index: usize, value: bool) -> u8 {
        if value {
            data |= 1 << index;
        } else {
            data &= !(1 << index);
        }

        data
    }
}

impl Default for ModTargetStorage {
    fn default() -> Self {
        Self {
            indices: 0,
            iter_range: 0,
        }
    }
}

impl std::fmt::Display for ModTargetStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (n, index) in self.active_indices().enumerate() {
            if n == 0 {
                write!(f, "{}", index + 1)?;
            } else {
                write!(f, ", {}", index + 1)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Operator2ModulationTargetValue(ModTargetStorage);

impl Default for Operator2ModulationTargetValue {
    fn default() -> Self {
        Self(ModTargetStorage::new(&[true]))
    }
}

impl Borrow<ModTargetStorage> for Operator2ModulationTargetValue {
    fn borrow(&self) -> &ModTargetStorage {
        &self.0
    }
}

impl ParameterValue for Operator2ModulationTargetValue {
    type Value = ModTargetStorage;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(_text: &str) -> Option<Self> {
        None
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(map_patch_value_to_step(OPERATOR_2_PERMUTATIONS, value))
    }
    fn to_patch(self) -> f32 {
        map_step_to_patch_value(OPERATOR_2_PERMUTATIONS, self.0)
    }
    fn get_formatted(self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Operator3ModulationTargetValue(ModTargetStorage);

impl Default for Operator3ModulationTargetValue {
    fn default() -> Self {
        Self(ModTargetStorage::new(&[false, true]))
    }
}

impl Borrow<ModTargetStorage> for Operator3ModulationTargetValue {
    fn borrow(&self) -> &ModTargetStorage {
        &self.0
    }
}

impl ParameterValue for Operator3ModulationTargetValue {
    type Value = ModTargetStorage;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(_text: &str) -> Option<Self> {
        None
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(map_patch_value_to_step(OPERATOR_3_PERMUTATIONS, value))
    }
    fn to_patch(self) -> f32 {
        map_step_to_patch_value(OPERATOR_3_PERMUTATIONS, self.0)
    }
    fn get_formatted(self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Operator4ModulationTargetValue(ModTargetStorage);

impl Default for Operator4ModulationTargetValue {
    fn default() -> Self {
        Self(ModTargetStorage::new(&[false, false, true]))
    }
}

impl Borrow<ModTargetStorage> for Operator4ModulationTargetValue {
    fn borrow(&self) -> &ModTargetStorage {
        &self.0
    }
}

impl ParameterValue for Operator4ModulationTargetValue {
    type Value = ModTargetStorage;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(_text: &str) -> Option<Self> {
        None
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(map_patch_value_to_step(OPERATOR_4_PERMUTATIONS, value))
    }
    fn to_patch(self) -> f32 {
        map_step_to_patch_value(OPERATOR_4_PERMUTATIONS, self.0)
    }
    fn get_formatted(self) -> String {
        self.0.to_string()
    }
}
