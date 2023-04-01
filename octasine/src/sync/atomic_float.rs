use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug)]
pub struct AtomicFloat(AtomicU32);

impl AtomicFloat {
    pub fn new(value: f32) -> Self {
        Self(AtomicU32::new(value.to_bits()))
    }

    #[inline]
    pub fn get(&self) -> f32 {
        f32::from_bits(self.0.load(Ordering::Relaxed))
    }

    #[inline]
    pub fn set(&self, value: f32) {
        self.0.store(value.to_bits(), Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::float_cmp)]
    #[test]
    fn test_atomic_double() {
        let a = 13.5;

        let atomic_float = AtomicFloat::new(a);

        assert_eq!(atomic_float.get(), a);

        for i in 0..100 {
            let b = 23_896.35 - i as f32;

            atomic_float.set(b);

            assert_eq!(atomic_float.get(), b);
        }
    }
}
