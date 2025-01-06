use crate::rules::Ruleset;
use crate::world::{Cell, BOARD_HEIGHT, BOARD_WIDTH};
use nannou::color::{encoding::Srgb, rgb::Rgb, *};
use nannou::rand;

pub struct AntiLife;
impl AntiLife {
	pub fn alive() -> Cell {
		Cell {
			ruleset: Ruleset::AntiLife,
			state: State::Alive as u8,
		}
	}

	pub fn dead() -> Cell {
		Cell {
			ruleset: Ruleset::AntiLife,
			state: State::Dead as u8,
		}
	}

	pub fn random() -> Cell {
		Cell {
			ruleset: Ruleset::AntiLife,
			state: rand::random(),
		}
	}

	fn state(cell: Cell) -> State {
		if cell.state >= 1 {
			State::Alive
		} else {
			State::Dead
		}
	}

	pub fn color(cell: Cell) -> Rgb<Srgb, u8> {
		match AntiLife::state(cell) {
			State::Alive => WHITE,
			State::Dead => BLACK,
		}
	}

	#[allow(dead_code)]
	pub fn debug(cell: Cell) -> Debug {
		Debug(AntiLife::state(cell) as u8)
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

fn next_cell_state(board: &[Cell], row: usize, col: usize) -> Cell {
	let live_neighbors = count_live_neighbors(board, row, col);
	let idx = row * BOARD_WIDTH + col;

	match (AntiLife::state(board[idx]), live_neighbors) {
		(State::Alive, x) if x != 5 => AntiLife::alive(),
		(State::Dead, x) if x != 5 && x != 6 => AntiLife::alive(),
		_ => AntiLife::dead(),
	}
}