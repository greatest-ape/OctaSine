#!/bin/bash

set -e

./scripts/macos/bundle.sh "./target/release/liboctasine.dylib"
./scripts/macos/package.sh "OctaSine.vst" "OctaSine-macOS-Intel"