#!/bin/sh

cargo +nightly run --profile "release-debug" -p octasine-cli --no-default-features --features "bench" -- bench-process