#!/bin/bash

set -e

if [[ -z $1 ]]; then
    echo "Usage: $0 [dylib path] [bundle name]"
    echo "Example: $0 ./target/release/liboctasine.dylib OctaSine"
else
    ./contrib/osx_vst_bundler.sh "$2" $1
fi
