pub mod fallback;

#[cfg(all(target_feature = "sse2", feature = "simd"))]
pub mod simd;