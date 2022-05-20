#[inline(always)]
pub fn exp2(value: f32) -> f32 {
    fast_math::exp2_raw(value)
}

cfg_if::cfg_if! {
    if #[cfg(feature = "simd")] {
        pub fn sin(value: f32) -> f32 {
            unsafe {
                ::sleef_sys::Sleef_cinz_sinf1_u35purec(value)
            }
        }
        pub fn cos(value: f32) -> f32 {
            unsafe {
                ::sleef_sys::Sleef_cinz_cosf1_u35purec(value)
            }
        }
    } else {
        pub fn sin(value: f32) -> f32 {
            value.sin()
        }
        pub fn cos(value: f32) -> f32 {
            value.cos()
        }
    }
}
