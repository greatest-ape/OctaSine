#!/bin/sh

cargo +nightly run --profile "release-debug" -p octasine_cli --no-default-features --features "bench" -- bench-process