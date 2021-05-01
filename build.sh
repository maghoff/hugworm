#!/usr/bin/env bash

set -e

# RUSTFLAGS can cause trouble.
#
# "-C target-cpu=native" causes warnings and can be overridden with "-C target-cpu=generic"
#
# "-C link-arg=-fuse-ld=lld" causes failure, but I don't know how to override it. The
# easiest way out is to just reset RUSTFLAGS
export RUSTFLAGS=

wasm-pack build

./node_modules/.bin/webpack \
    --mode production \
    --entry ./entry.js \
    --output-public-path ./dist/ \
    --output-path $PWD/site/dist
