#!/bin/sh

cargo +nightly run --profile "release-debug" -p octasine-cli --no-default-features -- bench-process