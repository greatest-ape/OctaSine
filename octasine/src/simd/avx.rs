use std::{
    arch::x86_64::*,
    ops::{Add, AddAssign, Mul, Sub},
};

use super::{Simd, SimdPackedDouble};

pub struct Avx;

impl Simd for Avx {
    type Pd = AvxPackedDouble;
}

/// Safety: only construct this if you know for sure that the CPU supports AVX
#[derive(Clone, Copy)]
pub struct AvxPackedDouble(__m256d);

impl AvxPackedDouble {
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn fract(self) -> Self {
        const TRUNCATE: i32 = _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC;

        Self(_mm256_sub_pd(
            self.0,
            _mm256_round_pd::<{ TRUNCATE }>(self.0),
        ))
    }
}

impl SimdPackedDouble for AvxPackedDouble {
    const WIDTH: usize = 4;

    type Arr = [f64; Self::WIDTH];

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
        Self(_mm256_set_pd(r, l, r, l))
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn from_arr(arr: Self::Arr) -> Self {
        Self(_mm256_loadu_pd(arr.as_ptr()))
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn to_arr(self) -> Self::Arr {
        let mut arr = Self::Arr::default();

        _mm256_storeu_pd(arr.as_mut_ptr(), self.0);

        arr
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn min(self, other: Self) -> Self {
        Self(_mm256_min_pd(self.0, other.0))
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn max(self, other: Self) -> Self {
        Self(_mm256_max_pd(self.0, other.0))
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn pairwise_horizontal_sum(self) -> Self {
        Self(_mm256_add_pd(self.0, _mm256_permute_pd(self.0, 0b0101)))
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn interleave(self, other: Self) -> Self {
        Self(_mm256_blend_pd(self.0, other.0, 0b1010))
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn any_over_zero(self) -> bool {
        _mm256_movemask_pd(_mm256_cmp_pd::<{ _CMP_GT_OQ }>(self.0, _mm256_setzero_pd())) != 0
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn floor(self) -> Self {
        Self(_mm256_floor_pd(self.0))
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn abs(self) -> Self {
        Self(_mm256_andnot_pd(_mm256_set1_pd(-0.0), self.0))
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn fast_sin(self) -> Self {
        Self(sleef_trig::Sleef_sind4_u35avx(self.0))
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn triangle(mut self) -> Self {
        self += Self::new(0.25);

        let two = Self::new(2.0);

        (two * (two * (self - (self + Self::new(0.5)).floor())).abs()) - Self::new(1.0)
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn square(self) -> Self {
        let x = self.0;

        // If x is negative, final result should be negated
        let negate_if_negative = _mm256_and_pd(_mm256_set1_pd(-0.0), x);

        // Get absolute values (clear sign bits)
        let x = _mm256_andnot_pd(_mm256_set1_pd(-0.0), x);

        // Get fractional parts
        let x = Self(x).fract().0;

        let gt_half_pd_mask = _mm256_cmp_pd::<{ _CMP_GT_OQ }>(x, _mm256_set1_pd(0.5));

        // If value > 0.5, replace with 1.0 - value
        let x = _mm256_blendv_pd(x, _mm256_sub_pd(_mm256_set1_pd(1.0), x), gt_half_pd_mask);

        // If x > 0.5, final result should be negated
        let negate_if_gt_half = _mm256_and_pd(_mm256_set1_pd(-0.0), gt_half_pd_mask);

        // Combine negation masks
        let negation_mask = _mm256_xor_pd(negate_if_negative, negate_if_gt_half);

        let one = _mm256_set1_pd(1.0);

        // More iterations cause "tighter interpolation"
        let a = _mm256_sub_pd(_mm256_mul_pd(_mm256_set1_pd(4.0), x), one);
        let a2 = _mm256_mul_pd(a, a);
        let a4 = _mm256_mul_pd(a2, a2);
        let a8 = _mm256_mul_pd(a4, a4);
        let a16 = _mm256_mul_pd(a8, a8);
        let a32 = _mm256_mul_pd(a16, a16);
        let a64 = _mm256_mul_pd(a32, a32);
        let a128 = _mm256_mul_pd(a64, a64);

        let x = _mm256_mul_pd(
            _mm256_set1_pd(2.0),
            _mm256_sub_pd(
                _mm256_div_pd(one, _mm256_add_pd(one, a128)),
                _mm256_set1_pd(0.5),
            ),
        );
        let x = _mm256_xor_pd(x, negation_mask);

        Self(x)
    }
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn saw(self) -> Self {
        const DOWN_FACTOR: f64 = 50.0;
        const X_INTERSECTION: f64 = 1.0 - (1.0 / DOWN_FACTOR);
        const UP_FACTOR: f64 = 1.0 / X_INTERSECTION;

        let x = self.abs().fract().0;

        // If x was originally negative, replace with 1.0 - x
        let x = _mm256_blendv_pd(
            x,
            _mm256_sub_pd(_mm256_set1_pd(1.0), x),
            _mm256_cmp_pd::<{ _CMP_LT_OQ }>(self.0, _mm256_setzero_pd()),
        );

        let up = _mm256_mul_pd(x, _mm256_set1_pd(UP_FACTOR));
        let down = _mm256_sub_pd(
            _mm256_set1_pd(DOWN_FACTOR),
            _mm256_mul_pd(_mm256_set1_pd(DOWN_FACTOR), x),
        );

        let y = _mm256_blendv_pd(
            down,
            up,
            _mm256_cmp_pd::<{ _CMP_LT_OQ }>(x, _mm256_set1_pd(X_INTERSECTION)),
        );

        Self(_mm256_mul_pd(
            _mm256_set1_pd(2.0),
            _mm256_sub_pd(y, _mm256_set1_pd(0.5)),
        ))
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
