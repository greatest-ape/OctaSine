#!/bin/sh

cargo +nightly clippy --workspace --all-targets --features "vst2 clap"
