#!/bin/bash

set -e

cargo +nightly build --profile "release-debug" --features "simd" -p octasine_vst2_plugin

./scripts/macos/bundle.sh "./target/release-debug/liboctasine.dylib" "OctaSine"
./scripts/macos/install.sh "./tmp/OctaSine.vst"
