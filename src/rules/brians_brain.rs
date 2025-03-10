use crate::rules::Ruleset;
use crate::world::{Board, Cell};
use nannou::color::LinSrgba;
use nannou::rand;
use std::fmt::Write;

pub struct BriansBrain;
impl BriansBrain {
	pub fn firing() -> Cell {
		Cell {
			ruleset: Ruleset::BriansBrain,
			state: State::Firing as u8,
		}
	}

	pub fn refractory() -> Cell {
		Cell {
			ruleset: Ruleset::BriansBrain,
			state: State::Refractory as u8,
		}
	}

	pub fn dead() -> Cell {
		Cell {
			ruleset: Ruleset::BriansBrain,
			state: State::Dead as u8,
		}
	}

	pub fn random() -> Cell {
		Cell {
			ruleset: Ruleset::BriansBrain,
			state: rand::random_range(0, 3),
		}
	}

	fn state(cell: Cell) -> State {
		if cell.state <= State::Dead as u8 {
			State::Dead
		} else if cell.state == State::Firing as u8 {
			State::Firing
		} else {
			State::Refractory
		}
	}

	pub fn color(cell: Cell) -> LinSrgba {
		match BriansBrain::state(cell) {
			State::Firing => LinSrgba::new(1.0, 0.0, 1.0, 1.0),
			State::Refractory => LinSrgba::new(0.0, 1.0, 1.0, 1.0),
			State::Dead => LinSrgba::new(0.0, 0.0, 0.0, 1.0),
		}
	}

	pub fn next_cell_state(board: &Board, row: usize, col: usize) -> Cell {
		next_cell_state(board, row, col)
	}

	pub fn write_debug<W: Write>(output: &mut W, state: u8) -> std::fmt::Result {
		write!(output, "{:02b}", state)
	}
}

#[derive(PartialEq, Eq)]
#[repr(u8)]
enum State {
	Dead = 0,
	Firing = 1,
	Refractory = 2,
}

fn next_cell_state(board: &Board, row: usize, col: usize) -> Cell {
	let idx = row * board.width + col;
	match BriansBrain::state(board[idx]) {
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
	}
}

fn count_firing_row_neighbors(board: &Board, row: usize, col: usize, exclude_center: bool) -> u8 {
	let idx = row * board.width + col;

	let mut live = if exclude_center {
		0
	} else {
		board[idx].state & 0b01
	};

	if col > 0 {
		live <<= 1;
		live |= board[idx - 1].state & 0b01;
	}

	if (col + 1) < board.width {
		live <<= 1;
		live |= board[idx + 1].state & 0b01;
	}

	live
}

fn count_firing_neighbors(board: &Board, row: usize, col: usize) -> u32 {
	let mut firing_neighbors = count_firing_row_neighbors(board, row, col, true);

	if row > 0 {
		firing_neighbors <<= 3;
		firing_neighbors |= count_firing_row_neighbors(board, row - 1, col, false);
	}

	if row + 1 < board.height {
		firing_neighbors <<= 3;
		firing_neighbors |= count_firing_row_neighbors(board, row + 1, col, false);
	}

	firing_neighbors.count_ones()
}
