//! Less precise SIMD sines using sleef
//! 
//! Code is heavily inspired by packed_simd crate.

#![cfg(feature = "simd")]
#![feature(link_llvm_intrinsics)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::mem::{transmute, uninitialized};

#[link(name = "sleef", kind = "static")]
extern crate sleef_sys;

use packed_simd::*;
use sleef_sys::*;
use cfg_if::cfg_if;


#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.sin.v4f64"]
    fn sin_v4f64(x: f64x4) -> f64x4;

    #[link_name = "llvm.sin.v2f64"]
    fn sin_v2f64(x: f64x2) -> f64x2;
}


/// SIMD sine with 3.5 ULP error bound using the sleef library.
/// 
/// Resorts to standard LLVM sine when SIMD is not available
pub trait SleefSin35 {
    fn sin(self) -> Self;
}


impl SleefSin35 for f64x4 {
    #[inline]
    fn sin(self) -> Self {
        cfg_if! {
            if #[cfg(target_feature = "avx2")] {
                #[inline]
                unsafe fn f(a: f64x4) -> f64x4 {
                    transmute(Sleef_sind4_u35avx2(transmute(a)))
                }
            } else if #[cfg(target_feature = "avx")] {
                #[inline]
                unsafe fn f(a: f64x4) -> f64x4 {
                    transmute(Sleef_sind4_u35avx(transmute(a)))
                }
            } else if #[cfg(target_feature = "sse4.2")] {
                #[inline]
                unsafe fn f(a: f64x4) -> f64x4 {
                    union U {
                        vec: f64x4,
                        halves: [f64x2; 2],
                    }

                    let mut halves = U { vec: a }.halves;

                    *halves.get_unchecked_mut(0) =
                        transmute(Sleef_sind2_u35sse4(transmute(*halves.get_unchecked(0))));
                    *halves.get_unchecked_mut(1) =
                        transmute(Sleef_sind2_u35sse4(transmute(*halves.get_unchecked(1))));

                    U { halves }.vec
                }
            } else {
                #[inline]
                unsafe fn f(a: f64x4) -> f64x4 {
                    return transmute(sin_v4f64(transmute(a)));
                }
            }
        }
        unsafe {
            f(self)
        }
    }
}


impl SleefSin35 for f64x2 {
    #[inline]
    fn sin(self) -> Self {
        cfg_if! {
            if #[cfg(target_feature = "avx2")] {
                #[inline]
                unsafe fn f(a: f64x2) -> f64x2 {
                    transmute(Sleef_sind2_u35avx2128(transmute(a)))
                }
            } else if #[cfg(target_feature = "avx")] {
                #[inline]
                unsafe fn f(a: f64x2) -> f64x2 {
                    transmute(Sleef_sind2_u35sse4(transmute(a)))
                }
            } else if #[cfg(target_feature = "sse4.2")] {
                #[inline]
                unsafe fn f(a: f64x2) -> f64x2 {
                    transmute(Sleef_sind2_u35sse4(transmute(a)))
                }
            } else {
                #[inline]
                unsafe fn f(a: f64x2) -> f64x2 {
                    transmute(sin_v2f64(transmute(a)))
                }
            }
        }
        unsafe {
            f(self)
        }
    }
}


#[cfg(test)]
mod tests {
    use quickcheck::{TestResult, quickcheck};

    use super::*;

    #[test]
    fn test_precision(){
        fn prop(value: f64) -> TestResult {
            let x2 = f64x2::splat(value);
            let x4 = f64x4::splat(value);

            let test_sin_x2 = SleefSin35::sin(x2).extract(0);
            let test_sin_x4 = SleefSin35::sin(x4).extract(0);

            assert_eq!(test_sin_x2, test_sin_x4);

            let reference_sin = value.sin();
            let diff = (test_sin_x2 - reference_sin).abs();

            let success = diff < 0.000000000000001;

            if !success {
                println!();
                println!("input value:      {}", value);
                println!("test sin():       {}", test_sin_x2);
                println!("reference sin():  {}", reference_sin);
                println!("difference:       {}", diff);
            }

            TestResult::from_bool(success)
        }

        quickcheck(prop as fn(f64) -> TestResult);
    }
}