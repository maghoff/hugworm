#!/usr/bin/env bash

npm install webpack webpack-cli

# install wasm-pack if it's not already in the path
which wasm-pack >/dev/null || (curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh)
