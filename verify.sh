#!/usr/bin/env bash

# Verify that all implementations produce the same output for the given arguments

set -eu

if [ $# -lt 1 ]; then
    echo "Usage: $0 <args...>"
    echo "Example: $0 100 100 100"
    exit 1
fi

# Run each language implementation and capture the command
echo "Verifying identical outputs for: ./run.sh <lang> $*"

# Use the verifier to compare outputs across all languages
cargo run --release --bin verifier -- \
    "./run.sh rust $*" \
    "./run.sh luajit $*" \
    "./run.sh lua $*" \
    "./run.sh rust-wasip2 $*" \
    "./run.sh rust-wasip2-precompile $*"
