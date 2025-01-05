use crate::rules::Ruleset;
use crate::world::{Cell, BOARD_HEIGHT, BOARD_WIDTH};
use nannou::color::{encoding::Srgb, rgb::Rgb, WHITE};
use nannou::rand;

pub struct Life;
impl Life {
	pub fn alive() -> Cell {
		Cell {
			ruleset: Ruleset::Life,
			state: State::Alive as u8,
		}
	}

	fn dead() -> Cell {
		Cell {
			ruleset: Ruleset::Life,
			state: State::Dead as u8,
		}
	}

	pub fn random() -> Cell {
		Cell {
			ruleset: Ruleset::Life,
			state: rand::random_range(0, 2),
		}
	}

	fn state(cell: Cell) -> State {
		if cell.state >= State::Alive as u8 {
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
		Debug(Life::state(cell) as u8)
	}

	pub fn next_cell_state(board: &[Cell], row: usize, col: usize) -> Cell {
		next_cell_state(board, row, col)
	}
}

#[derive(PartialEq, Eq)]
#[repr(u8)]
enum State {
	Dead = 0,
	Alive = 1,
}

pub struct Debug(u8);

impl std::fmt::Debug for Debug {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.0.fmt(fmt)
	}
}

fn count_live_row_neighbors(board: &[Cell], row: usize, col: usize) -> u8 {
	let idx = row * BOARD_WIDTH + col;

	let mut live = if Life::state(board[idx]) == State::Alive {
		1
	} else {
		0
	};

	if col > 0 {
		if Life::state(board[idx - 1]) == State::Alive {
			live += 1;
		}
	}

	if (col + 1) < BOARD_WIDTH {
		if Life::state(board[idx + 1]) == State::Alive {
			live += 1;
		}
	}
	live
}

fn count_live_neighbors(board: &[Cell], row: usize, col: usize) -> u8 {
	let mut live_neighbors = 0;

	if row > 0 {
		live_neighbors += count_live_row_neighbors(board, row - 1, col);
	}

	live_neighbors += count_live_row_neighbors(board, row, col);
	// Exclude the central 'self' if it was counted as alive
	if Life::state(board[row * BOARD_WIDTH + col]) == State::Alive {
		live_neighbors -= 1;
	}

	if row + 1 < BOARD_HEIGHT {
		live_neighbors += count_live_row_neighbors(board, row + 1, col);
	}

	live_neighbors
}

fn next_cell_state(board: &[Cell], row: usize, col: usize) -> Cell {
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
