use crate::rules::Ruleset;
use crate::world::{Board, Cell, BOARD_HEIGHT, BOARD_WIDTH};
use nannou::color::{encoding::Srgb, rgb::Rgb, BLUE, WHITE};
use nannou::rand;

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

	pub fn next_cell_state(board: &Board, row: usize, col: usize) -> Cell {
		next_cell_state(board, row, col)
	}
}

#[derive(PartialEq, Eq)]
#[repr(u8)]
enum State {
	Dead = 0,
	Firing = 1,
	Refractory = 2,
}

pub struct Debug(u8);

impl std::fmt::Debug for Debug {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.0.fmt(fmt)
	}
}

fn next_cell_state(board: &Board, row: usize, col: usize) -> Cell {
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

fn count_firing_row_neighbors(board: &Board, row: usize, col: usize) -> u8 {
	let idx = row * BOARD_WIDTH + col;

	let mut live = if BriansBrain::state(board[idx]) == State::Firing {
		1
	} else {
		0
	};

	if col > 0 {
		if BriansBrain::state(board[idx - 1]) == State::Firing {
			live += 1;
		}
	}

	if (col + 1) < BOARD_WIDTH {
		if BriansBrain::state(board[idx + 1]) == State::Firing {
			live += 1;
		}
	}
	live
}

fn count_firing_neighbors(board: &Board, row: usize, col: usize) -> u8 {
	let mut firing_neighbors = 0;

	if row > 0 {
		firing_neighbors += count_firing_row_neighbors(board, row - 1, col);
	}

	firing_neighbors += count_firing_row_neighbors(board, row, col);
	// Exclude the central 'self' if it was counted as alive
	if BriansBrain::state(board[row * BOARD_WIDTH + col]) == State::Firing {
		firing_neighbors -= 1;
	}

	if row + 1 < BOARD_HEIGHT {
		firing_neighbors += count_firing_row_neighbors(board, row + 1, col);
	}

	firing_neighbors
}
