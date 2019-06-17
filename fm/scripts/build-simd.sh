#!/bin/bash

set -e

RUSTFLAGS="-C target-cpu=native" cargo +nightly build --release --features "simd"