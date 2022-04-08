#!/bin/sh

cargo +nightly run --profile "release-debug" -p bench_process --features "simd"