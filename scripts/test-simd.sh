#!/bin/bash

RUSTFLAGS="-C target-cpu=native" cargo +nightly test --release --features "simd"
# cargo +nightly test --features "simd"