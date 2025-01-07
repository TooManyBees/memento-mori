use crate::rules::Ruleset;
use crate::world::{Cell, BOARD_HEIGHT, BOARD_WIDTH};
use nannou::color::{encoding::Srgb, rgb::Rgb, *};
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

	pub fn color(cell: Cell) -> Rgb<Srgb, u8> {
		match BriansBrain::state(cell) {
			State::Firing => FUCHSIA,
			State::Refractory => CYAN,
			State::Dead => BLACK,
		}
	}

	pub fn next_cell_state(board: &[Cell], row: usize, col: usize) -> Cell {
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

fn next_cell_state(board: &[Cell], row: usize, col: usize) -> Cell {
	let idx = row * BOARD_WIDTH + col;
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

fn count_firing_row_neighbors(board: &[Cell], row: usize, col: usize, exclude_center: bool) -> u8 {
	let idx = row * BOARD_WIDTH + col;

	let mut live = if exclude_center {
		0
	} else {
		board[idx].state & 0b01
	};

	if col > 0 {
		live <<= 1;
		live |= board[idx - 1].state & 0b01;
	}

	if (col + 1) < BOARD_WIDTH {
		live <<= 1;
		live |= board[idx + 1].state & 0b01;
	}

	live
}

fn count_firing_neighbors(board: &[Cell], row: usize, col: usize) -> u32 {
	let mut firing_neighbors = count_firing_row_neighbors(board, row, col, true);

	if row > 0 {
		firing_neighbors <<= 3;
		firing_neighbors |= count_firing_row_neighbors(board, row - 1, col, false);
	}

	if row + 1 < BOARD_HEIGHT {
		firing_neighbors <<= 3;
		firing_neighbors |= count_firing_row_neighbors(board, row + 1, col, false);
	}

	firing_neighbors.count_ones()
}
