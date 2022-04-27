use super::utils::*;
use super::ParameterValue;

pub trait ModTarget: Copy + std::fmt::Display {
    fn set_index(&mut self, index: usize, value: bool);
    fn index_active(&self, index: usize) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModTargetStorage<const N: usize>([bool; N]);

impl<const N: usize> ModTargetStorage<N> {
    pub fn active_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.0
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(index, active)| if active { Some(index) } else { None })
    }
}

impl<const N: usize> std::fmt::Display for ModTargetStorage<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

impl ModTargetStorage<1> {
    pub fn permutations() -> &'static [Self] {
        &[ModTargetStorage([true]), ModTargetStorage([false])]
    }
}

impl ModTargetStorage<2> {
    pub fn permutations() -> &'static [Self] {
        &[
            ModTargetStorage([false, false]),
            ModTargetStorage([true, false]),
            ModTargetStorage([false, true]),
            ModTargetStorage([true, true]),
        ]
    }
}

impl ModTargetStorage<3> {
    pub fn permutations() -> &'static [Self] {
        &[
            ModTargetStorage([true, false, false]),
            ModTargetStorage([true, true, false]),
            ModTargetStorage([true, false, true]),
            ModTargetStorage([true, true, true]),
            ModTargetStorage([false, true, false]),
            ModTargetStorage([false, false, true]),
            ModTargetStorage([false, true, true]),
            ModTargetStorage([false, false, true]),
            ModTargetStorage([false, false, false]),
        ]
    }
}

impl Default for ModTargetStorage<1> {
    fn default() -> Self {
        Self([true])
    }
}

impl Default for ModTargetStorage<2> {
    fn default() -> Self {
        Self([false, true])
    }
}

impl Default for ModTargetStorage<3> {
    fn default() -> Self {
        Self([false, false, true])
    }
}

impl<const N: usize> ModTarget for ModTargetStorage<N> {
    fn set_index(&mut self, index: usize, value: bool) {
        self.0[index] = value;
    }

    fn index_active(&self, index: usize) -> bool {
        self.0[index]
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Operator2ModulationTargetValue(ModTargetStorage<1>);

impl ParameterValue for Operator2ModulationTargetValue {
    type Value = ModTargetStorage<1>;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(
            Self::Value::permutations(),
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(Self::Value::permutations(), self.0)
    }
    fn format(self) -> String {
        self.0.to_string()
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Operator3ModulationTargetValue(ModTargetStorage<2>);

impl ParameterValue for Operator3ModulationTargetValue {
    type Value = ModTargetStorage<2>;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(
            Self::Value::permutations(),
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(Self::Value::permutations(), self.0)
    }
    fn format(self) -> String {
        self.0.to_string()
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Operator4ModulationTargetValue(ModTargetStorage<3>);

impl ParameterValue for Operator4ModulationTargetValue {
    type Value = ModTargetStorage<3>;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(map_parameter_value_to_step(
            Self::Value::permutations(),
            sync,
        ))
    }
    fn to_sync(self) -> f64 {
        map_step_to_parameter_value(Self::Value::permutations(), self.0)
    }
    fn format(self) -> String {
        self.0.to_string()
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}
