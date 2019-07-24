#!/bin/sh

cd octasine_vst

RUSTFLAGS="-C target-cpu=native" cargo +nightly asm "octasine::OctaSine::gen_samples_for_voices" --rust --features "simd logging"