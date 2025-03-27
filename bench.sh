#!/usr/bin/env bash

set -eux

if [ $# -lt 1 ]; then
    echo "Usage: $0 [args...]"
    echo "Example: $0 80 1024 patterns/10cellinfinitegrowth.cells"
    echo "Arguments will be passed to each implementation"
    exit 1
fi

# Build anything that needs building first
./build.sh

# Make sure all implementations are given the same output
./verify.sh "all $*"

# Run benchmarks with hyperfine
hyperfine \
    --export-markdown bench_results.md \
    --export-json bench_results.json \
    "./run.sh lua final $*" \
    "./run.sh luajit final $*" \
    "./run.sh rust final $*" \
    "./run.sh rust-wasip2 final $*"
