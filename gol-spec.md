# Game of Life Implementation Specification

This document specifies the requirements for implementing Conway's Game of Life for benchmarking purposes.

## Command Line Interface

The program must accept exactly 4 command-line arguments in the following order:
1. `size` - Positive integer specifying the size of the square grid
2. `iterations` - Non-negative integer specifying number of generations to simulate
3. `print_mode` - One of: "all", "final", "none"
   - "all": Print initial state and every generation
   - "final": Print only the final state
   - "none": No printing
4. `pattern_file` - Path to a file containing the initial pattern

## Input Validation

- If incorrect number of arguments: display usage message and exit with code 1
- If size ≤ 0 or iterations < 0: display error and exit with code 1
- If print_mode is invalid: display error and exit with code 1
- If pattern file cannot be opened: display error and exit with code 1

## Pattern File Format

- Lines starting with '!' are comments and should be ignored
- Non-comment lines represent rows of the pattern
- 'O' (capital O) represents a live cell
- Any other character represents a dead cell
- Pattern may be rectangular (rows can have different lengths)

## Grid Behavior

- Grid is square with dimensions size × size
- Pattern should be centered on the grid
- Grid edges are fixed (no wrapping around)

## Game Rules

For each generation:
1. All updates happen simultaneously
2. For each cell, count live neighbors (including diagonals, max 8)
3. Apply rules:
   - Live cell with 2 or 3 live neighbors survives
   - Dead cell with exactly 3 live neighbors becomes alive
   - All other cells die or stay dead

## Output Format

When printing the grid:
- Use 'O' for live cells
- Use '.' for dead cells
- No spaces between cells
- Print newline after each row
- Print empty line between generations
- When printing generations, include header "Generation N:" (N = generation number)
- For final state, include header "Final state after N generations:"
- Initial state should be labeled "Initial state:"

## Implementation Notes

- Do not use any libraries
- Program should be optimized for performance
- Memory usage should be reasonable for the given grid size
- No interactive features or GUI elements
- Output must exactly match the format specified above
