#!/bin/sh

RUSTFLAGS="-C target-cpu=native" cargo +stable run --release plot_curves