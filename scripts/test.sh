#!/bin/sh

RUSTFLAGS="-C target-cpu=native" cargo test --features "vst2 clap"