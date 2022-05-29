#!/bin/sh

if [[ -z $1 ]]; then
    echo "Usage: $0 [glow|wgpu]"
else
    cargo run -p octasine-cli --no-default-features --features $1 -- run-gui
fi
