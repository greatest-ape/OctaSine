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

/// Approximate a square wave
///
/// Check absence of branches with:
/// cargo asm --lib --no-default-features --full-name --rust -p octasine "octasine::math::square"
pub fn square(x: f64) -> f64 {
    // If x is negative, final result should be negated
    let negate_if_x_negative: f64 = if x.is_sign_negative() { -1.0 } else { 1.0 };

    // x is now between 0.0 and 1.0
    let mut x = x.abs().fract();

    // If x > 0.5, final result should be negated
    let negate_if_x_gt_half: f64 = if x > 0.5 { -1.0 } else { 1.0 };

    let sign_mask = negate_if_x_negative.to_bits() ^ negate_if_x_gt_half.to_bits();

    // Adjust for x > 0.5
    if x > 0.5 {
        x = 1.0 - x;
    }

    // Higher values cause "tighter interpolation"
    const FIT: f64 = 128.0;

    let approximation = 2.0 * ((1.0 / (1.0 + (x * 4.0 - 1.0).powf(FIT))) - 0.5);

    f64::from_bits(approximation.to_bits() ^ sign_mask)
}

pub fn triangle(x: f64) -> f64 {
    let x = x + 0.25;

    (2.0 * (2.0 * (x - (x + 0.5).floor())).abs()) - 1.0
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
