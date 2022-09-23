use super::{Simd, SimdPackedDouble};

use std::ops::{Add, AddAssign, Mul, Sub};

macro_rules! apply_to_arrays {
    ($f:expr, $a:expr) => {{
        let [a1, a2] = $a;

        [$f(a1), $f(a2)]
    }};
    ($f:expr, $a:expr, $b:expr) => {{
        let [a1, a2] = $a;
        let [b1, b2] = $b;

        [$f(a1, b1), $f(a2, b2)]
    }};
}

pub struct Fallback;

impl Simd for Fallback {
    type Pd = FallbackPackedDouble;
}

#[derive(Clone, Copy)]
pub struct FallbackPackedDouble([f64; 2]);

impl SimdPackedDouble for FallbackPackedDouble {
    const SAMPLES: usize = 1;

    type Arr = [f64; 2];

    #[inline(always)]
    unsafe fn new(value: f64) -> Self {
        Self([value, value])
    }
    #[inline(always)]
    unsafe fn new_zeroed() -> Self {
        Self([0.0, 0.0])
    }
    #[inline(always)]
    unsafe fn new_from_pair(l: f64, r: f64) -> Self {
        Self([l, r])
    }
    #[inline(always)]
    unsafe fn from_arr(arr: Self::Arr) -> Self {
        Self(arr)
    }
    #[inline(always)]
    unsafe fn load_ptr(source: *const f64) -> Self {
        Self(*(source as *const [f64; 2]))
    }
    #[inline(always)]
    unsafe fn to_arr(self) -> Self::Arr {
        self.0
    }
    #[inline(always)]
    unsafe fn min(self, other: Self) -> Self {
        Self(apply_to_arrays!(f64::min, self.0, other.0))
    }
    #[inline(always)]
    unsafe fn max(self, other: Self) -> Self {
        Self(apply_to_arrays!(f64::max, self.0, other.0))
    }
    #[inline(always)]
    unsafe fn fast_sin(self) -> Self {
        Self(apply_to_arrays!(sleef_trig::Sleef_sind1_u35purec, self.0))
    }
    #[inline(always)]
    unsafe fn pairwise_horizontal_sum(self) -> Self {
        let [l, r] = self.0;

        Self([l + r, l + r])
    }
    #[inline(always)]
    unsafe fn interleave(self, other: Self) -> Self {
        Self([self.0[0], other.0[1]])
    }
    #[inline(always)]
    unsafe fn any_over_zero(self) -> bool {
        (self.0[0] > 0.0) | (self.0[1] > 0.0)
    }
}

impl Add for FallbackPackedDouble {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self(apply_to_arrays!(Add::add, self.0, rhs.0))
    }
}

impl AddAssign for FallbackPackedDouble
where
    FallbackPackedDouble: Copy,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for FallbackPackedDouble {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(apply_to_arrays!(Sub::sub, self.0, rhs.0))
    }
}

impl Mul for FallbackPackedDouble {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(apply_to_arrays!(Mul::mul, self.0, rhs.0))
    }
}
