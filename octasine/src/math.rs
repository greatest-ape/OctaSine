#[inline(always)]
pub fn exp2_fast(value: f32) -> f32 {
    fast_math::exp2_raw(value)
}

cfg_if::cfg_if! {
    if #[cfg(feature = "simd")] {
        #[inline(always)]
        pub fn sin(value: f32) -> f32 {
            unsafe {
                ::sleef_sys::Sleef_cinz_sinf1_u35purec(value)
            }
        }
        #[inline(always)]
        pub fn cos(value: f32) -> f32 {
            unsafe {
                ::sleef_sys::Sleef_cinz_cosf1_u35purec(value)
            }
        }
    } else {
        #[inline(always)]
        pub fn sin(value: f32) -> f32 {
            value.sin()
        }
        #[inline(always)]
        pub fn cos(value: f32) -> f32 {
            value.cos()
        }
    }
}
