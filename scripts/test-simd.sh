#!/bin/sh

RUSTFLAGS="-C target-cpu=native" cargo +nightly test --features "simd"