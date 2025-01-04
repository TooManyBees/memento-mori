use crate::rules::Ruleset;

pub const BOARD_WIDTH: usize = 40;
pub const BOARD_HEIGHT: usize = 40;

#[derive(Copy, Clone, Default, Debug)]
pub struct Cell {
	pub ruleset: Ruleset,
	pub state: u8,
}

pub type Board = [Cell; BOARD_WIDTH * BOARD_HEIGHT];

#[derive(Clone, Debug)]
pub struct World {
	state_a: Board,
	state_b: Board,
	current_board: CurrentBoard,
}

impl World {
	pub fn new() -> Self {
		let state_a = [Cell::default(); BOARD_WIDTH * BOARD_HEIGHT];
		let state_b = state_a.clone();

		World {
			state_a,
			state_b,
			current_board: CurrentBoard::A,
		}
	}

	pub fn board(&self) -> &Board {
		match self.current_board {
			CurrentBoard::A => &self.state_a,
			CurrentBoard::B => &self.state_b,
		}
	}

	fn board_mut(&mut self) -> &mut Board {
		match self.current_board {
			CurrentBoard::A => &mut self.state_a,
			CurrentBoard::B => &mut self.state_b,
		}
	}

	pub fn this_board_and_next(&mut self) -> (&Board, &mut Board) {
		match self.current_board {
			CurrentBoard::A => (&self.state_a, &mut self.state_b),
			CurrentBoard::B => (&self.state_b, &mut self.state_a),
		}
	}

	pub fn randomize(&mut self) {
		for cell in self.board_mut() {
			*cell = cell.ruleset.random();
		}
	}

	pub fn generate(&mut self) {
		let (board, next_board) = self.this_board_and_next();
		for row in 0..BOARD_HEIGHT {
			for col in 0..BOARD_WIDTH {
				let idx = row * BOARD_WIDTH + col;
				let cell = board[idx];
				let next_cell = cell.ruleset.next_cell_state(board, row, col);
				next_board[idx] = next_cell;
			}
		}
	}

	pub fn swap(&mut self) {
		self.current_board = match self.current_board {
			CurrentBoard::A => CurrentBoard::B,
			CurrentBoard::B => CurrentBoard::A,
		};
	}
}

#[derive(Copy, Clone, Debug)]
enum CurrentBoard {
	A,
	B,
}
