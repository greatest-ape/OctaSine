use std::{
    arch::x86_64::*,
    f64::consts::{PI, TAU},
    ops::{Add, AddAssign, Mul, Sub, BitAnd, BitOr, BitXor},
};

use super::{Simd, SimdPackedDouble};

pub struct Avx;

impl Simd for Avx {
    type Pd = AvxPackedDouble;
}

/// Safety: only construct this if you know for sure that the CPU supports AVX
#[derive(Clone, Copy)]
pub struct AvxPackedDouble(__m256d);

impl SimdPackedDouble for AvxPackedDouble {
    const SAMPLES: usize = 2;

    type Arr = [f64; 4];

    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn new(value: f64) -> Self {
        Self(_mm256_set1_pd(value))
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn new_zeroed() -> Self {
        Self(_mm256_setzero_pd())
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn new_from_pair(l: f64, r: f64) -> Self {
        let lr = [l, r, l, r];

        Self(_mm256_loadu_pd(lr.as_ptr()))
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn from_arr(arr: Self::Arr) -> Self {
        Self(_mm256_loadu_pd(arr.as_ptr()))
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn load_ptr(source: *const f64) -> Self {
        Self(_mm256_loadu_pd(source))
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn to_arr(&self) -> Self::Arr {
        let mut arr = Self::Arr::default();

        _mm256_storeu_pd(arr.as_mut_ptr(), self.0);

        arr
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn min(&self, other: Self) -> Self {
        Self(_mm256_min_pd(self.0, other.0))
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn max(&self, other: Self) -> Self {
        Self(_mm256_max_pd(self.0, other.0))
    }
    //#[target_feature(enable = "avx")]
    //#[inline]
    //unsafe fn fast_sin(&self) -> Self {
    //    Self(sleef_sys::Sleef_cinz_sind4_u35avx(self.0))
    //}
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn pairwise_horizontal_sum(&self) -> Self {
        Self(_mm256_add_pd(self.0, _mm256_permute_pd(self.0, 0b0101)))
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn interleave(&self, other: Self) -> Self {
        Self(_mm256_blend_pd(self.0, other.0, 0b1010))
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn any_over_zero(&self) -> bool {
        _mm256_movemask_pd(_mm256_cmp_pd::<{ _CMP_GT_OQ }>(self.0, _mm256_setzero_pd())) != 0
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn multiply_negative_sign(self, other: Self) -> Self {
        // _mm256_blendv_pd(other.0, other.0 * _mm256_set1(-1.0), self.0)

        // _mm256_xor_pd(
        //     other,
        //     _mm256_and_pd(_mm256_set1_pd(f64::from_bits(1 << 63)), self.0),
        // )

        other ^ (Self::new(f64::from_bits(1 << 63)) & self)
    }

    #[target_feature(enable = "avx")]
    unsafe fn fast_sin(self) -> Self {
        let tau = Self::new(TAU);
        let phase = self;

        // Constrain input to range (-TAU, TAU)
        let phase = phase.constrain_to_tau_fmod();

        // Add TAU to members that are smaller than -PI
        let lt_minus_pi_mask = _mm256_cmp_pd::<{ _CMP_LT_OQ }>(self.0, _mm256_set1_pd(-PI));
        let phase = Self(_mm256_blendv_pd(phase.0, (phase + tau).0, lt_minus_pi_mask));

        // Subtract TAU from members that are larger than PI
        let gt_pi_mask = _mm256_cmp_pd::<{ _CMP_GT_OQ }>(self.0, _mm256_set1_pd(PI));
        let phase = Self(_mm256_blendv_pd(phase.0, (phase - tau).0, gt_pi_mask));

        let output = phase.chebyshev_sin();

        output
    }
}

impl AvxPackedDouble {
    #[inline]
    #[target_feature(enable = "avx")]
    unsafe fn constrain_to_tau(self) -> Self {
        self - ((self * Self::new(1.0 / TAU)).round_towards_zero() * Self::new(TAU))
    }

    #[inline(never)]
    unsafe fn constrain_to_tau_fmod(self) -> Self {
        let arr = self.to_arr();

        Self::from_arr([
            arr[0] % TAU,
            arr[1] % TAU,
            arr[2] % TAU,
            arr[3] % TAU,
        ])
    }

    #[inline]
    #[target_feature(enable = "avx")]
    unsafe fn round_towards_zero(self) -> Self {
        Self(_mm256_round_pd(self.0, _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC))
    }

    /// Fast sine approximation valid for range (-pi, pi). Originally designed
    /// for f32.
    ///
    /// Adapted from http://mooooo.ooo/chebyshev-sine-approximation/
    #[inline]
    #[target_feature(enable = "avx")]
    unsafe fn chebyshev_sin(self) -> Self {
        const COEFFICIENT_X: f64 = -0.10132118; // x
        const COEFFICIENT_X3: f64 = 0.0066208798; // x^3
        const COEFFICIENT_X5: f64 = -0.00017350505; // x^5
        const COEFFICIENT_X7: f64 = 0.0000025222919; // x^7
        const COEFFICIENT_X9: f64 = -0.000000023317787; // x^9
        const COEFFICIENT_X11: f64 = 0.00000000013291342; // x^11

        let pi_major = Self::new(3.1415927);
        let pi_minor = Self::new(-0.00000008742278);

        let x = self;
        let x2 = x * x;

        let p11 = Self::new(COEFFICIENT_X11);
        let p9 = p11 * x2 + Self::new(COEFFICIENT_X9);
        let p7 = p9 * x2 + Self::new(COEFFICIENT_X7);
        let p5 = p7 * x2 + Self::new(COEFFICIENT_X5);
        let p3 = p5 * x2 + Self::new(COEFFICIENT_X3);
        let p1 = p3 * x2 + Self::new(COEFFICIENT_X);

        (x - pi_major - pi_minor) * (x + pi_major + pi_minor) * p1 * x
    }
}

impl Add for AvxPackedDouble {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        unsafe { Self(_mm256_add_pd(self.0, rhs.0)) }
    }
}

impl AddAssign for AvxPackedDouble {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for AvxPackedDouble {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { Self(_mm256_sub_pd(self.0, rhs.0)) }
    }
}

impl Mul for AvxPackedDouble {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        unsafe { Self(_mm256_mul_pd(self.0, rhs.0)) }
    }
}

impl BitAnd for AvxPackedDouble {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe { Self(_mm256_and_pd(self.0, rhs.0)) }
    }
}

impl BitOr for AvxPackedDouble {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { Self(_mm256_or_pd(self.0, rhs.0)) }
    }
}

impl BitXor for AvxPackedDouble {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe { Self(_mm256_xor_pd(self.0, rhs.0)) }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::*;

    use super::*;

    fn input_invalid(a: f64) -> bool {
        a.is_infinite() || a.is_nan()
    }

    #[test]
    fn test_fast_sin() {
        assert!(is_x86_feature_detected!("avx"));

        fn prop(a: f64, b: f64, c: f64, d: f64) -> TestResult {
            let arr = [a, b, c, d];

            for x in arr.iter().copied() {
                if input_invalid(x) {
                    return TestResult::discard();
                }
            }

            #[target_feature(enable = "avx")]
            unsafe fn fast_sin(arr: [f64; 4]) -> [f64; 4] {
                AvxPackedDouble::from_arr(arr).fast_sin().to_arr()
            }

            #[target_feature(enable = "avx")]
            unsafe fn fast_sin_pre_rem(arr: [f64; 4]) -> [f64; 4] {
                let arr = [
                    arr[0] % TAU,
                    arr[1] % TAU,
                    arr[2] % TAU,
                    arr[3] % TAU,
                ];
                AvxPackedDouble::from_arr(arr).fast_sin().to_arr()
            }

            #[target_feature(enable = "avx")]
            unsafe fn constrain_to_tau(arr: [f64; 4]) -> [f64; 4] {
                AvxPackedDouble::from_arr(arr).constrain_to_tau().to_arr()
            }

            let result = unsafe { fast_sin(arr) };

            for (i, (input, result)) in arr.into_iter().zip(result).enumerate() {
                let reference = input.sin();
                let diff = (result - reference).abs();

                if diff > 10.0f64.powi(-6) {
                    let result_rem = unsafe {
                        fast_sin_pre_rem(arr)
                    };

                    let constrained = unsafe {
                        constrain_to_tau(arr)
                    };

                    println!("input:            {}", input);
                    println!("result:           {}", result);
                    println!("reference:        {}", reference);
                    println!("result (pre rem): {}", result_rem[i]);
                    println!("diff:             {}", diff);
                    println!("diff (pre rem):   {}", (result_rem[i] - reference).abs());
                    println!("index:            {}", i);
                    println!("constrained:      {}", constrained[i]);
                    println!("rem:              {}", input % TAU);
                    println!();

                    return TestResult::failed();
                }
            }

            TestResult::passed()
        }

        quickcheck(prop as fn(f64, f64, f64, f64) -> TestResult);
    }
}