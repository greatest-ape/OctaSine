#!/bin/sh

cargo +nightly clippy  --workspace --all-targets --all-features
