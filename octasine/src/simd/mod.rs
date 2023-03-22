//! SIMD abstraction

use std::ops::{Add, AddAssign, Index, Mul, Sub};

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
    type Arr: Index<usize, Output = f64>;

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
    unsafe fn triangle(self) -> Self;
    unsafe fn floor(self) -> Self;
    unsafe fn abs(self) -> Self;
}

#[cfg(test)]
mod tests {
    #[cfg(target_arch = "x86_64")]
    #[test]
    fn test_triangle() {
        use quickcheck::{quickcheck, TestResult};

        use crate::simd::SimdPackedDouble;

        fn prop(x: f64) -> TestResult {
            if x.is_infinite() || x.is_nan() {
                return TestResult::discard();
            }

            let fallback = unsafe { super::FallbackPackedDouble::new(x).triangle().to_arr() };
            let sse2 = unsafe { super::Sse2PackedDouble::new(x).triangle().to_arr() };
            #[cfg(target_feature = "avx")]
            let avx = unsafe { super::AvxPackedDouble::new(x).triangle().to_arr() };

            let all = {
                let mut all = fallback.to_vec();

                all.extend_from_slice(&sse2[..]);
                #[cfg(target_feature = "avx")]
                all.extend_from_slice(&avx[..]);

                all
            };

            let first = *all.get(0).unwrap();

            for y in all.into_iter() {
                if y != first {
                    dbg!(x, fallback, sse2);
                    #[cfg(target_feature = "avx")]
                    dbg!(avx);

                    return TestResult::failed();
                }
            }

            TestResult::passed()
        }

        quickcheck(prop as fn(f64) -> TestResult);
    }
}
