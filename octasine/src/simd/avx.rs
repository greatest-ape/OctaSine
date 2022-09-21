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
    #[target_feature(enable = "avx")]
    #[inline]
    unsafe fn fast_sin(&self) -> Self {
        Self(sleef_trig::Sleef_sind4_u35avx(self.0))
    }
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
