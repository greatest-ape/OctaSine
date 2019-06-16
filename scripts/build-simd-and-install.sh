#!/bin/bash

set -e

NAME="FM Rust"
MOVE_TO="/Library/Audio/Plug-Ins/VST/FM_Rust/"

RUSTFLAGS="-C target-cpu=native" cargo +nightly build --release --features "simd"

# TODO: static linking
install_name_tool -add_rpath /Users/j/Projects/sleef/my-sleef-install/lib/ target/release/libfm.dylib

./scripts/osx_vst_bundler.sh "$NAME" target/release/libfm.dylib

if [ -d "$MOVE_TO" ]; then
    rm -r "$MOVE_TO"
fi

mkdir -p "$MOVE_TO"
mv "$NAME.vst" "$MOVE_TO"

echo "Copied VST bundle to $MOVE_TO"