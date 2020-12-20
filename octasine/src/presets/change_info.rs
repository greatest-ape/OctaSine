use std::sync::atomic::{AtomicU64, Ordering};

use array_init::array_init;

use super::{PresetParameter, ProcessingValue};


/// Cache for marking parameters as changed and listing them.
pub struct ParameterChangeInfo {
    changed: AtomicU64,
    index_masks: [u64; 64],
}


impl ParameterChangeInfo {
    pub fn mark_as_changed(&self, index: usize){
        if index > 63 {
            return;
        }

        self.changed.fetch_or(self.index_masks[index], Ordering::SeqCst);
    }

    pub fn mark_all_as_changed(&self){
        self.changed.store(!0u64, Ordering::SeqCst);
    }

    #[cfg(test)]
    pub fn changes_exist(&self) -> bool {
        self.changed.load(Ordering::SeqCst) != 0
    }

    /// Go through change info. Get all changed parameters. If parameters are
    /// marked as changed but are in fact not registered as changed in
    /// PresetParameters, mark them as changed in Self.changed
    pub fn get_changed_parameters(
        &self,
        parameters: &Vec<PresetParameter>
    ) -> Option<[Option<f64>; 64]> {
        let changed = self.changed.fetch_and(0, Ordering::SeqCst);

        if changed == 0 {
            return None;
        }

        let mut changes = [None; 64];

        for (index, c) in changes.iter_mut().enumerate(){
            if (changed >> index) & 1 == 1 {
                if let Some(p) = parameters.get(index){
                    if let Some(value) = p.value.get_if_changed(){
                        *c = Some(value);
                    } else {
                        self.mark_as_changed(index);
                    }
                }
            }
        }

        Some(changes)
    }

    /// Get all changed parameters, without reading or modifying the changed flag
    /// in the AtomicPositiveDouble
    pub fn get_changed_parameters_transient(
        &self,
        parameters: &Vec<PresetParameter>
    ) -> Option<[Option<f64>; 64]> {
        let changed = self.changed.fetch_and(0, Ordering::SeqCst);

        if changed == 0 {
            return None;
        }

        let mut changes = [None; 64];

        for (index, c) in changes.iter_mut().enumerate(){
            if (changed >> index) & 1 == 1 {
                if let Some(p) = parameters.get(index){
                    *c = Some(p.value.get());
                }
            }
        }

        Some(changes)
    }
}


impl Default for ParameterChangeInfo {
    fn default() -> Self {
        Self {
            changed: AtomicU64::new(0),
            index_masks: array_init(|i| 2u64.pow(i as u32))
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use quickcheck::{TestResult, quickcheck};

    use super::super::Preset;
    use super::*;

    #[test]
    fn test_changed_parameters(){
        let c = ParameterChangeInfo::default();

        // Not checked
        let preset_parameters = Preset::new("1".to_string()).parameters;

        assert!(c.get_changed_parameters(&preset_parameters).is_none());

        preset_parameters.get(0).unwrap().value.set(1.0);
        preset_parameters.get(10).unwrap().value.set(1.0);
        c.mark_as_changed(0);
        c.mark_as_changed(10);

        let indeces: Vec<usize> = c.get_changed_parameters(&preset_parameters)
            .unwrap()
            .iter()
            .enumerate()
            .filter_map(|(index, changed)| if changed.is_some() { Some(index) } else { None })
            .collect();

        println!("{:?}", indeces);
        assert!(indeces.len() == 2);
        assert!(indeces[0] == 0);
        assert!(indeces[1] == 10);

        preset_parameters.get(1).unwrap().value.set(1.0);
        preset_parameters.get(4).unwrap().value.set(1.0);
        preset_parameters.get(5).unwrap().value.set(1.0);
        c.mark_as_changed(1);
        c.mark_as_changed(4);
        c.mark_as_changed(5);

        let indeces: Vec<usize> = c.get_changed_parameters(&preset_parameters)
            .unwrap()
            .iter()
            .enumerate()
            .filter_map(|(index, changed)| if changed.is_some() { Some(index) } else { None })
            .collect();

        println!("{:?}", indeces);
        assert!(indeces.len() == 3);
        assert!(indeces[0] == 1);
        assert!(indeces[1] == 4);
        assert!(indeces[2] == 5);
    }

    #[test]
    fn test_changed_parameters_quickcheck(){
        fn prop(data: Vec<(usize, f64)>) -> TestResult {
            let preset_parameters = Preset::new("1".to_string()).parameters;

            for (i, v) in data.iter(){
                if *i > 63 || *v < 0.0 {
                    return TestResult::discard();
                }
            }

            fn f(
                c: &ParameterChangeInfo,
                preset_parameters: &Vec<PresetParameter>,
                data: &[(usize, f64)]
            ) -> bool {
                let mut set_parameters = HashMap::new();

                for (index, value) in data.iter(){
                    if let Some(p) = preset_parameters.get(*index){
                        p.value.set(*value + 1.43432);
                        p.value.set(*value + 5.55);
                        p.value.set(*value);
                        
                        set_parameters.insert(*index, *value);
                    }

                    c.mark_as_changed(*index);
                }

                if let Some(changed_parameters) = c.get_changed_parameters(preset_parameters){
                    let results: HashMap<usize, f64> = changed_parameters
                        .iter()
                        .enumerate()
                        .filter_map(|(index, opt_value)| {
                            if let Some(value) = opt_value {
                                Some((index, *value))
                            } else {
                                None
                            }
                        }).collect();

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
                if !f(&c, &preset_parameters, &data) {
                    return TestResult::from_bool(false);
                }
            }

            let changes_exist = c.changes_exist();

            if changes_exist {
                println!("changes exist");
            }

            TestResult::from_bool(!changes_exist)
        }

        quickcheck(prop as fn(Vec<(usize, f64)>) -> TestResult);
    }
}