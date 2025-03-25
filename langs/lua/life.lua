#!/usr/bin/env lua

-- Validate command line arguments
if #arg < 4 or #arg > 4 then
    print("Usage: lua life.lua <size> <iterations> <all|final|none> <pattern_file>")
    os.exit(1)
end

local size = tonumber(arg[1])
local iterations = tonumber(arg[2])
local print_mode = arg[3]
local pattern_file = arg[4]

-- Validate numeric inputs
if not size or size < 1 or not iterations or iterations < 0 then
    print("Error: Size must be positive and iterations must be non-negative")
    os.exit(1)
end

-- Validate print mode
if not print_mode or not (print_mode == "all" or print_mode == "final" or print_mode == "none") then
    print("Error: print mode must be one of: all, final, none")
    os.exit(1)
end

-- Create a new board
local function create_board(size)
    local board = {}
    for i = 1, size do
        board[i] = {}
        for j = 1, size do
            board[i][j] = false
        end
    end
    return board
end

-- Load pattern from file and center it on the board
local function load_pattern(filename, board, size)
    local pattern = {}
    local pattern_width = 0
    local pattern_height = 0

    -- Read pattern from file
    local file = io.open(filename, "r")
    if not file then
        print("Error: Cannot open pattern file")
        os.exit(1)
    end

    for line in file:lines() do
        if not line:match("^!") then  -- Skip comments (lines starting with !)
            pattern_height = pattern_height + 1
            pattern[pattern_height] = {}
            pattern_width = math.max(pattern_width, #line)

            for i = 1, #line do
                pattern[pattern_height][i] = (line:sub(i,i) == "O")
            end
        end
    end
    file:close()

    -- Calculate starting position to center the pattern
    local start_row = math.floor((size - pattern_height) / 2) + 1
    local start_col = math.floor((size - pattern_width) / 2) + 1

    -- Place pattern on board
    for i = 1, pattern_height do
        for j = 1, pattern_width do
            if pattern[i][j] then
                board[start_row + i - 1][start_col + j - 1] = true
            end
        end
    end
end

-- Count live neighbors for a cell
local function count_neighbors(board, row, col, size)
    local count = 0
    for i = -1, 1 do
        for j = -1, 1 do
            if not (i == 0 and j == 0) then
                local r = row + i
                local c = col + j
                if r >= 1 and r <= size and c >= 1 and c <= size then
                    if board[r][c] then count = count + 1 end
                end
            end
        end
    end
    return count
end

-- Compute next generation
local function next_generation(board, size)
    local new_board = create_board(size)

    for i = 1, size do
        for j = 1, size do
            local neighbors = count_neighbors(board, i, j, size)
            if board[i][j] then
                -- Live cell survives if it has 2 or 3 neighbors
                new_board[i][j] = (neighbors == 2 or neighbors == 3)
            else
                -- Dead cell becomes alive if it has exactly 3 neighbors
                new_board[i][j] = (neighbors == 3)
            end
        end
    end

    return new_board
end

-- Print the current board state
local function print_board(board, size)
    for i = 1, size do
        local line = ""
        for j = 1, size do
            line = line .. (board[i][j] and "O" or ".")
        end
        print(line)
    end
    print()  -- Empty line between generations
end

-- Main game loop
local board = create_board(size)
load_pattern(pattern_file, board, size)

if print_mode == "all" then
    print("Initial state:")
    print_board(board, size)
end

for gen = 1, iterations do
    board = next_generation(board, size)
    if print_mode == "all" then
        print(string.format("Generation %d:", gen))
        print_board(board, size)
    end
end

if print_mode == "final" then
    print(string.format("Final state after %d generations:", iterations))
    print_board(board, size)
end
