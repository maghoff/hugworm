#!/usr/bin/env bash

set -e

# Silence warning caused by target-cpu=native
export RUSTFLAGS="$RUSTFLAGS -C target-cpu=generic"

wasm-pack build

./node_modules/.bin/webpack \
    --mode production \
    --entry ./site/script.js \
    --output-public-path /dist/
