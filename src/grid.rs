use crate::square_walls::SquareWalls;
use crate::wall::Wall;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct Grid {
    column_count: i32,
    row_count: i32,
    columns: Vec<Vec<bool>>,
    rows: Vec<Vec<bool>>,
    wall_count: Vec<Vec<i32>>,
}

impl Grid {
    pub fn new(column_count: i32, row_count: i32) -> Self {
        let mut new_grid = Self {
            column_count,
            row_count,
            columns: vec![vec![false; (row_count - 1) as usize]; column_count as usize],
            rows: vec![vec![false; row_count as usize]; (column_count - 1) as usize],
            wall_count: vec![vec![0; (row_count - 1) as usize]; (column_count - 1) as usize],
        };
        new_grid.recalculate_wall_count();
        return new_grid;
    }

    pub fn column_count(&self) -> i32 {
        self.column_count
    }

    pub fn row_count(&self) -> i32 {
        self.row_count
    }

    pub fn set_wall(&mut self, is_column: bool, column: i32, row: i32, set: bool) {
        // Check indices.
        if column < 0 {
            panic!("Column index {} is below 0.", column);
        }
        if row < 0 {
            panic!("Row index {} is below 0.", row);
        }
        if is_column {
            if column > self.column_count - 1 {
                panic!(
                    "Column index {} is greater than {}.",
                    column,
                    self.column_count - 1
                );
            }
            if row > self.row_count - 2 {
                panic!("Row index {} is greater than {}.", row, self.row_count - 2);
            }
        } else {
            if column > self.column_count - 2 {
                panic!(
                    "Column index {} is greater than {}.",
                    column,
                    self.column_count - 2
                );
            }
            if row > self.row_count - 1 {
                panic!("Row index {} is greater than {}.", row, self.row_count - 1);
            }
        }

        // Update wall data.
        let wall_count_increment = if set { 1 } else { -1 };
        if is_column {
            self.columns[column as usize][row as usize] = set;
            // Update wall counts.
            if column > 0 {
                self.wall_count[(column - 1) as usize][row as usize] += wall_count_increment;
            } // Not the first column: update the left square.
            if column < self.column_count - 1 {
                self.wall_count[column as usize][row as usize] += wall_count_increment;
            } // Not the last column: update right square.
        } else {
            self.rows[column as usize][row as usize] = set;
            // Update wall counts.
            if row > 0 {
                self.wall_count[column as usize][(row - 1) as usize] += wall_count_increment;
            } // Not the first row: update the up square.
            if row < self.row_count - 1 {
                self.wall_count[column as usize][row as usize] += wall_count_increment;
            } // Not the last row: update down square.
        }
    }

    pub fn set_wall_with_wall(&mut self, wall: &Wall, set: bool) {
        self.set_wall(wall.is_column, wall.column, wall.row, set);
    }

    pub fn get_wall(&self, is_column: bool, column: i32, row: i32) -> Wall {
        // Check indices.
        if column < 0 {
            panic!("Column index {} is below 0.", column);
        }
        if row < 0 {
            panic!("Row index {} is below 0.", row);
        }
        if is_column {
            if column > self.column_count - 1 {
                panic!(
                    "Column index {} is greater than {}.",
                    column,
                    self.column_count - 1
                );
            }
            if row > self.row_count - 2 {
                panic!("Row index {} is greater than {}.", row, self.row_count - 2);
            }
        } else {
            if column > self.column_count - 2 {
                panic!(
                    "Column index {} is greater than {}.",
                    column,
                    self.column_count - 2
                );
            }
            if row > self.row_count - 1 {
                panic!("Row index {} is greater than {}.", row, self.row_count - 1);
            }
        }

        // Return a new wall object.
        Wall {
            set: if is_column {
                self.columns[column as usize][row as usize]
            } else {
                self.rows[column as usize][row as usize]
            },
            is_column,
            column,
            row,
        }
    }

    pub fn recalculate_wall_count(&mut self) {
        for column in 0..self.column_count {
            for row in 0..self.row_count {
                if row < self.row_count - 1 && self.get_wall(true, column, row).set {
                    if row < self.row_count - 1 && self.get_wall(true, column, row).set {
                        // Not the last row and the column wall is set.
                        if column < self.column_count - 1 {
                            self.wall_count[column as usize][row as usize] += 1;
                        } // Right square.
                        if column > 0 {
                            self.wall_count[(column - 1) as usize][row as usize];
                        } // Left square.
                    }
                    if column < self.column_count - 1 && self.get_wall(false, column, row).set {
                        // Not the last column and the row wall is set.
                        if row < self.row_count - 1 {
                            self.wall_count[column as usize][row as usize] += 1;
                        } // Below square.
                        if row > 0 {
                            self.wall_count[column as usize][(row - 1) as usize] += 1;
                        } // Above square.
                    }
                }
            }
        }
    }

    pub fn get_square_walls(&self, column: i32, row: i32) -> SquareWalls {
        // Check indices.
        if column < 0 {
            panic!("Column index {} is below 0.", column);
        }
        if row < 0 {
            panic!("Row index {} is below 0.", row);
        }
        if column > self.column_count - 2 {
            panic!(
                "Column index {} is greater than {}.",
                column,
                self.column_count - 2
            );
        }
        if row > self.row_count - 2 {
            panic!("Row index {} is greater than {}.", row, self.row_count - 2);
        }

        SquareWalls {
            top: Wall {
                set: self.rows[column as usize][row as usize],
                is_column: false,
                column,
                row,
            },
            right: Wall {
                set: self.columns[(column + 1) as usize][row as usize],
                is_column: true,
                column: column + 1,
                row,
            },
            bottom: Wall {
                set: self.rows[column as usize][(row + 1) as usize],
                is_column: false,
                column,
                row: row + 1,
            },
            left: Wall {
                set: self.columns[column as usize][row as usize],
                is_column: true,
                column,
                row,
            },
        }
    }

    pub fn get_wall_count(&self, column: i32, row: i32) -> i32 {
        // Check indices.
        if column < 0 {
            panic!("Column index {} is below 0.", column);
        }
        if row < 0 {
            panic!("Row index {} is below 0.", row);
        }
        if column > self.column_count - 2 {
            panic!(
                "Column index {} is greater than {}.",
                column,
                self.column_count - 2
            );
        }
        if row > self.row_count - 2 {
            panic!("Row index {} is greater than {}.", row, self.row_count - 2);
        }

        self.wall_count[column as usize][row as usize]
    }

    pub fn get_optimal_moves(&self) -> HashSet<Wall> {
        let mut optimal_moves = HashSet::new();

        // Find unset walls on squares with three set walls.
        println!("Searching for creatable boxes.");
        for column in 0..(self.column_count - 1) {
            for row in 0..(self.row_count - 1) {
                if self.get_wall_count(column, row) == 3 {
                    // The square has three walls. Fill in the last wall to complete the square.
                    print!("    Coordinate: ({column}, {row}), Walls:");
                    let square_walls = self.get_square_walls(column, row);
                    // Exactly one of the below conditions should evaluate to true.
                    if !square_walls.top.set {
                        print!(" top");
                        optimal_moves.insert(square_walls.top);
                    }
                    if !square_walls.right.set {
                        print!(" right");
                        optimal_moves.insert(square_walls.right);
                    }
                    if !square_walls.bottom.set {
                        print!(" bottom");
                        optimal_moves.insert(square_walls.bottom);
                    }
                    if !square_walls.left.set {
                        print!(" left");
                        optimal_moves.insert(square_walls.left);
                    }
                    println!(".");
                }
            }
        }
        if optimal_moves.len() > 0 {
            return optimal_moves;
        }

        // Find walls with adjacent squares with less than two walls.
        println!("Searching for safe moves.");
        for column in 0..(self.column_count - 1) {
            for row in 0..(self.row_count - 1) {
                if self.get_wall_count(column, row) < 2 {
                    // The square has less than two walls.
                    print!("    Coordinate: ({column}, {row}), Walls:");
                    let square_walls = self.get_square_walls(column, row);
                    if !square_walls.top.set
                        && (row == 0 || self.get_wall_count(column, row - 1) < 2)
                    {
                        // The top wall is not set and the above square (if one exists) has less than two walls.
                        print!(" top");
                        optimal_moves.insert(square_walls.top);
                    }
                    if !square_walls.right.set
                        && (column == self.column_count - 2
                        || self.get_wall_count(column + 1, row) < 2)
                    {
                        // The right wall is not set and the right square (if one exists) has less than two walls.
                        print!(" right");
                        optimal_moves.insert(square_walls.right);
                    }
                    if !square_walls.bottom.set
                        && (row == self.row_count - 2 || self.get_wall_count(column, row + 1) < 2)
                    {
                        // The bottom wall is not set and the below square (if one exists) has less than two walls.
                        print!(" bottom");
                        optimal_moves.insert(square_walls.bottom);
                    }
                    if !square_walls.left.set
                        && (column == 0 || self.get_wall_count(column - 1, row) < 2)
                    {
                        // The left wall is not set and the left square (if one exists) has less than two walls.
                        print!(" left");
                        optimal_moves.insert(square_walls.left);
                    }
                    println!(".");
                }
            }
        }
        if optimal_moves.len() > 0 {
            return optimal_moves;
        }

        // Find walls that each trigger a shortest chain (multiple chains may have the least length).
        // Save grid data because it will be temporarily mutated when determining chains (this helps
        // simplify cases where chains wrap around). When a chain ends along its length (not at its
        // other end to form a perfect loop) walls in its "tail" will result in a shorter chain.

        fn get_chain_length(temporary_grid: &mut Grid, mut column: i32, mut row: i32) -> i32 {
            println!("    Evaluating chain ({column}, {row}):");
            let mut chain_length = 0;

            // While the current position is in squares and the current square has three walls.
            while column >= 0
                && column < temporary_grid.column_count - 1
                && row >= 0
                && row < temporary_grid.row_count - 1
                && temporary_grid.get_wall_count(column, row) == 3
            {
                print!("        Coordinate: ({column}, {row}), Length: {chain_length}. Setting ");

                // Set the unset wall and move in that direction to the next square.
                let square_walls = temporary_grid.get_square_walls(column, row);
                if !square_walls.top.set {
                    println!("top");
                    temporary_grid.set_wall_with_wall(&square_walls.top, true);
                    row -= 1;
                } else if !square_walls.right.set {
                    println!("right");
                    temporary_grid.set_wall_with_wall(&square_walls.right, true);
                    column += 1;
                } else if !square_walls.bottom.set {
                    println!("bottom");
                    temporary_grid.set_wall_with_wall(&square_walls.bottom, true);
                    row += 1;
                } else if !square_walls.left.set {
                    println!("left");
                    temporary_grid.set_wall_with_wall(&square_walls.left, true);
                    column -= 1;
                } else {
                    println!("none");
                    break;
                }

                // A square must have been made. Increase the length of this chain.
                chain_length += 1;

                // Handles the edge case where the last line results in two boxes instead of one.
                if column >= 0
                    && column < temporary_grid.column_count - 1
                    && row >= 0
                    && row < temporary_grid.row_count - 1
                    && temporary_grid.get_wall_count(column, row) == 4
                {
                    chain_length += 1;
                }
            }

            println!("        Chain length: {chain_length}.");
            chain_length
        }

        // Get the length of each chain triggered by a wall.
        println!("Searching for the shortest chains.");
        let mut chain_lengths: HashMap<Wall, i32> = HashMap::new();
        for column in 0..(self.column_count - 1) {
            for row in 0..(self.row_count - 1) {
                if self.get_wall_count(column, row) == 2 {
                    // The square has two walls.

                    let first_square = self.get_square_walls(column, row);

                    // For each unset of wall of this square, calculate (if not already done) the length of the chain triggered if it was set.
                    // Both the length of "sides" of the chain need to be calculated. The first starts in the square we are already in and
                    // the second starts in the square in direction of the triggering wall.
                    if !first_square.top.set && !chain_lengths.contains_key(&first_square.top) {
                        let chain_start = first_square.top;
                        let mut temporary_grid = self.clone();
                        temporary_grid.set_wall_with_wall(&chain_start, true);
                        chain_lengths.insert(
                            chain_start,
                            get_chain_length(&mut temporary_grid, column, row)
                                + get_chain_length(&mut temporary_grid, column, row - 1),
                        );
                    }
                    if !first_square.right.set && !chain_lengths.contains_key(&first_square.right) {
                        let chain_start = first_square.right;
                        let mut temporary_grid = self.clone();
                        temporary_grid.set_wall_with_wall(&chain_start, true);
                        chain_lengths.insert(
                            chain_start,
                            get_chain_length(&mut temporary_grid, column, row)
                                + get_chain_length(&mut temporary_grid, column + 1, row),
                        );
                    }
                    if !first_square.bottom.set && !chain_lengths.contains_key(&first_square.bottom)
                    {
                        let chain_start = first_square.bottom;
                        let mut temporary_grid = self.clone();
                        temporary_grid.set_wall_with_wall(&chain_start, true);
                        chain_lengths.insert(
                            chain_start,
                            get_chain_length(&mut temporary_grid, column, row)
                                + get_chain_length(&mut temporary_grid, column, row + 1),
                        );
                    }
                    if !first_square.left.set && !chain_lengths.contains_key(&first_square.left) {
                        let chain_start = first_square.left;
                        let mut temporary_grid = self.clone();
                        temporary_grid.set_wall_with_wall(&chain_start, true);
                        chain_lengths.insert(
                            chain_start,
                            get_chain_length(&mut temporary_grid, column, row)
                                + get_chain_length(&mut temporary_grid, column - 1, row),
                        );
                    }
                }
            }
        }

        // Get the walls that each trigger the shortest chain.
        let minimum_length = *chain_lengths.values().min().unwrap_or(&0);
        for (wall, length) in chain_lengths {
            println!(
                "    {} ({}, {}) generates a chain with length {length}.",
                if wall.is_column { "Column" } else { "Row" },
                wall.column,
                wall.row
            );
            if length == minimum_length {
                optimal_moves.insert(wall);
            }
        }

        optimal_moves
    }
}
