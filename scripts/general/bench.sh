#!/bin/sh

cd octasine

cargo +nightly bench --features "simd2" --bench process
