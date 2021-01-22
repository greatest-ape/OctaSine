#!/bin/sh

cd octasine

cargo +nightly bench --features "simd" --bench process
