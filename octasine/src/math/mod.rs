pub mod bhaskara;
pub mod wave;

#[inline(always)]
pub fn exp2_fast(value: f32) -> f32 {
    fast_math::exp2_raw(value)
}
