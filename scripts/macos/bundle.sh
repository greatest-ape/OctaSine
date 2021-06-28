#!/bin/bash

set -e

if [[ -z $1 ]]; then
    echo "Usage: $0 [dylib path]"
    echo "Example: $0 ./target/release/liboctasine.dylib"
else
    ./contrib/osx_vst_bundler.sh "OctaSine" $1
fi
