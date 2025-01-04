use crate::world::{Board, Cell, BOARD_HEIGHT, BOARD_WIDTH};
use nannou::color::{encoding::Srgb, rgb::Rgb};
use nannou::prelude::{BLUE, WHITE};
use nannou::rand;

pub struct BriansBrain;
impl BriansBrain {
    pub fn firing() -> Cell {
        Cell(State::Firing as u8)
    }

    pub fn refractory() -> Cell {
        Cell(State::Refractory as u8)
    }

    pub fn dead() -> Cell {
        Cell(State::Dead as u8)
    }

    pub fn random() -> Cell {
        Cell(rand::random_range::<u8>(0, 3))
    }

    fn state(cell: Cell) -> State {
        if cell.0 < 1 {
            State::Dead
        } else if cell.0 == 1 {
            State::Refractory
        } else {
            State::Firing
        }
    }

    pub fn color(cell: Cell) -> Option<Rgb<Srgb, u8>> {
        match BriansBrain::state(cell) {
            State::Firing => Some(WHITE),
            State::Refractory => Some(BLUE),
            State::Dead => None,
        }
    }

    #[allow(dead_code)]
    pub fn debug(cell: Cell) -> Debug {
        Debug(BriansBrain::state(cell) as u8)
    }
}

#[derive(PartialEq, Eq)]
#[repr(u8)]
enum State {
    Dead = 0,
    Refractory = 1,
    Firing = 2,
}

pub struct Debug(u8);

impl std::fmt::Debug for Debug {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(fmt)
    }
}

pub fn brians_brain(board: &Board, next_board: &mut Board) {
    for row in 0..BOARD_HEIGHT {
        for col in 0..BOARD_WIDTH {
            let idx = row * BOARD_WIDTH + col;
            next_board[idx] = match BriansBrain::state(board[idx]) {
                State::Firing => BriansBrain::refractory(),
                State::Refractory => BriansBrain::dead(),
                State::Dead => {
                    let firing_neighbors = count_firing_neighbors(board, row, col);
                    if firing_neighbors == 2 {
                        BriansBrain::firing()
                    } else {
                        BriansBrain::dead()
                    }
                }
            };
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Delta {
    LT,
    EQ,
    GT,
}

fn count_firing_neighbors(board: &Board, row: usize, col: usize) -> u8 {
    let mut firing_neighbors = 0;

    for dy in [Delta::LT, Delta::EQ, Delta::GT].into_iter() {
        if (dy == Delta::LT && row == 0) || (dy == Delta::GT && (row + 1) == BOARD_HEIGHT) {
            continue;
        }

        for dx in [Delta::LT, Delta::EQ, Delta::GT].into_iter() {
            if (dx == Delta::LT && col == 0)
                || (dx == Delta::GT && (col + 1) == BOARD_WIDTH)
                || (dx == Delta::EQ && dy == Delta::EQ)
            {
                continue;
            }

            let neighbor_idx = match dy {
                Delta::LT => row - 1,
                Delta::EQ => row,
                Delta::GT => row + 1,
            } * BOARD_WIDTH
                + match dx {
                    Delta::LT => col - 1,
                    Delta::EQ => col,
                    Delta::GT => col + 1,
                };

            if BriansBrain::state(board[neighbor_idx]) == State::Firing {
                firing_neighbors += 1;
            }
        }
    }

    firing_neighbors
}
