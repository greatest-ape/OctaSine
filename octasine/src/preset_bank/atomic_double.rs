use std::sync::atomic::{AtomicU64, Ordering};

/// Binary `AND` with this to set changed bit to false.
const SET_NOT_CHANGED_MASK: u64 = !(1 << 63);

/// Atomic double that uses sign bit to store if it has been changed or not.
/// When calling .get_if_changed(), only return the value if changed bit
/// is set, and set the bit to zero (will discard any negative sign)
#[derive(Debug)]
pub struct AtomicPositiveDouble {
    value: AtomicU64,
}

impl AtomicPositiveDouble {
    pub fn new(value: f64) -> Self {
        Self {
            value: AtomicU64::new(value.to_bits()),
        }
    }

    #[inline]
    pub fn get(&self) -> f64 {
        Self::convert_to_f64(self.value.load(Ordering::Relaxed))
    }

    #[inline]
    pub fn get_if_changed(&self) -> Option<f64> {
        let value = self
            .value
            .fetch_and(SET_NOT_CHANGED_MASK, Ordering::Relaxed);

        if (value >> 63) & 1 == 1 {
            Some(Self::convert_to_f64(value))
        } else {
            None
        }
    }

    #[inline]
    fn convert_to_f64(value: u64) -> f64 {
        f64::from_bits((value & SET_NOT_CHANGED_MASK) as u64)
    }

    #[inline]
    pub fn set(&self, value: f64) {
        let value = value.to_bits() | (1 << 63);

        self.value.store(value, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::float_cmp)]
    #[test]
    fn test_atomic_double() {
        let a = 13.5;

        let atomic_double = AtomicPositiveDouble::new(a);

        assert_eq!(atomic_double.get(), a);

        for i in 0..100 {
            let b = 23896.3487 - i as f64;

            atomic_double.set(b);

            assert_eq!(atomic_double.get(), b);

            assert_eq!(atomic_double.get_if_changed(), Some(b));

            assert_eq!(atomic_double.get(), b);

            assert_eq!(atomic_double.get_if_changed(), None);
            assert_eq!(atomic_double.get_if_changed(), None);

            assert_eq!(atomic_double.get(), b);
        }
    }
}
