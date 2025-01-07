use crate::rules::Ruleset;
use crate::world::{Cell, BOARD_HEIGHT, BOARD_WIDTH};
use nannou::color::{encoding::Srgb, rgb::Rgb, *};
use nannou::rand;
use std::fmt::Write;

pub struct Seeds;
impl Seeds {
	pub fn alive() -> Cell {
		Cell {
			ruleset: Ruleset::Seeds,
			state: State::Alive as u8,
		}
	}

	fn dead() -> Cell {
		Cell {
			ruleset: Ruleset::Seeds,
			state: State::Dead as u8,
		}
	}

	pub fn random() -> Cell {
		Cell {
			ruleset: Ruleset::Seeds,
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

	pub fn color(cell: Cell) -> Rgb<Srgb, u8> {
		match Seeds::state(cell) {
			State::Alive => WHITE,
			State::Dead => BLACK,
		}
	}

	pub fn next_cell_state(board: &[Cell], row: usize, col: usize) -> Cell {
		let idx = row * BOARD_WIDTH + col;
		if board[idx].state == State::Alive as u8 {
			Seeds::dead()
		} else {
			let live_neighbors = count_live_neighbors(board, row, col);
			if live_neighbors == 2 {
				Seeds::alive()
			} else {
				Seeds::dead()
			}
		}
	}

	pub fn write_debug<W: Write>(output: &mut W, state: u8) -> std::fmt::Result {
		write!(output, "{:02b}", state)
	}
}

#[derive(PartialEq, Eq)]
#[repr(u8)]
enum State {
	Dead = 0,
	Alive = 1,
}

fn count_live_row_neighbors(board: &[Cell], row: usize, col: usize, exclude_center: bool) -> u8 {
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

fn count_live_neighbors(board: &[Cell], row: usize, col: usize) -> u32 {
	let mut live_neighbors = count_live_row_neighbors(board, row, col, true);

	if row > 0 {
		live_neighbors <<= 3;
		live_neighbors |= count_live_row_neighbors(board, row - 1, col, false);
	}

	if row + 1 < BOARD_HEIGHT {
		live_neighbors <<= 3;
		live_neighbors |= count_live_row_neighbors(board, row + 1, col, false);
	}

	live_neighbors.count_ones()
}
