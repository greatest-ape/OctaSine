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
    type Arr = [f64; 2];
    const SAMPLES: usize = 1;

    #[inline(always)]
    unsafe fn new(value: f64) -> Self {
        Self(_mm_set1_pd(value))
    }
    #[inline(always)]
    unsafe fn new_zeroed() -> Self {
        Self(_mm_setzero_pd())
    }
    #[inline(always)]
    unsafe fn load_ptr(source: *const f64) -> Self {
        Self(_mm_loadu_pd(source))
    }
    #[inline(always)]
    unsafe fn from_arr(arr: Self::Arr) -> Self {
        Self(_mm_loadu_pd(arr.as_ptr()))
    }
    #[inline(always)]
    unsafe fn to_arr(&self) -> Self::Arr {
        let mut arr = Self::Arr::default();

        _mm_storeu_pd(arr.as_mut_ptr(), self.0);

        arr
    }
    #[inline(always)]
    unsafe fn min(&self, other: Self) -> Self {
        Self(_mm_min_pd(self.0, other.0))
    }
    #[inline(always)]
    unsafe fn max(&self, other: Self) -> Self {
        Self(_mm_max_pd(self.0, other.0))
    }
    #[inline(always)]
    unsafe fn fast_sin(&self) -> Self {
        Self(sleef_trig::Sleef_sind2_u35sse2(self.0))
    }
    #[inline(always)]
    unsafe fn interleave(&self, other: Self) -> Self {
        Self(_mm_move_sd(other.0, self.0))
    }
    #[inline(always)]
    unsafe fn pairwise_horizontal_sum(&self) -> Self {
        Self(_mm_add_pd(self.0, _mm_shuffle_pd(self.0, self.0, 0b01)))
    }
    #[inline(always)]
    unsafe fn new_from_pair(l: f64, r: f64) -> Self {
        let lr = [l, r];

        Self(_mm_loadu_pd(lr.as_ptr()))
    }
    #[inline(always)]
    unsafe fn any_over_zero(&self) -> bool {
        _mm_movemask_pd(_mm_cmpgt_pd(self.0, _mm_setzero_pd())) != 0
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
