#!/bin/bash

set -e

cargo +nightly build --release --features "simd" -p octasine_vst2_plugin

./scripts/macos/bundle.sh "./target/release/liboctasine.dylib"
./scripts/macos/install.sh "./tmp/OctaSine.vst"
