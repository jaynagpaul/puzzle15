use std::fmt;

use rand::seq::SliceRandom;
use rand::thread_rng;

const BOARD_SIZE: usize = 4;

/// Puzzle represents the state of a 15puzzle game
pub(crate) struct Puzzle {
    // Location of the 0 (i.e the empty space)
    empty_row: usize,
    empty_col: usize,

    /// The actual board, contains numbers 1-15, and number 0 represents the empty space.
    board: [[i8; BOARD_SIZE]; BOARD_SIZE],
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.board {
            for cell in row {
                let displayed_cell = if cell == 0 {
                    "  ".to_string()
                } else if cell < 10 {
                    format!(" {}", cell)
                } else {
                    format!("{}", cell)
                };

                write!(f, " {displayed_cell} ")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Puzzle {
    pub fn new_random_board() -> Self {
        let mut board = [[0; BOARD_SIZE]; BOARD_SIZE];
        let mut numbers = (0..=15).collect::<Vec<i8>>();
        numbers.shuffle(&mut thread_rng());

        let mut empty_row = 0;
        let mut empty_col = 0;

        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                board[i][j] = numbers.pop().unwrap();
                if board[i][j] == 0 {
                    empty_row = i;
                    empty_col = j;
                }
            }
        }

        Puzzle {
            empty_row,
            empty_col,
            board,
        }
    }

    pub fn has_won(&self) -> bool {
        let mut nums = 1..=15;

        for row in self.board {
            for cell in row {
                let expected = nums.next().unwrap_or(-1);

                if expected != cell {
                    return false;
                }
            }
        }

        true
    }

    pub fn move_left(&mut self) {
        self.move_dir((0, 1));
    }

    pub fn move_right(&mut self) {
        self.move_dir((0, -1));
    }

    pub fn move_up(&mut self) {
        self.move_dir((1, 0));
    }

    pub fn move_down(&mut self) {
        self.move_dir((-1, 0));
    }
}

impl Puzzle {
    fn move_dir(&mut self, dir: (i8, i8)) {
        let can_move = |row, col| {
            let range = 0..(BOARD_SIZE as i8);
            let next_row = dir.0 + row as i8;
            let next_col = dir.1 + col as i8;

            if !range.contains(&next_row) || !range.contains(&next_col) {
                None
            } else {
                Some((next_row as usize, next_col as usize))
            }
        };

        while let Some((next_row, next_col)) = can_move(self.empty_row, self.empty_col) {
            self.board[self.empty_row][self.empty_col] = self.board[next_row][next_col];
            self.empty_row = next_row;
            self.empty_col = next_col;

            self.board[self.empty_row][self.empty_col] = 0;
        }
    }
}
