#!/bin/bash

# Validate with: https://github.com/free-audio/clap-validator

set -e

cargo build --profile "release-debug" -p octasine

./scripts/macos/bundle.sh "./target/release-debug/liboctasine.dylib" "OctaSine"

TARGET="/Library/Audio/Plug-Ins/CLAP/OctaSine.clap"

if [ -d "$TARGET" ]; then
	rm -r "$TARGET"
fi

cp -r "./tmp/OctaSine.vst" "$TARGET"
echo "Copied CLAP bundle to $TARGET"
