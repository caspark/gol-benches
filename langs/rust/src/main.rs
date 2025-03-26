use std::env;
use std::fmt::{self, Display, Write as _};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

/// Error type for Game of Life operations
#[allow(dead_code)]
#[derive(Debug)]
enum AppErr {
    /// IO operation failed
    Io(io::Error),
    /// Invalid argument count
    InvalidArgCount(usize),
    /// Invalid size parameter
    InvalidSize(String),
    /// Invalid iterations parameter
    InvalidIterations(String),
    /// Invalid print mode
    InvalidPrintMode(String),
}

impl From<io::Error> for AppErr {
    fn from(err: io::Error) -> Self {
        AppErr::Io(err)
    }
}

struct Args {
    print_mode: String,
    size: usize,
    iterations: usize,
    pattern_file: String,
}

/// Represents the game grid and contains all game logic
#[derive(Debug, Clone)]
struct Grid {
    size: usize,
    cells: Vec<bool>,
}

impl Grid {
    /// Creates a new grid with the specified size
    ///
    /// # Arguments
    /// * `size` - The width/height of the square grid
    fn new(size: usize) -> Self {
        Grid {
            size,
            cells: vec![false; size * size],
        }
    }

    /// Gets the cell state at the specified coordinates
    ///
    /// # Arguments
    /// * `row` - The row index
    /// * `col` - The column index
    fn get(&self, row: usize, col: usize) -> bool {
        self.cells[row * self.size + col]
    }

    /// Sets the cell state at the specified coordinates
    ///
    /// # Arguments
    /// * `row` - The row index
    /// * `col` - The column index
    /// * `value` - The new cell state
    fn set(&mut self, row: usize, col: usize, value: bool) {
        self.cells[row * self.size + col] = value;
    }

    /// Counts live neighbors for a cell at the specified coordinates
    ///
    /// # Arguments
    /// * `row` - The row index
    /// * `col` - The column index
    fn count_live_neighbors(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        let row = row as i32;
        let col = col as i32;
        let size = self.size as i32;

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let new_row = row + i;
                let new_col = col + j;

                if new_row >= 0 && new_row < size && new_col >= 0 && new_col < size {
                    if self.get(new_row as usize, new_col as usize) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    /// Computes the next generation of the game grid
    fn next_generation(&self) -> Grid {
        let mut next = Grid::new(self.size);

        for row in 0..self.size {
            for col in 0..self.size {
                let neighbors = self.count_live_neighbors(row, col);
                let is_alive = self.get(row, col);

                next.set(
                    row,
                    col,
                    match (is_alive, neighbors) {
                        (true, 2) | (true, 3) => true,
                        (false, 3) => true,
                        _ => false,
                    },
                );
            }
        }

        next
    }

    /// Loads a pattern into the grid, centered
    ///
    /// # Arguments
    /// * `pattern_file` - Path to the pattern file
    fn load_pattern_from_file(&mut self, pattern_file: &str) -> Result<(), AppErr> {
        let file = File::open(pattern_file)?;
        let reader = BufReader::new(file);
        let mut pattern_rows = Vec::new();
        let mut max_width = 0;

        for line in reader.lines() {
            let line = line?;
            if line.starts_with('!') {
                continue;
            }

            let row: Vec<bool> = line.chars().map(|c| c == 'O').collect();
            max_width = max_width.max(row.len());
            pattern_rows.push(row);
        }

        let pattern_height = pattern_rows.len();
        let start_row = (self.size - pattern_height) / 2;
        let start_col = (self.size - max_width) / 2;

        for (i, row) in pattern_rows.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                self.set(start_row + i, start_col + j, cell);
            }
        }

        Ok(())
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.size {
            for col in 0..self.size {
                f.write_char(if self.get(row, col) { 'O' } else { '.' })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn run(print_mode: &str, size: usize, iterations: usize, pattern_file: &str) -> Result<(), AppErr> {
    let mut grid = Grid::new(size);
    grid.load_pattern_from_file(pattern_file)?;

    if print_mode == "all" {
        println!("Initial state:\n{}", grid);
    }

    let mut current = grid;
    for i in 1..=iterations {
        current = current.next_generation();
        if print_mode == "all" {
            println!("Generation {}:\n{}", i, current);
        }
    }

    if print_mode == "final" {
        println!("Final state after {} generations:\n{}", iterations, current);
    }

    Ok(())
}

fn parse_args(args: &[String]) -> Result<Args, AppErr> {
    if args.len() != 5 {
        return Err(AppErr::InvalidArgCount(args.len()));
    }

    let print_mode = args[1].to_owned();
    if !["all", "final", "none"].contains(&print_mode.as_str()) {
        return Err(AppErr::InvalidPrintMode(print_mode));
    }

    let size: usize = args[2]
        .parse()
        .map_err(|_| AppErr::InvalidSize(args[2].clone()))?;
    if size == 0 {
        return Err(AppErr::InvalidSize(size.to_string()));
    }

    let iterations: usize = args[3]
        .parse()
        .map_err(|_| AppErr::InvalidIterations(args[3].clone()))?;

    let pattern_file = args[4].to_owned();

    Ok(Args {
        print_mode,
        size,
        iterations,
        pattern_file,
    })
}

fn main() {
    let arg_strings: Vec<String> = env::args().collect();

    let args = match parse_args(&arg_strings) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            eprintln!(
                "Usage: {} <print_mode> <size> <iterations> <pattern_file>",
                arg_strings[0]
            );
            process::exit(1);
        }
    };

    if let Err(e) = run(
        &args.print_mode,
        args.size,
        args.iterations,
        &args.pattern_file,
    ) {
        eprintln!("Error: {:?}", e);
        process::exit(1);
    }
}
