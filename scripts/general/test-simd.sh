#!/bin/bash

cd octasine

RUSTFLAGS="-C target-cpu=native" cargo +nightly test --release --features "simd"
