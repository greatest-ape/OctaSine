#[cfg(all(feature = "simd", target_arch = "x86_64"))]
use core::arch::x86_64::*;

use crate::constants::ZERO_VALUE_LIMIT;

pub trait FallbackSine {
    fn sin(a: [f64; 2]) -> [f64; 2];
}

pub struct FallbackSineStd;

impl FallbackSine for FallbackSineStd {
    fn sin([a1, a2]: [f64; 2]) -> [f64; 2] {
        [a1.sin(), a2.sin()]
    }
}

#[cfg(feature = "simd")]
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

pub trait Simd {
    type PackedDouble;
    const PD_WIDTH: usize;
    const SAMPLES: usize;

    unsafe fn pd_set1(value: f64) -> Self::PackedDouble;
    unsafe fn pd_setzero() -> Self::PackedDouble;
    unsafe fn pd_loadu(source: *const f64) -> Self::PackedDouble;
    unsafe fn pd_storeu(target: *mut f64, a: Self::PackedDouble);
    unsafe fn pd_add(a: Self::PackedDouble, b: Self::PackedDouble) -> Self::PackedDouble;
    unsafe fn pd_sub(a: Self::PackedDouble, b: Self::PackedDouble) -> Self::PackedDouble;
    unsafe fn pd_mul(a: Self::PackedDouble, b: Self::PackedDouble) -> Self::PackedDouble;
    unsafe fn pd_min(a: Self::PackedDouble, b: Self::PackedDouble) -> Self::PackedDouble;
    unsafe fn pd_max(a: Self::PackedDouble, b: Self::PackedDouble) -> Self::PackedDouble;
    unsafe fn pd_fast_sin(a: Self::PackedDouble) -> Self::PackedDouble;
    unsafe fn pd_pairwise_horizontal_sum(a: Self::PackedDouble) -> Self::PackedDouble;
    unsafe fn pd_distribute_left_right(l: f64, r: f64) -> Self::PackedDouble;
    unsafe fn pd_over_zero_limit(volume: Self::PackedDouble) -> bool;
}

pub struct Fallback<T> {
    phantom_data: ::std::marker::PhantomData<T>,
}

impl<T: FallbackSine> Simd for Fallback<T> {
    type PackedDouble = [f64; 2];
    const PD_WIDTH: usize = 2;
    const SAMPLES: usize = 1;

    unsafe fn pd_set1(value: f64) -> [f64; 2] {
        [value, value]
    }
    unsafe fn pd_setzero() -> [f64; 2] {
        [0.0, 0.0]
    }
    unsafe fn pd_loadu(source: *const f64) -> [f64; 2] {
        *(source as *const [f64; 2])
    }
    unsafe fn pd_storeu(target: *mut f64, a: [f64; 2]) {
        ::std::ptr::write(target as *mut [f64; 2], a);
    }
    unsafe fn pd_add([a1, a2]: [f64; 2], [b1, b2]: [f64; 2]) -> [f64; 2] {
        [a1 + b1, a2 + b2]
    }
    unsafe fn pd_sub([a1, a2]: [f64; 2], [b1, b2]: [f64; 2]) -> [f64; 2] {
        [a1 - b1, a2 - b2]
    }
    unsafe fn pd_mul([a1, a2]: [f64; 2], [b1, b2]: [f64; 2]) -> [f64; 2] {
        [a1 * b1, a2 * b2]
    }
    unsafe fn pd_min([a1, a2]: [f64; 2], [b1, b2]: [f64; 2]) -> [f64; 2] {
        [a1.min(b1), a2.min(b2)]
    }
    unsafe fn pd_max([a1, a2]: [f64; 2], [b1, b2]: [f64; 2]) -> [f64; 2] {
        [a1.max(b1), a2.max(b2)]
    }
    unsafe fn pd_fast_sin(a: [f64; 2]) -> [f64; 2] {
        T::sin(a)
    }
    unsafe fn pd_pairwise_horizontal_sum([l, r]: [f64; 2]) -> [f64; 2] {
        [l + r, l + r]
    }
    unsafe fn pd_distribute_left_right(l: f64, r: f64) -> [f64; 2] {
        [l, r]
    }
    unsafe fn pd_over_zero_limit([a1, a2]: [f64; 2]) -> bool {
        (a1 > ZERO_VALUE_LIMIT) & (a2 > ZERO_VALUE_LIMIT)
    }
}

pub struct Sse2;

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
impl Simd for Sse2 {
    type PackedDouble = __m128d;
    const PD_WIDTH: usize = 2;
    const SAMPLES: usize = 1;

    #[target_feature(enable = "sse2")]
    unsafe fn pd_set1(value: f64) -> __m128d {
        _mm_set1_pd(value)
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_setzero() -> __m128d {
        _mm_setzero_pd()
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_loadu(source: *const f64) -> __m128d {
        _mm_loadu_pd(source)
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_storeu(target: *mut f64, a: __m128d) {
        _mm_storeu_pd(target, a)
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_add(a: __m128d, b: __m128d) -> __m128d {
        _mm_add_pd(a, b)
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_sub(a: __m128d, b: __m128d) -> __m128d {
        _mm_sub_pd(a, b)
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_mul(a: __m128d, b: __m128d) -> __m128d {
        _mm_mul_pd(a, b)
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_min(a: __m128d, b: __m128d) -> __m128d {
        _mm_min_pd(a, b)
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_max(a: __m128d, b: __m128d) -> __m128d {
        _mm_max_pd(a, b)
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_fast_sin(a: __m128d) -> __m128d {
        sleef_sys::Sleef_cinz_sind2_u35sse2(a)
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_pairwise_horizontal_sum(a: __m128d) -> __m128d {
        _mm_add_pd(a, _mm_shuffle_pd(a, a, 0b01))
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_distribute_left_right(l: f64, r: f64) -> __m128d {
        let lr = [l, r];

        _mm_loadu_pd(&lr[0])
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_over_zero_limit(a: __m128d) -> bool {
        _mm_movemask_pd(_mm_cmpgt_pd(a, _mm_set1_pd(ZERO_VALUE_LIMIT))) == 0b11
    }
}

pub struct Avx;

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
impl Simd for Avx {
    type PackedDouble = __m256d;
    const PD_WIDTH: usize = 4;
    const SAMPLES: usize = 2;

    #[target_feature(enable = "avx")]
    unsafe fn pd_set1(value: f64) -> __m256d {
        _mm256_set1_pd(value)
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_setzero() -> __m256d {
        _mm256_setzero_pd()
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_loadu(source: *const f64) -> __m256d {
        _mm256_loadu_pd(source)
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_storeu(target: *mut f64, a: __m256d) {
        _mm256_storeu_pd(target, a)
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_add(a: __m256d, b: __m256d) -> __m256d {
        _mm256_add_pd(a, b)
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_sub(a: __m256d, b: __m256d) -> __m256d {
        _mm256_sub_pd(a, b)
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_mul(a: __m256d, b: __m256d) -> __m256d {
        _mm256_mul_pd(a, b)
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_min(a: __m256d, b: __m256d) -> __m256d {
        _mm256_min_pd(a, b)
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_max(a: __m256d, b: __m256d) -> __m256d {
        _mm256_max_pd(a, b)
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_fast_sin(a: __m256d) -> __m256d {
        sleef_sys::Sleef_cinz_sind4_u35avx(a)
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_pairwise_horizontal_sum(a: __m256d) -> __m256d {
        _mm256_add_pd(a, _mm256_permute_pd(a, 0b0101))
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_distribute_left_right(l: f64, r: f64) -> __m256d {
        let lr = [l, r, l, r];

        _mm256_loadu_pd(&lr[0])
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_over_zero_limit(a: __m256d) -> bool {
        _mm256_movemask_pd(
            _mm256_cmp_pd::<{_CMP_GT_OQ}>(a, _mm256_set1_pd(ZERO_VALUE_LIMIT))
        ) == 0b1111
    }
}

pub type FallbackStd = Fallback<FallbackSineStd>;
#[cfg(feature = "simd")]
pub type FallbackSleef = Fallback<FallbackSineSleef>;
