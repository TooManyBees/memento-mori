use crate::world::{Board, Cell, BOARD_HEIGHT, BOARD_WIDTH};
use nannou::color::{encoding::Srgb, rgb::Rgb};
use nannou::prelude::WHITE;
use nannou::rand;

pub struct Life;
impl Life {
    pub fn alive() -> Cell {
        Cell(1)
    }

    fn dead() -> Cell {
        Cell(0)
    }

    pub fn random() -> Cell {
        Cell(rand::random_range::<u8>(0, 2))
    }

    fn state(cell: Cell) -> State {
        if cell.0 >= 1 {
            State::Alive
        } else {
            State::Dead
        }
    }

    pub fn color(cell: Cell) -> Option<Rgb<Srgb, u8>> {
        match Life::state(cell) {
            State::Alive => Some(WHITE),
            State::Dead => None,
        }
    }

    #[allow(dead_code)]
    pub fn debug(cell: Cell) -> Debug {
        Debug(cell.0)
    }
}

#[derive(PartialEq, Eq)]
enum State {
    Alive,
    Dead,
}

pub struct Debug(u8);

impl std::fmt::Debug for Debug {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.0 == 0 { 0u8 } else { 1u8 }.fmt(fmt)
    }
}

pub fn life(board: &Board, next_board: &mut Board) {
    for row in 0..BOARD_HEIGHT {
        for col in 0..BOARD_WIDTH {
            let next_cell_state = next_cell_state(board, row, col);
            next_board[row * BOARD_WIDTH + col] = next_cell_state;
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Delta {
    LT,
    EQ,
    GT,
}

fn count_live_neighbors(board: &Board, row: usize, col: usize) -> u8 {
    let mut live_neighbors = 0;

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

            if Life::state(board[neighbor_idx]) == State::Alive {
                live_neighbors += 1;
            }
        }
    }

    live_neighbors
}

fn next_cell_state(board: &Board, row: usize, col: usize) -> Cell {
    let live_neighbors = count_live_neighbors(board, row, col);
    let idx = row * BOARD_WIDTH + col;

    match (Life::state(board[idx]), live_neighbors) {
        (State::Alive, 0) | (State::Alive, 1) => Life::dead(),
        (State::Alive, 2) | (State::Alive, 3) => Life::alive(),
        (State::Alive, _) => Life::dead(),
        (State::Dead, 3) => Life::alive(),
        _ => board[idx],
    }
}
