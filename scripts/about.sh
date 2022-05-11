#!/bin/sh

cargo-about generate -o licenses.html about.hbs --features "gui_glow simd" -m "octasine_vst2_plugin/Cargo.toml" 
