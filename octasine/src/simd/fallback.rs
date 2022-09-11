use super::{Simd, SimdPackedDouble};

use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Div, Mul, Sub},
};

pub type FallbackStd = Fallback<FallbackSineStd>;
#[cfg(feature = "simd")]
pub type FallbackSleef = Fallback<FallbackSineSleef>;

pub struct Fallback<T: FallbackSine>(PhantomData<T>);

impl<T: FallbackSine> Simd for Fallback<T> {
    type Pd = FallbackPackedDouble<T>;
}

#[derive(Clone, Copy)]
pub struct FallbackPackedDouble<T>([f64; 2], PhantomData<T>);

impl<T: FallbackSine> SimdPackedDouble for FallbackPackedDouble<T> {
    const SAMPLES: usize = 1;

    type Arr = [f64; 2];

    #[inline]
    unsafe fn new(value: f64) -> Self {
        Self([value, value], Default::default())
    }
    #[inline]
    unsafe fn new_zeroed() -> Self {
        Self([0.0, 0.0], Default::default())
    }
    #[inline]
    unsafe fn new_from_pair(l: f64, r: f64) -> Self {
        Self([l, r], Default::default())
    }
    #[inline]
    unsafe fn from_arr(arr: Self::Arr) -> Self {
        Self(arr, Default::default())
    }
    #[inline]
    unsafe fn load_ptr(source: *const f64) -> Self {
        Self(*(source as *const [f64; 2]), Default::default())
    }
    #[inline]
    unsafe fn to_arr(&self) -> Self::Arr {
        self.0
    }
    #[inline]
    unsafe fn min(&self, other: Self) -> Self {
        let [a1, a2] = self.0;
        let [b1, b2] = other.0;

        Self([a1.min(b1), a2.min(b2)], self.1)
    }
    #[inline]
    unsafe fn max(&self, other: Self) -> Self {
        let [a1, a2] = self.0;
        let [b1, b2] = other.0;

        Self([a1.max(b1), a2.max(b2)], self.1)
    }
    #[inline]
    unsafe fn fast_sin(&self) -> Self {
        Self(T::sin(self.0), self.1)
    }
    #[inline]
    unsafe fn pairwise_horizontal_sum(&self) -> Self {
        let [l, r] = self.0;

        Self([l + r, l + r], self.1)
    }
    #[inline]
    unsafe fn interleave(&self, other: Self) -> Self {
        Self([self.0[0], other.0[1]], self.1)
    }
    #[inline]
    unsafe fn any_over_zero(&self) -> bool {
        (self.0[0] > 0.0) | (self.0[1] > 0.0)
    }
    #[inline]
    unsafe fn log10(&self) -> Self {
        let [a, b] = self.0;

        Self([a.log10(), b.log10()], self.1)
    }
}

impl<T> Add for FallbackPackedDouble<T> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        let [a1, a2] = self.0;
        let [b1, b2] = rhs.0;

        Self([a1 + b1, a2 + b2], self.1)
    }
}

impl<T> AddAssign for FallbackPackedDouble<T>
where
    FallbackPackedDouble<T>: Copy,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T> Sub for FallbackPackedDouble<T> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        let [a1, a2] = self.0;
        let [b1, b2] = rhs.0;

        Self([a1 - b1, a2 - b2], self.1)
    }
}

impl<T> Mul for FallbackPackedDouble<T> {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        let [a1, a2] = self.0;
        let [b1, b2] = rhs.0;

        Self([a1 * b1, a2 * b2], self.1)
    }
}

impl<T> Div for FallbackPackedDouble<T> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        let [a1, a2] = self.0;
        let [b1, b2] = rhs.0;

        Self([a1 / b1, a2 / b2], self.1)
    }
}

pub trait FallbackSine: Copy {
    fn sin(a: [f64; 2]) -> [f64; 2];
}

#[derive(Clone, Copy)]
pub struct FallbackSineStd;

impl FallbackSine for FallbackSineStd {
    fn sin([a1, a2]: [f64; 2]) -> [f64; 2] {
        [a1.sin(), a2.sin()]
    }
}

#[cfg(feature = "simd")]
#[derive(Clone, Copy)]
pub struct FallbackSineSleef;

#[cfg(feature = "simd")]
impl FallbackSine for FallbackSineSleef {
    fn sin([a1, a2]: [f64; 2]) -> [f64; 2] {
        unsafe {
            [
                sleef_sys::Sleef_cinz_sind1_u35purec(a1),
                sleef_sys::Sleef_cinz_sind1_u35purec(a2),
            ]
        }
    }
}
