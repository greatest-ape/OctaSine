#!/bin/sh

cd octasine

cargo +nightly test --release --features "simd"