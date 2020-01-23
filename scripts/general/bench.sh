#!/bin/sh

cd octasine

RUSTFLAGS="-C target-cpu=native" cargo +nightly bench --features "simd"
