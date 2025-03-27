# Game of Life Benchmarks

This repository contains implementations of Conway's Game of Life in different programming languages for benchmarking purposes.

## Requirements

* bash
* [rust](https://www.rust-lang.org/) (tested with 1.85.0)
* [lua](https://www.lua.org/) (tested with 5.2.4)
* [luajit](http://luajit.org/) (tested with 2.1.1720049189)
* [hyperfine](https://github.com/sharkdp/hyperfine) (tested with 1.19.0)
* [wasmtime](https://wasmtime.dev/) (tested with 31.0.0)

NB: no particular effort was taken to write code compatible for those particular versions of tools/languages - they're just what I had installed or had easy access to via Fedora's package manager.

## Supported Languages

- `lua` - whatever version of lua you have installed
- `luajit` - whatever version of luajit you have installed
- `rust` - compile Rust using your default toolchain to native code
- `rust-wasip2` - compile Rust as above but to wasm and run it via wasmtime
- `rust-wasip2-precompiled` as above but also have wasmtime precompile the wasm to native code (no JIT compilation)

The implementations haven't had crazy optimizations applied to them; the goal is to have the
implementations be the "obvious idiomatic" code.

## Usage

### Running a Single Simulation

Use the `run.sh` script to execute a simulation:

```bash
./run.sh <language> [args...]
```

Each implementation accepts exactly 4 command-line arguments as `args`:

1. `print_mode` - One of: "all", "final", "none"
   - "all": Print initial state and every generation
   - "final": Print only the final state
   - "none": No printing
2. `size` - Positive integer specifying the size of the square grid
3. `iterations` - Non-negative integer specifying number of generations to simulate
4. `pattern_file` - Path to a file containing the initial pattern.
   - The pattern format is that of the plaintext format from [the game of life
     wiki](https://conwaylife.com/wiki/); see [the example in patterns/](patterns/).

Example:

```bash
./run.sh rust 80 1024 patterns/10cellinfinitegrowth.cells
```

### Building

Use the `build.sh` script to build all implementations:

```bash
./build.sh
```

### Verification

Use the `verify.sh` script to verify implementations have identical output:

```bash
./verify.sh
```

### Benchmarking

Use the `bench.sh` script to run benchmarks:

```bash
./bench.sh 80 1024 patterns/10cellinfinitegrowth.cells
```

When benchmarking, the implementations skip actually writing each generation's output state to stdout and only the final generation is output; this is to avoid accidentally bottlenecking on IO or string concatenation.

Benchmarking results will be output to stdout and saved to `bench_results.json` and `bench_results.md`.

Here's an example run for 1024 generations on a 80x80 grid:

```
$ ./bench.sh 80 1024 patterns/10cellinfinitegrowth.cells

<earlier output elided>

Summary
  ./run.sh rust final 80 1024 patterns/10cellinfinitegrowth.cells ran
    1.49 ± 0.04 times faster than ./run.sh rust-wasip2-precompile final 80 1024 patterns/10cellinfinitegrowth.cells
    1.57 ± 0.06 times faster than ./run.sh rust-wasip2 final 80 1024 patterns/10cellinfinitegrowth.cells
    7.33 ± 3.00 times faster than ./run.sh luajit final 80 1024 patterns/10cellinfinitegrowth.cells
   89.00 ± 2.43 times faster than ./run.sh lua final 80 1024 patterns/10cellinfinitegrowth.cells
```

## Implementation Details

All implementations follow the specification at [gol-spec.md](gol-spec.md) and should have identical behavior.

Generally you can point an LLM at that spec and they'll do a decent first pass of implementing a new language.
