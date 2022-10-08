#!/bin/bash

set -e

if [[ -z $1 || -z $2 ]]; then
    echo "Usage: $0 [bundle path] [target path]"
    echo "Example: $0 OctaSine.vst OctaSine-macOS-Intel"
    echo "Assumes bundle is in ./tmp"
else
    FROM="$1"
    TO="$2.zip"

    mkdir -p ./tmp
    cd tmp

    if [ -d "OctaSine" ]; then
        rm -r "OctaSine"
    fi

    mkdir "OctaSine"

    cp -r "$FROM" "OctaSine/"

    cargo-about generate -o "OctaSine/LICENSES.html" "../about.hbs" -m "../octasine-vst2-plugin/Cargo.toml" 
    # cp ../LICENSE "OctaSine/LICENSE.txt"

    if [ -f "$TO" ]; then
        rm "$TO"
    fi

    zip -9 -r "$TO" "OctaSine"

    rm -r "OctaSine"

    echo "Created $TO"
fi
