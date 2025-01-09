use crate::rules::Ruleset;
use crate::world::{Cell, BOARD_HEIGHT, BOARD_WIDTH};
use nannou::color::{encoding::Srgb, rgb::Rgb};
use nannou::rand;
use std::fmt::Write;

pub struct LatticeGas;
impl LatticeGas {
	pub fn empty() -> Cell {
		Cell {
			ruleset: Ruleset::LatticeGas,
			state: 0,
		}
	}

	pub fn random() -> Cell {
		let state = if rand::random::<bool>() {
			LatticeGas::random_dir() | POPULATED
		} else {
			0
		};

		Cell {
			ruleset: Ruleset::LatticeGas,
			state,
		}
	}

	pub fn random_populated() -> Cell {
		Cell {
			ruleset: Ruleset::LatticeGas,
			state: LatticeGas::random_dir() | POPULATED,
		}
	}

	fn random_dir() -> u8 {
		1u8 << rand::random_range::<u8>(1, 5)
	}

	pub fn next_cell_state(board: &[Cell], row: usize, col: usize) -> Cell {
		next_cell_state(board, row, col)
	}

	pub fn color(cell: Cell) -> Rgb<Srgb, f32> {
		if cell.state & POPULATED > 0 {
			Rgb::new(0.0, 0.0, 0.0)
		} else {
			Rgb::new(1.0, 1.0, 1.0)
		}
	}

	fn going_up(cell: Cell) -> u8 {
		cell.state & GOING_UP
	}

	fn going_down(cell: Cell) -> u8 {
		cell.state & GOING_DOWN
	}

	fn going_left(cell: Cell) -> u8 {
		cell.state & GOING_LEFT
	}

	fn going_right(cell: Cell) -> u8 {
		cell.state & GOING_RIGHT
	}

	pub fn write_debug<W: Write>(output: &mut W, state: u8) -> std::fmt::Result {
		write!(output, "{:04b}", state)
	}
}

const GOING_UP: u8 = 0b10000;
const GOING_DOWN: u8 = 0b01000;
const GOING_LEFT: u8 = 0b00100;
const GOING_RIGHT: u8 = 0b00010;
const POPULATED: u8 = 0b00001;

fn next_cell_state(board: &[Cell], row: usize, col: usize) -> Cell {
	let mut combined_states = 0;
	if row > 0 {
		combined_states |= LatticeGas::going_down(board[(row - 1) * BOARD_WIDTH + col]);
	}
	if row < BOARD_HEIGHT - 1 {
		combined_states |= LatticeGas::going_up(board[(row + 1) * BOARD_WIDTH + col]);
	}
	if col > 0 {
		combined_states |= LatticeGas::going_right(board[row * BOARD_WIDTH + col - 1]);
	}
	if col < BOARD_WIDTH - 1 {
		combined_states |= LatticeGas::going_left(board[row * BOARD_WIDTH + col + 1]);
	}

	if combined_states == 0 {
		return Cell {
			ruleset: Ruleset::LatticeGas,
			state: combined_states,
		};
	}

	if combined_states == GOING_UP | GOING_DOWN {
		combined_states = GOING_LEFT | GOING_RIGHT;
	} else if combined_states == GOING_LEFT | GOING_RIGHT {
		combined_states = GOING_UP | GOING_DOWN;
	}

	if col == 0 && combined_states & GOING_LEFT > 0 {
		combined_states = combined_states & (!GOING_LEFT) | GOING_RIGHT;
	} else if col == BOARD_WIDTH - 1 && combined_states & GOING_RIGHT > 0 {
		combined_states = combined_states & (!GOING_RIGHT) | GOING_LEFT;
	}
	if row == 0 && combined_states & GOING_UP > 0 {
		combined_states = combined_states & (!GOING_UP) | GOING_DOWN;
	} else if row == BOARD_HEIGHT - 1 && combined_states & GOING_DOWN > 0 {
		combined_states = combined_states & (!GOING_DOWN) | GOING_UP;
	}

	if combined_states != 0 {
		combined_states |= POPULATED;
	}

	Cell {
		ruleset: Ruleset::LatticeGas,
		state: combined_states,
	}
}
