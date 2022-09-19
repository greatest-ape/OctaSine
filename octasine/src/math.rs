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

/// Approximate cos(a * PI / 2) for range 0.0 to 1.0
#[allow(dead_code)]
pub fn bhaskara_cos_frac_pi_2(a: f32) -> f32 {
    let a2 = a * a;

    4.0 * ((1.0 - a2) / (4.0 + a2))
}

/// Approximate sin(a * PI / 2) for range 0.0 to 1.0
#[allow(dead_code)]
pub fn bhaskara_sin_frac_pi_2(a: f32) -> f32 {
    bhaskara_cos_frac_pi_2(1.0 - a)
}

#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_PI_2;

    use super::*;

    #[test]
    fn test_bhaskara() {
        let precision = u16::MAX;

        for i in 0..=precision {
            let a = f32::from(i) / f32::from(precision);

            assert_approx_eq::assert_approx_eq!(
                bhaskara_cos_frac_pi_2(a),
                (a * FRAC_PI_2).cos(),
                0.005
            );
            assert_approx_eq::assert_approx_eq!(
                bhaskara_sin_frac_pi_2(a),
                (a * FRAC_PI_2).sin(),
                0.005
            );
        }
    }
}
