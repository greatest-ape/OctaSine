#!/bin/bash

# Validate with: https://github.com/free-audio/clap-validator

set -e

cargo xtask bundle -p octasine --profile "release-debug" --features "clap"

TARGET="/Library/Audio/Plug-Ins/CLAP/OctaSine.clap"

if [ -d "$TARGET" ]; then
	rm -r "$TARGET"
fi

cp -r "./target/bundled/octasine.clap" "$TARGET"
echo "Copied CLAP bundle to $TARGET"
