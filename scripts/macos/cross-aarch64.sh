#!/bin/bash

set -e

# Cross-compile for Apple silicon
# More info:
# - https://github.com/shepmaster/rust/blob/silicon/silicon/README.md
# - https://stackoverflow.com/questions/66849112/how-do-i-cross-compile-a-rust-application-from-macos-x86-to-macos-silicon
export SDKROOT=$(xcrun -sdk macosx11.1 --show-sdk-path)
export MACOSX_DEPLOYMENT_TARGET=$(xcrun -sdk macosx11.1 --show-sdk-platform-version)
cargo build --release -p octasine-vst2-plugin --target=aarch64-apple-darwin

./scripts/macos/bundle.sh "./target/aarch64-apple-darwin/release/liboctasine.dylib" "OctaSine"
./scripts/unix/package.sh "OctaSine.vst" "OctaSine-macOS-AppleSilicon"
