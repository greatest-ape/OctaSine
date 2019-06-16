//! Less precise sines using sleef. Code is largely from packed_simd crate.

#![allow(dead_code)]

use std::mem::{transmute, uninitialized};

use packed_simd::*;
use sleef_sys::*;


#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.sin.v4f32"]
    fn sin_v4f32(x: f32x4) -> f32x4;

    #[link_name = "llvm.sin.f32"]
    fn sin_f32(x: f32) -> f32;
}


/// SIMD sine with 3.5 ULP error bound using the sleef library.
/// 
/// Resorts to standard LLVM sine when SSE4.2 is not available.
pub trait SleefSin35 {
    fn sin(self) -> Self;
}


impl SleefSin35 for f32x4 {
    #[inline]
    fn sin(self) -> Self {
        unsafe {
            #[cfg(target_feature = "sse4.2")]
            return transmute(Sleef_sinf4_u35sse4(transmute(self)));
            #[cfg(not(target_feature = "sse4.2"))]
            return transmute(sin_v4f32(transmute(self)));
        }
    }
}


impl SleefSin35 for f32x2 {
    #[inline]
    fn sin(self) -> Self {
        unsafe {
            #[cfg(target_feature = "sse4.2")]
            return {
                union U {
                    vec: [f32x2; 2],
                    twice: f32x4,
                }

                let twice = U { vec: [self, uninitialized()] }.twice;
                let twice = transmute(Sleef_sinf4_u35sse4(transmute(twice)));

                *(U { twice }.vec.get_unchecked(0))
            };
            #[cfg(not(target_feature = "sse4.2"))]
            return {
                union U {
                    vec: f32x2,
                    scalars: [f32; 2],
                }
                let mut scalars = U { vec: self }.scalars;
                for i in &mut scalars {
                    *i = sin_f32(*i);
                }
                U { scalars }.vec
            };
        }
    }
}


#[cfg(test)]
mod tests {
    use quickcheck::{TestResult, quickcheck};

    use super::*;

    #[test]
    fn test_sleef_sin_40(){
        fn prop(value: f32) -> TestResult {
            let x2 = f32x2::splat(value);
            let x4 = f32x4::splat(value);

            let test_sin_x2 = SleefSin35::sin(x2).extract(0);
            let test_sin_x4 = SleefSin35::sin(x4).extract(0);

            assert_eq!(test_sin_x2, test_sin_x4);

            let reference_sin = value.sin();
            let diff = (test_sin_x2 - reference_sin).abs();

            let success = diff < 0.000001;

            if !success {
                println!();
                println!("input value:      {}", value);
                println!("test sin():       {}", test_sin_x2);
                println!("reference sin():  {}", reference_sin);
                println!("difference:       {}", diff);
            }

            TestResult::from_bool(success)
        }

        quickcheck(prop as fn(f32) -> TestResult);
    }
}