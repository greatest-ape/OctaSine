#[inline(always)]
pub fn exp2_fast(value: f32) -> f32 {
    fast_math::exp2_raw(value)
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

/// Approximate [cos(a * PI / 2), sin(a * PI / 2)] for range 0.0 to 1.0
#[allow(dead_code)]
pub fn bhaskara_constant_power_panning(pan: f32) -> [f32; 2] {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            unsafe {
                use std::arch::x86_64::*;

                let mut arr = [pan, 1.0 - pan, 0.0, 0.0];

                let a = _mm_loadu_ps(arr.as_ptr());
                let a2 = _mm_mul_ps(a, a);

                let result = _mm_mul_ps(_mm_set1_ps(4.0),
                    _mm_div_ps(
                        _mm_sub_ps(_mm_set1_ps(1.0), a2),
                        _mm_add_ps(_mm_set1_ps(4.0), a2),
                    )
                );

                _mm_storeu_ps(arr.as_mut_ptr(), result);

                let mut output = [0.0f32; 2];

                output.copy_from_slice(&arr[..2]);

                output
            }
        } else {
            [
                bhaskara_cos_frac_pi_2(pan),
                bhaskara_sin_frac_pi_2(pan),
            ]
        }
    }
}

pub fn saw(x: f64) -> f64 {
    const DOWN_FACTOR: f64 = 50.0;
    const DOWN_FACTOR_RECIP: f64 = 1.0 / DOWN_FACTOR;
    const INTERCEPT: f64 = 1.0 - DOWN_FACTOR_RECIP;
    const UP_FACTOR: f64 = 1.0 / (1.0 - DOWN_FACTOR_RECIP);

    let x_is_negative = x.is_sign_negative();

    let mut x = x.abs().fract();

    if x_is_negative {
        x = 1.0 - x;
    }

    let up = x * UP_FACTOR;
    let down = DOWN_FACTOR - DOWN_FACTOR * x;

    let y = if x < INTERCEPT { up } else { down };

    (y - 0.5) * 2.0
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

            let [l, r] = bhaskara_constant_power_panning(a);

            assert_approx_eq::assert_approx_eq!(l, (a * FRAC_PI_2).cos(), 0.005);
            assert_approx_eq::assert_approx_eq!(r, (a * FRAC_PI_2).sin(), 0.005);
        }
    }
}
