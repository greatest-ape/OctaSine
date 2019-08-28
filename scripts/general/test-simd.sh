#!/bin/bash

cd octasine

RUSTFLAGS="-C target-cpu=native" cargo +nightly test --release --features "simd"

cd ..

cd vst2_helpers

RUSTFLAGS="-C target-cpu=native" cargo +nightly test --release
