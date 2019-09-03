#!/usr/bin/env bash

set -e

wasm-pack build

./node_modules/.bin/webpack \
    --entry ./site/script.js \
    --output-public-path /dist/
