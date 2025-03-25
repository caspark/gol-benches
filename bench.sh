#!/usr/bin/env bash

set -eux

if [ $# -lt 1 ]; then
    echo "Usage: $0 [args...]"
    echo "Arguments will be passed to each implementation"
    exit 1
fi

# Build anything that needs building first
./build.sh

# Run benchmarks with hyperfine
hyperfine \
    --warmup 1 \
    --min-runs 3 \
    --max-runs 10 \
    --export-markdown bench_results.md \
    --export-json bench_results.json \
    "./run.sh lua $*" \
    "./run.sh luajit $*" \
    "./run.sh rust $*"
