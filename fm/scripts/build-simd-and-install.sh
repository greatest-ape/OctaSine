#!/bin/bash

set -e

# Settings

NAME="FM Rust"

# Script

VST_NAME="$NAME.vst"
MOVE_TO="/Library/Audio/Plug-Ins/VST/$VST_NAME"
TMP_DIR="tmp"

RUSTFLAGS="-C target-cpu=native" cargo +nightly build --release --features "simd"

# TODO: static linking
install_name_tool -add_rpath /Users/j/Projects/sleef/my-sleef-install/lib/ ../target/release/libfm.dylib

./scripts/osx_vst_bundler.sh "$NAME" ../target/release/libfm.dylib

if [ -d "$MOVE_TO" ]; then
    rm -r "$MOVE_TO"
fi

mv "$TMP_DIR/$VST_NAME" "$MOVE_TO"

echo "Copied VST bundle to $MOVE_TO"