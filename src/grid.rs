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
        HashSet::new()
    }
}
