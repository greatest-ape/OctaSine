#!/bin/bash

# Validate with: https://github.com/free-audio/clap-validator

set -e

cargo xtask bundle -p octasine --profile "release-debug" --features "vst2"

TARGET="/Library/Audio/Plug-Ins/VST/OctaSine.vst"

if [ -d "$TARGET" ]; then
	rm -r "$TARGET"
fi

cp -r "./target/bundled/octasine.vst" "$TARGET"
echo "Copied VST bundle to $TARGET"
