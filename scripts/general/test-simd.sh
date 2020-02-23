#!/bin/sh

cd octasine

cargo +nightly test --release --features "simd"

cd ..

cd vst2_helpers

cargo +nightly test --release
