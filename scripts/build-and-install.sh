#!/bin/bash

set -e

NAME="FM Rust"
MOVE_TO="/Library/Audio/Plug-Ins/VST/FM_Rust/"

cargo build --release
./scripts/osx_vst_bundler.sh "$NAME" target/release/libfm.dylib
rm -r "$MOVE_TO"
mkdir -p "$MOVE_TO"
mv "$NAME.vst" "$MOVE_TO"
