use std::sync::atomic::{AtomicU64, Ordering};

use array_init::array_init;

use super::{SyncParameter, ProcessingValue};


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

    pub fn changes_exist(&self) -> bool {
        self.changed.load(Ordering::SeqCst) != 0
    }

    /// Go through change info. Get all changed parameters. If parameters are
    /// marked as changed but are in fact not registered as changed in
    /// PresetParameters, mark them as changed in Self.changed
    pub fn get_changed_parameters(
        &self,
        parameters: &Vec<SyncParameter>
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
        parameters: &Vec<SyncParameter>
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
