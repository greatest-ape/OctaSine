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
    const WIDTH: usize = 2;

    type Arr = [f64; Self::WIDTH];

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
    #[inline(always)]
    unsafe fn floor(self) -> Self {
        Self(apply_to_arrays!(f64::floor, self.0))
    }
    #[inline(always)]
    unsafe fn abs(self) -> Self {
        Self(apply_to_arrays!(f64::abs, self.0))
    }
    #[inline(always)]
    unsafe fn fast_sin(self) -> Self {
        Self(apply_to_arrays!(sleef_trig::Sleef_sind1_u35purec, self.0))
    }
    #[inline(always)]
    unsafe fn triangle(self) -> Self {
        Self(apply_to_arrays!(triangle, self.0))
    }
    #[inline(always)]
    unsafe fn square(self) -> Self {
        Self(apply_to_arrays!(square, self.0))
    }
    #[inline(always)]
    unsafe fn saw(self) -> Self {
        Self(apply_to_arrays!(saw, self.0))
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

/// Triangle wave
#[inline]
pub fn triangle(x: f64) -> f64 {
    let x = x + 0.25;

    (2.0 * (2.0 * (x - (x + 0.5).floor())).abs()) - 1.0
}

/// Square wave with smooth transitions
///
/// Check absence of branches with:
/// cargo asm --lib --no-default-features --full-name --rust -p octasine "octasine::simd::fallback::square"
#[inline]
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

    // More iterations cause "tighter interpolation"
    //
    // Do repeated multiplications instead of using powf to be consistent with
    // SIMD implementations.
    let a = x * 4.0 - 1.0;
    let a2 = a * a;
    let a4 = a2 * a2;
    let a8 = a4 * a4;
    let a16 = a8 * a8;
    let a32 = a16 * a16;
    let a64 = a32 * a32;
    let a128 = a64 * a64;

    let approximation = 2.0 * ((1.0 / (1.0 + a128)) - 0.5);

    f64::from_bits(approximation.to_bits() ^ sign_mask)
}

#[inline]
pub fn saw(x: f64) -> f64 {
    const DOWN_FACTOR: f64 = 50.0;
    const X_INTERSECTION: f64 = 1.0 - (1.0 / DOWN_FACTOR);
    const UP_FACTOR: f64 = 1.0 / X_INTERSECTION;

    let x_is_negative = x.is_sign_negative();

    let mut x = x.abs().fract();

    if x_is_negative {
        x = 1.0 - x;
    }

    let up = x * UP_FACTOR;
    let down = DOWN_FACTOR - DOWN_FACTOR * x;

    let y = if x < X_INTERSECTION { up } else { down };

    (y - 0.5) * 2.0
}
