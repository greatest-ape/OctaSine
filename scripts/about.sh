#!/bin/sh

mkdir -p tmp

cargo-about generate -o tmp/licenses.html about.hbs -m "octasine/Cargo.toml" 
