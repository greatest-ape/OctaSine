#!/bin/bash

set -e

RUSTFLAGS="-C target-cpu=native" cargo +stable build --profile "release-debug" -p octasine-vst2-plugin

./scripts/macos/bundle.sh "./target/release/liboctasine.dylib" "OctaSine"
./scripts/macos/install.sh "./tmp/OctaSine.vst"