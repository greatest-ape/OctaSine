use std::cell::UnsafeCell;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

/// Non-blocking lock allowing access only from a single thread at a time
pub struct SingleAccessLock<T> {
    contents: UnsafeCell<T>,
    borrowed: Arc<AtomicBool>,
}

impl<T> SingleAccessLock<T> {
    pub fn new(contents: T) -> Self {
        Self {
            contents: UnsafeCell::new(contents),
            borrowed: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn get_mut(&self) -> Option<SingleAccessLockGuard<T>> {
        if let Ok(_) =
            self.borrowed
                .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        {
            if let Some(reference) = unsafe { self.contents.get().as_mut() } {
                return Some(SingleAccessLockGuard {
                    reference,
                    borrowed: &self.borrowed,
                });
            }
        }

        None
    }
}

unsafe impl<T> Sync for SingleAccessLock<T> {}

pub struct SingleAccessLockGuard<'a, T> {
    reference: &'a mut T,
    borrowed: &'a Arc<AtomicBool>,
}

impl<'a, T> Deref for SingleAccessLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.reference
    }
}

impl<'a, T> DerefMut for SingleAccessLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.reference
    }
}

impl<'a, T> Drop for SingleAccessLockGuard<'a, T> {
    fn drop(&mut self) {
        self.borrowed.store(false, Ordering::SeqCst);
    }
}

#[derive(Debug)]
pub struct VoiceData<const PD_WIDTH: usize> {
    pub active: bool,
    pub operator_volumes: [[f64; PD_WIDTH]; 4],
    pub operator_modulation_indices: [[f64; PD_WIDTH]; 4],
    pub operator_feedbacks: [[f64; PD_WIDTH]; 4],
    pub operator_additives: [[f64; PD_WIDTH]; 4],
    pub operator_frequencies: [[f64; PD_WIDTH]; 4],
    pub operator_pannings: [[f64; PD_WIDTH]; 4],
    pub operator_constant_power_pannings: [[f64; PD_WIDTH]; 4],
    pub operator_envelope_volumes: [[f64; PD_WIDTH]; 4],
    pub operator_phases: [[f64; PD_WIDTH]; 4],
    pub operator_wave_type: [crate::WaveType; 4],
    pub operator_modulation_targets: [usize; 4],
    pub volume_factors: [f64; 4],
}

impl Default for VoiceData<2> {
    fn default() -> Self {
        Self {
            active: false,
            operator_volumes: Default::default(),
            operator_modulation_indices: Default::default(),
            operator_feedbacks: Default::default(),
            operator_additives: Default::default(),
            operator_frequencies: Default::default(),
            operator_pannings: Default::default(),
            operator_constant_power_pannings: Default::default(),
            operator_envelope_volumes: Default::default(),
            operator_phases: Default::default(),
            operator_wave_type: Default::default(),
            operator_modulation_targets: Default::default(),
            volume_factors: Default::default(),
        }
    }
}

impl Default for VoiceData<4> {
    fn default() -> Self {
        Self {
            active: false,
            operator_volumes: Default::default(),
            operator_modulation_indices: Default::default(),
            operator_feedbacks: Default::default(),
            operator_additives: Default::default(),
            operator_frequencies: Default::default(),
            operator_pannings: Default::default(),
            operator_constant_power_pannings: Default::default(),
            operator_envelope_volumes: Default::default(),
            operator_phases: Default::default(),
            operator_wave_type: Default::default(),
            operator_modulation_targets: Default::default(),
            volume_factors: Default::default(),
        }
    }
}
