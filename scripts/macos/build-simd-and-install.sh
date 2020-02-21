#!/bin/bash

set -e

# Settings

NAME="OctaSine"

# Script

VST_NAME="$NAME.vst"
MOVE_TO="/Library/Audio/Plug-Ins/VST/$VST_NAME"
TMP_DIR="tmp"

cd octasine_vst

# RUSTFLAGS="-C target-cpu=native" 
cargo +nightly build --release --features "simd2 logging"

../contrib/osx_vst_bundler.sh "$NAME" ../target/release/liboctasine.dylib

if [ -d "$MOVE_TO" ]; then
    rm -r "$MOVE_TO"
fi

mv "$TMP_DIR/$VST_NAME" "$MOVE_TO"

echo "Copied VST bundle to $MOVE_TO"
