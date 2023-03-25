use std::arch::x86_64::*;
use std::ops::{Add, AddAssign, Mul, Sub};

use super::{Simd, SimdPackedDouble};

pub struct Sse2;

impl Simd for Sse2 {
    type Pd = Sse2PackedDouble;
}

#[derive(Copy, Clone, Debug)]
pub struct Sse2PackedDouble(__m128d);

impl SimdPackedDouble for Sse2PackedDouble {
    const WIDTH: usize = 2;

    type Arr = [f64; Self::WIDTH];

    #[inline(always)]
    unsafe fn new(value: f64) -> Self {
        Self(_mm_set1_pd(value))
    }
    #[inline(always)]
    unsafe fn new_zeroed() -> Self {
        Self(_mm_setzero_pd())
    }
    #[inline(always)]
    unsafe fn from_arr(arr: Self::Arr) -> Self {
        Self(_mm_loadu_pd(arr.as_ptr()))
    }
    #[inline(always)]
    unsafe fn to_arr(self) -> Self::Arr {
        let mut arr = Self::Arr::default();

        _mm_storeu_pd(arr.as_mut_ptr(), self.0);

        arr
    }
    #[inline(always)]
    unsafe fn min(self, other: Self) -> Self {
        Self(_mm_min_pd(self.0, other.0))
    }
    #[inline(always)]
    unsafe fn max(self, other: Self) -> Self {
        Self(_mm_max_pd(self.0, other.0))
    }
    #[inline(always)]
    unsafe fn interleave(self, other: Self) -> Self {
        Self(_mm_move_sd(other.0, self.0))
    }
    #[inline(always)]
    unsafe fn pairwise_horizontal_sum(self) -> Self {
        Self(_mm_add_pd(self.0, _mm_shuffle_pd(self.0, self.0, 0b01)))
    }
    #[inline(always)]
    unsafe fn new_from_pair(l: f64, r: f64) -> Self {
        Self(_mm_set_pd(r, l))
    }
    #[inline(always)]
    unsafe fn any_over_zero(self) -> bool {
        _mm_movemask_pd(_mm_cmpgt_pd(self.0, _mm_setzero_pd())) != 0
    }
    // Workaround due to lack of instructions
    #[inline(always)]
    unsafe fn floor(self) -> Self {
        let mut a = self.to_arr();

        for a in a.iter_mut() {
            *a = a.floor();
        }

        Self::from_arr(a)
    }
    #[inline(always)]
    unsafe fn abs(self) -> Self {
        Self(_mm_andnot_pd(_mm_set1_pd(-0.0), self.0))
    }
    #[inline(always)]
    unsafe fn fast_sin(self) -> Self {
        Self(sleef_trig::Sleef_sind2_u35sse2(self.0))
    }
    #[inline(always)]
    unsafe fn triangle(mut self) -> Self {
        self += Self::new(0.25);

        let two = Self::new(2.0);

        (two * (two * (self - (self + Self::new(0.5)).floor())).abs()) - Self::new(1.0)
    }
    #[inline(always)]
    unsafe fn square(self) -> Self {
        let x = self.0;

        // If x is negative, final result should be negated
        let negate_if_negative = _mm_and_pd(_mm_set1_pd(-0.0), self.0);

        // Get absolute value (clear sign bit)
        let x = _mm_andnot_pd(_mm_set1_pd(-0.0), x);

        // Workaround due to lack of fract and dynamic shuffle instructions
        let (x, negate_if_gt_half) = {
            let tmp = Self(x).to_arr();

            let mut a = tmp[0].fract();
            let mut b = tmp[1].fract();

            // If x > 0.5, final result should be negated
            let negate_a = if a > 0.5 { -0.0 } else { 0.0 };
            let negate_b = if b > 0.5 { -0.0 } else { 0.0 };
            let negate_if_gt_half = Self::from_arr([negate_a, negate_b]).0;

            // If value > 0.5, replace with 1.0 - value
            if a > 0.5 {
                a = 1.0 - a;
            }
            if b > 0.5 {
                b = 1.0 - b;
            }
            let x = Self::from_arr([a, b]).0;

            (x, negate_if_gt_half)
        };

        let negation_mask = _mm_xor_pd(negate_if_negative, negate_if_gt_half);

        let one = _mm_set1_pd(1.0);

        // More iterations cause "tighter interpolation"
        let a = _mm_sub_pd(_mm_mul_pd(_mm_set1_pd(4.0), x), one);
        let a2 = _mm_mul_pd(a, a);
        let a4 = _mm_mul_pd(a2, a2);
        let a8 = _mm_mul_pd(a4, a4);
        let a16 = _mm_mul_pd(a8, a8);
        let a32 = _mm_mul_pd(a16, a16);
        let a64 = _mm_mul_pd(a32, a32);
        let a128 = _mm_mul_pd(a64, a64);

        let x = _mm_mul_pd(
            _mm_set1_pd(2.0),
            _mm_sub_pd(_mm_div_pd(one, _mm_add_pd(one, a128)), _mm_set1_pd(0.5)),
        );
        let x = _mm_xor_pd(x, negation_mask);

        Self(x)
    }
    #[inline(always)]
    unsafe fn saw(self) -> Self {
        // Scalar workaround due to lack of blend/shuffle instructions
        Self::from_arr(
            super::FallbackPackedDouble::from_arr(self.to_arr())
                .saw()
                .to_arr(),
        )
    }
}

impl Add for Sse2PackedDouble {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        unsafe { Self(_mm_add_pd(self.0, rhs.0)) }
    }
}

impl AddAssign for Sse2PackedDouble {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Sse2PackedDouble {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { Self(_mm_sub_pd(self.0, rhs.0)) }
    }
}

impl Mul for Sse2PackedDouble {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        unsafe { Self(_mm_mul_pd(self.0, rhs.0)) }
    }
}
