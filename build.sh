#!/usr/bin/env bash

set -e

# Silence warning caused by target-cpu=native
export RUSTFLAGS="$RUSTFLAGS -C target-cpu=generic"

wasm-pack build

./node_modules/.bin/webpack \
    --mode production \
    --entry ./entry.js \
    --output-public-path /dist/ \
    --output-path $PWD/site/dist
