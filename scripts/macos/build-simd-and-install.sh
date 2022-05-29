#!/bin/bash

set -e

cargo +nightly build --profile "release-debug" --features "simd" -p octasine-vst2-plugin

./scripts/macos/bundle.sh "./target/release-debug/liboctasine.dylib" "OctaSine"
./scripts/macos/install.sh "./tmp/OctaSine.vst"
