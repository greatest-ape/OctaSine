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
        #[inline(always)]
        pub fn exp2(value: f32) -> f32 {
            unsafe {
                ::sleef_sys::Sleef_cinz_exp2f1_u10purec(value)
            }
        }
        #[inline(always)]
        pub fn log2(value: f32) -> f32 {
            unsafe {
                ::sleef_sys::Sleef_cinz_log2f1_u10purec(value)
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
        #[inline(always)]
        pub fn exp2(value: f32) -> f32 {
            value.exp2()
        }
        #[inline(always)]
        pub fn log2(value: f32) -> f32 {
            value.log2()
        }
    }
}
