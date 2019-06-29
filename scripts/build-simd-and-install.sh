#!/bin/bash

set -e

# Settings

NAME="OctaSine"

# Script

VST_NAME="$NAME.vst"
MOVE_TO="/Library/Audio/Plug-Ins/VST/$VST_NAME"
SLEEF_DIR="$HOME/Projects/sleef/my-sleef-install/lib/"
TMP_DIR="tmp"

cd octasine_vst

RUSTFLAGS="-C target-cpu=native" cargo +nightly build --release --features "simd logging"

# TODO: static linking
install_name_tool -add_rpath "$SLEEF_DIR" ../target/release/liboctasine.dylib

../scripts/osx_vst_bundler.sh "$NAME" ../target/release/liboctasine.dylib

if [ -d "$MOVE_TO" ]; then
    rm -r "$MOVE_TO"
fi

mv "$TMP_DIR/$VST_NAME" "$MOVE_TO"

echo "Copied VST bundle to $MOVE_TO"
echo "Currently requires sleef library in $SLEEF_DIR to run!"