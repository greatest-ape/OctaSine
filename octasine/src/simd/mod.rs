//! SIMD abstraction

#[cfg(target_arch = "x86_64")]
pub mod avx;
pub mod fallback;

#[cfg(target_arch = "x86_64")]
pub use avx::*;
pub use fallback::*;

pub trait Simd {
    type Pd: SimdPackedDouble;
}

pub trait SimdPackedDouble: Copy {
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
    unsafe fn fast_sin(&self) -> Self;
    unsafe fn pairwise_horizontal_sum(&self) -> Self;
    unsafe fn interleave(&self, other: Self) -> Self;
    unsafe fn any_over_zero(&self) -> bool;
}
