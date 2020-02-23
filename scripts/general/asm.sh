#!/bin/sh

cd octasine_vst

cargo +nightly asm "octasine::gen::simd::process_f32_runtime_select" --rust --features "simd logging"