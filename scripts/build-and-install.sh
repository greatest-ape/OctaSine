#!/bin/bash

set -e

NAME="FM Rust"
MOVE_TO="/Library/Audio/Plug-Ins/VST/FM_Rust/"

cargo build --release
./scripts/osx_vst_bundler.sh "$NAME" target/release/libfm.dylib

if [ -d "$MOVE_TO" ]; then
    rm -r "$MOVE_TO"
fi

mkdir -p "$MOVE_TO"
mv "$NAME.vst" "$MOVE_TO"

echo "Copied VST bundle to $MOVE_TO"