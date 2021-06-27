#!/bin/bash

# Cross-compile for Apple silicon
# More info: https://github.com/shepmaster/rust/blob/silicon/silicon/README.md
# Building sleef fails, see https://github.com/shibatch/sleef/issues/249
export SDKROOT=$(xcrun -sdk macosx11.1 --show-sdk-path)
export MACOSX_DEPLOYMENT_TARGET=$(xcrun -sdk macosx11.1 --show-sdk-platform-version)
cargo build --release -p octasine_vst2_plugin --target=aarch64-apple-darwin
