#!/bin/sh

RUSTFLAGS="-C target-cpu=native" cargo +stable run --release --bin plot_curves