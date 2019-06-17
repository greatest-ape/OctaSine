use std::sync::atomic::{AtomicU64, Ordering};


const SET_NOT_CHANGED_MASK: u64 = (!0) ^ (1 << 63);


/// Atomic float that uses a bit to store if it has been changed or not.
/// When calling .get_if_changed(), only return the value if changed bit
/// is set, and set the bit to zero.
#[derive(Debug)]
pub struct AtomicFloat {
    value: AtomicU64,
}

impl AtomicFloat {
    pub fn new(value: f32) -> Self {
        Self {
            value: AtomicU64::new(value.to_bits() as u64),
        }
    }

    pub fn get(&self) -> f32 {
        Self::convert_to_f32(self.value.load(Ordering::Relaxed))
    }

    pub fn get_if_changed(&self) -> Option<f32> {
        let value = self.value.fetch_and(SET_NOT_CHANGED_MASK, Ordering::Relaxed);

        if (value >> 63) & 1 == 1 {
            Some(Self::convert_to_f32(value))
        } else {
            None
        }
    }

    fn convert_to_f32(value: u64) -> f32 {
        f32::from_bits((value & SET_NOT_CHANGED_MASK) as u32)
    }

    pub fn set(&self, value: f32){
        let value = (value.to_bits() as u64) | (1 << 63);

        self.value.store(value, Ordering::Relaxed);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_float(){
        let a = 13.5;

        let atomic_float = AtomicFloat::new(a);

        assert_eq!(atomic_float.get(), a);

        for i in 0..100 {
            let b = 23896.3487 - i as f32;

            atomic_float.set(b);

            assert_eq!(atomic_float.get(), b);

            assert_eq!(atomic_float.get_if_changed(), Some(b));

            assert_eq!(atomic_float.get(), b);

            assert_eq!(atomic_float.get_if_changed(), None);
            assert_eq!(atomic_float.get_if_changed(), None);

            assert_eq!(atomic_float.get(), b);
        }
    }
}