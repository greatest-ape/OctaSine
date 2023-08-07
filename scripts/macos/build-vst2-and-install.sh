#!/bin/bash

# Validate with: https://github.com/free-audio/clap-validator

set -e

cargo xtask bundle octasine --profile "release-debug" --features "vst2 wgpu"

TARGET="/Library/Audio/Plug-Ins/VST/OctaSine.vst"

if [ -d "$TARGET" ]; then
	rm -r "$TARGET"
fi

cp -r "./target/bundled/octasine.vst" "$TARGET"
echo "Copied VST bundle to $TARGET"
