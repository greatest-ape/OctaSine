#!/bin/sh

cargo +nightly run --release -p bench_process --features "simd"
