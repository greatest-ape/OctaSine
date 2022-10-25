//! SIMD abstraction

use std::ops::{Add, AddAssign, Mul, Sub};

#[cfg(target_arch = "x86_64")]
pub mod avx;
pub mod fallback;
#[cfg(target_arch = "x86_64")]
pub mod sse2;

#[cfg(target_arch = "x86_64")]
pub use avx::*;
pub use fallback::*;
#[cfg(target_arch = "x86_64")]
pub use sse2::*;

pub trait Simd {
    type Pd: SimdPackedDouble;
}

pub trait SimdPackedDouble: Copy + Add + AddAssign + Sub + Mul {
    // Number of doubles that this packed double fits
    const WIDTH: usize;
    /// Number of stereo audio samples that this packed double fits
    const SAMPLES: usize = Self::WIDTH / 2;

    /// f64 array with same number of members as this packed double
    type Arr;

    unsafe fn new(value: f64) -> Self;
    unsafe fn new_zeroed() -> Self;
    unsafe fn new_from_pair(l: f64, r: f64) -> Self;
    unsafe fn from_arr(arr: Self::Arr) -> Self;
    unsafe fn to_arr(self) -> Self::Arr;
    unsafe fn min(self, other: Self) -> Self;
    unsafe fn max(self, other: Self) -> Self;
    unsafe fn fast_sin(self) -> Self;
    unsafe fn pairwise_horizontal_sum(self) -> Self;
    unsafe fn interleave(self, other: Self) -> Self;
    unsafe fn any_over_zero(self) -> bool;
}