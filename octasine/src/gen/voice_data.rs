use std::cell::UnsafeCell;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;

/// Lock allowing access only from a single thread at a time
pub struct SingleAccessLock<T> {
    inner: UnsafeCell<T>,
    holders: Arc<AtomicUsize>,
}

impl<T> SingleAccessLock<T> {
    pub fn new(contents: T) -> Self {
        Self {
            inner: UnsafeCell::new(contents),
            holders: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn get_mut(&self) -> Option<SingleAccessLockGuard<T>> {
        if let Ok(_) = self
            .holders
            .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
        {
            if let Some(reference) = unsafe { self.inner.get().as_mut() } {
                return Some(SingleAccessLockGuard {
                    reference,
                    holders: &self.holders,
                });
            }
        }

        None
    }
}

unsafe impl<T> Sync for SingleAccessLock<T> {}

pub struct SingleAccessLockGuard<'a, T> {
    reference: &'a mut T,
    holders: &'a Arc<AtomicUsize>,
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
        self.holders.store(0, Ordering::SeqCst);
    }
}

#[derive(Debug)]
pub struct VoiceData<const PD_WIDTH: usize, const SAMPLES: usize> {
    pub active: bool,
    pub operator_volumes: [[f64; PD_WIDTH]; 4],
    pub operator_modulation_indices: [[f64; PD_WIDTH]; 4],
    pub operator_feedbacks: [[f64; PD_WIDTH]; 4],
    pub operator_additives: [[f64; PD_WIDTH]; 4],
    pub operator_frequencies: [[f64; PD_WIDTH]; 4],
    pub operator_pannings: [[f64; SAMPLES]; 4],
    pub operator_constant_power_pannings: [[f64; PD_WIDTH]; 4],
    pub operator_envelope_volumes: [[f64; PD_WIDTH]; 4],
    pub operator_phases: [[f64; PD_WIDTH]; 4],
    pub operator_wave_type: [crate::WaveType; 4],
    pub operator_modulation_targets: [usize; 4],
    pub volume_factors: [f64; 4],
}

impl Default for VoiceData<2, 1> {
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

impl Default for VoiceData<4, 2> {
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
