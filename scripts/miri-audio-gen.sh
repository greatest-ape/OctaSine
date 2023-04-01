#!/bin/sh

# Currently doesn't build
cargo +nightly miri run --profile "release-debug" -p octasine-cli --no-default-features -- bench-process