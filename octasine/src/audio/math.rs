#[inline(always)]
pub fn exp2(value: f32) -> f32 {
    fast_math::exp2_raw(value)
}