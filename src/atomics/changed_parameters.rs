use std::sync::atomic::{AtomicU64, Ordering};

use array_init::array_init;

use crate::parameters::sync::SyncParameters;


/// Cache for marking parameters as changed and listing them.
pub struct ChangedParametersInfo {
    changed: AtomicU64,
    index_masks: [u64; 64],
}


impl ChangedParametersInfo {
    pub fn new() -> Self {
        Self {
            changed: AtomicU64::new(0),
            index_masks: array_init(|i| 2u64.pow(i as u32))
        }
    }

    pub fn mark_as_changed(&self, index: usize){
        if index > 63 {
            return;
        }

        self.changed.fetch_or(self.index_masks[index], Ordering::SeqCst);
    }

    #[cfg(test)]
    pub fn changes_exist(&self) -> bool {
        self.changed.load(Ordering::SeqCst) != 0
    }

    /// Go through change info. Get all changed parameters. If parameters are
    /// marked as changed but are in fact not registered as changed in
    /// SyncParameters, mark them as changed in Self.changed
    pub fn get_changed_parameters(
        &self,
        sync_parameters: &SyncParameters
    ) -> Option<[Option<f32>; 64]> {
        let changed = self.changed.fetch_and(0, Ordering::SeqCst);

        if changed == 0 {
            return None;
        }

        let mut changes = [None; 64];

        for index in 0..64 {
            if (changed >> index) & 1 == 1 {
                if let Some(p) = sync_parameters.get(index){
                    if let Some(value) = p.get_parameter_value_float_if_changed() {
                        changes[index] = Some(value);
                    } else {
                        self.mark_as_changed(index);
                    }
                }
            }
        }

        Some(changes)
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use quickcheck::{TestResult, quickcheck};

    use super::*;

    #[test]
    fn test_changed_parameters(){
        let c = ChangedParametersInfo::new();

        // Not checked
        let sync_parameters = SyncParameters::new();

        assert!(c.get_changed_parameters(&sync_parameters).is_none());

        sync_parameters.get(0).unwrap().set_parameter_value_float(1.0);
        sync_parameters.get(10).unwrap().set_parameter_value_float(1.0);
        c.mark_as_changed(0);
        c.mark_as_changed(10);

        let indeces: Vec<usize> = c.get_changed_parameters(&sync_parameters)
            .unwrap()
            .iter()
            .enumerate()
            .filter_map(|(index, changed)| if changed.is_some() { Some(index) } else { None })
            .collect();

        println!("{:?}", indeces);
        assert!(indeces.len() == 2);
        assert!(indeces[0] == 0);
        assert!(indeces[1] == 10);

        sync_parameters.get(1).unwrap().set_parameter_value_float(1.0);
        sync_parameters.get(4).unwrap().set_parameter_value_float(1.0);
        sync_parameters.get(5).unwrap().set_parameter_value_float(1.0);
        c.mark_as_changed(1);
        c.mark_as_changed(4);
        c.mark_as_changed(5);

        let indeces: Vec<usize> = c.get_changed_parameters(&sync_parameters)
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
        fn prop(data: Vec<(usize, f32)>) -> TestResult {
            let sync_parameters = SyncParameters::new();

            if data.iter().any(|(i, _)| *i > 63) {
                return TestResult::discard();
            }

            fn f(
                c: &ChangedParametersInfo,
                sync_parameters: &SyncParameters,
                data: &Vec<(usize, f32)>
            ) -> bool {
                let mut set_parameters = HashMap::new();

                for (index, value) in data.iter(){
                    if let Some(p) = sync_parameters.get(*index){
                        p.set_parameter_value_float(*value + 1.43432);
                        p.set_parameter_value_float(*value - 1.55);
                        p.set_parameter_value_float(*value);
                        
                        set_parameters.insert(*index, *value);
                    }

                    c.mark_as_changed(*index);
                }

                if let Some(changed_parameters) = c.get_changed_parameters(&sync_parameters){
                    let results: HashMap<usize, f32> = changed_parameters
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
                    data.len() == 0
                }
            }

            let c = ChangedParametersInfo::new();

            for _ in 0..2 {
                if !f(&c, &sync_parameters, &data) {
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