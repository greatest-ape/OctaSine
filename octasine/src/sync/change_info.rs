use std::sync::atomic::{AtomicU64, Ordering};

use array_init::array_init;

use crate::{common::IndexMap, parameters::ParameterKey};

use super::parameters::PatchParameter;

const NUM_ATOMIC_U64S: usize = 2;
pub const MAX_NUM_PARAMETERS: usize = NUM_ATOMIC_U64S * 64;

/// Cache for marking parameters as changed and listing them.
pub struct ParameterChangeInfo {
    atomic_u64s: [AtomicU64; NUM_ATOMIC_U64S],
    index_masks: [u64; 64],
}

impl ParameterChangeInfo {
    pub fn mark_as_changed(&self, parameter_index: usize) {
        if parameter_index > MAX_NUM_PARAMETERS - 1 {
            return;
        }

        let atomic_u64_index = parameter_index / 64;
        let atomic_u64_bit = parameter_index % 64;

        self.atomic_u64s[atomic_u64_index]
            .fetch_or(self.index_masks[atomic_u64_bit], Ordering::SeqCst);
    }

    pub fn mark_all_as_changed(&self) {
        for atomic_u64 in self.atomic_u64s.iter() {
            atomic_u64.store(!0u64, Ordering::SeqCst);
        }
    }

    #[cfg(test)]
    pub fn changes_exist(&self) -> bool {
        for atomic_u64 in self.atomic_u64s.iter() {
            if atomic_u64.load(Ordering::SeqCst) != 0 {
                return true;
            }
        }

        false
    }

    /// Get all changed parameters
    pub fn get_changed_parameters(
        &self,
        parameters: &IndexMap<ParameterKey, PatchParameter>,
    ) -> Option<[Option<f32>; MAX_NUM_PARAMETERS]> {
        let mut no_changes = true;
        let mut changed = [0u64; NUM_ATOMIC_U64S];

        for (c, atomic_u64) in changed.iter_mut().zip(self.atomic_u64s.iter()) {
            let changed = atomic_u64.fetch_and(0, Ordering::SeqCst);

            no_changes &= changed == 0;

            *c = changed;
        }

        if no_changes {
            return None;
        }

        let mut changes = [None; MAX_NUM_PARAMETERS];

        for (parameter_index, c) in changes.iter_mut().enumerate() {
            let u64_index = parameter_index / 64;
            let u64_bit = parameter_index % 64;

            let changed = changed[u64_index];

            if (changed >> u64_bit) & 1 == 1 {
                if let Some((_, p)) = parameters.get_index(parameter_index) {
                    *c = Some(p.get_value());
                }
            }
        }

        Some(changes)
    }
}

impl Default for ParameterChangeInfo {
    fn default() -> Self {
        Self {
            atomic_u64s: array_init(|_| AtomicU64::new(0)),
            index_masks: array_init(|i| 2u64.pow(i as u32)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use quickcheck::{quickcheck, TestResult};

    use super::*;

    #[test]
    fn test_changed_parameters() {
        let c = ParameterChangeInfo::default();

        // Not checked
        let patch_parameters = PatchParameter::all();

        assert!(c.get_changed_parameters(&patch_parameters).is_none());

        patch_parameters.get_index(0).unwrap().1.set_value(1.0);
        patch_parameters.get_index(10).unwrap().1.set_value(1.0);
        c.mark_as_changed(0);
        c.mark_as_changed(10);

        let indeces: Vec<usize> = c
            .get_changed_parameters(&patch_parameters)
            .unwrap()
            .iter()
            .enumerate()
            .filter_map(
                |(index, changed)| {
                    if changed.is_some() {
                        Some(index)
                    } else {
                        None
                    }
                },
            )
            .collect();

        println!("{:?}", indeces);
        assert!(indeces.len() == 2);
        assert!(indeces[0] == 0);
        assert!(indeces[1] == 10);

        patch_parameters.get_index(1).unwrap().1.set_value(1.0);
        patch_parameters.get_index(4).unwrap().1.set_value(1.0);
        patch_parameters.get_index(5).unwrap().1.set_value(1.0);
        c.mark_as_changed(1);
        c.mark_as_changed(4);
        c.mark_as_changed(5);

        let indeces: Vec<usize> = c
            .get_changed_parameters(&patch_parameters)
            .unwrap()
            .iter()
            .enumerate()
            .filter_map(
                |(index, changed)| {
                    if changed.is_some() {
                        Some(index)
                    } else {
                        None
                    }
                },
            )
            .collect();

        println!("{:?}", indeces);
        assert!(indeces.len() == 3);
        assert!(indeces[0] == 1);
        assert!(indeces[1] == 4);
        assert!(indeces[2] == 5);
    }

    #[test]
    fn test_changed_parameters_quickcheck() {
        fn prop(data: Vec<(usize, f32)>) -> TestResult {
            let patch_parameters = PatchParameter::all();

            for (i, v) in data.iter() {
                if *i > 63 || *v < 0.0 || v.is_nan() {
                    return TestResult::discard();
                }
            }

            fn f(
                c: &ParameterChangeInfo,
                preset_parameters: &IndexMap<ParameterKey, PatchParameter>,
                data: &[(usize, f32)],
            ) -> bool {
                let mut set_parameters = HashMap::new();

                for (index, value) in data.iter() {
                    if let Some((_, p)) = preset_parameters.get_index(*index) {
                        p.set_value(*value + 1.43432);
                        p.set_value(*value + 5.55);
                        p.set_value(*value);

                        set_parameters.insert(*index, *value);
                    }

                    c.mark_as_changed(*index);
                }

                if let Some(changed_parameters) = c.get_changed_parameters(preset_parameters) {
                    let results: HashMap<usize, f32> = changed_parameters
                        .iter()
                        .enumerate()
                        .filter_map(|(index, opt_value)| {
                            if let Some(value) = opt_value {
                                Some((index, *value))
                            } else {
                                None
                            }
                        })
                        .collect();

                    let success = results == set_parameters;

                    if !success {
                        println!("results: {:#?}", results);
                        println!("reference: {:#?}", set_parameters);
                    }

                    success
                } else {
                    data.is_empty()
                }
            }

            let c = ParameterChangeInfo::default();

            for _ in 0..2 {
                if !f(&c, &patch_parameters, &data) {
                    return TestResult::from_bool(false);
                }
            }

            let changes_exist = c.changes_exist();

            if changes_exist {
                println!("changes exist");
            }

            TestResult::from_bool(!changes_exist)
        }

        quickcheck(prop as fn(Vec<(usize, f32)>) -> TestResult);
    }
}
