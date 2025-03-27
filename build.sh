#!/usr/bin/env bash

set -eux

# test harness bins
cargo build --release -p verifier

# rust life impls
cargo build --release -p life
cargo build --release -p life --target wasm32-wasip2
wasmtime compile target/wasm32-wasip2/release/life.wasm --output target/wasm32-wasip2/release/life.wasmc
