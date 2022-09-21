#!/bin/sh

cargo-about generate -o licenses.html about.hbs --features "gui_glow" -m "octasine-vst2-plugin/Cargo.toml" 
