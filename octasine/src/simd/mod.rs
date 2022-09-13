//! SIMD abstraction

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
pub mod avx;
pub mod fallback;

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
pub use avx::*;
pub use fallback::*;

use std::ops::{Add, AddAssign, Mul, Sub, BitAnd, BitOr, BitXor};

pub trait Simd {
    type Pd: SimdPackedDouble;
}

pub trait SimdPackedDouble:
    Copy + Add<Output = Self> + AddAssign + Sub<Output = Self> + Mul<Output = Self>
 + BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self>
{
    /// Number of stereo audio samples that this packed double fits
    const SAMPLES: usize;

    /// f64 array with same number of members as this packed double
    type Arr;

    unsafe fn new(value: f64) -> Self;
    unsafe fn new_zeroed() -> Self;
    unsafe fn new_from_pair(l: f64, r: f64) -> Self;
    unsafe fn load_ptr(source: *const f64) -> Self;
    unsafe fn from_arr(arr: Self::Arr) -> Self;
    unsafe fn to_arr(&self) -> Self::Arr;
    unsafe fn min(&self, other: Self) -> Self;
    unsafe fn max(&self, other: Self) -> Self;
    unsafe fn fast_sin(self) -> Self;
    unsafe fn pairwise_horizontal_sum(&self) -> Self;
    unsafe fn interleave(&self, other: Self) -> Self;
    unsafe fn any_over_zero(&self) -> bool;
    /// For members of self with negative sign, multiply members of other with -1.0
    unsafe fn multiply_negative_sign(self, other: Self) -> Self;
}

/// Fast sine approximation valid for range (-pi, pi)
///
/// Adapted from http://mooooo.ooo/chebyshev-sine-approximation/
pub unsafe fn chebyshev_sin_x<Pd: SimdPackedDouble>(x: Pd) -> Pd {
    const COEFFICIENT_X: f64 = -0.10132118; // x
    const COEFFICIENT_X3: f64 = 0.0066208798; // x^3
    const COEFFICIENT_X5: f64 = -0.00017350505; // x^5
    const COEFFICIENT_X7: f64 = 0.0000025222919; // x^7
    const COEFFICIENT_X9: f64 = -0.000000023317787; // x^9
    const COEFFICIENT_X11: f64 = 0.00000000013291342; // x^11

    let pi_major = Pd::new(3.1415927);
    let pi_minor = Pd::new(-0.00000008742278);

    let x2 = x * x;

    let p11 = Pd::new(COEFFICIENT_X11);
    let p9 = p11 * x2 + Pd::new(COEFFICIENT_X9);
    let p7 = p9 * x2 + Pd::new(COEFFICIENT_X7);
    let p5 = p7 * x2 + Pd::new(COEFFICIENT_X5);
    let p3 = p5 * x2 + Pd::new(COEFFICIENT_X3);
    let p1 = p3 * x2 + Pd::new(COEFFICIENT_X);

    (x - pi_major - pi_minor) * (x + pi_major + pi_minor) * p1 * x
}