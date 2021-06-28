#!/bin/bash

set -e

RUSTFLAGS="-C target-cpu=native" cargo +stable build --release -p octasine_vst2_plugin

./scripts/macos/bundle.sh "./target/release/liboctasine.dylib"
./scripts/macos/install.sh "./tmp/OctaSine.vst"