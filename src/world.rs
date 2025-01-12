use crate::rules::Ruleset;

pub const BOARD_WIDTH: usize = 256;
pub const BOARD_HEIGHT: usize = 256;

#[derive(Copy, Clone, Default, Debug)]
pub struct Cell {
	pub ruleset: Ruleset,
	pub state: u8,
}

#[derive(Clone, Debug)]
pub struct World {
	state_a: Vec<Cell>,
	state_b: Vec<Cell>,
	current_board: CurrentBoard,
}

impl World {
	pub fn new() -> Self {
		let state_a = vec![Cell::default(); BOARD_WIDTH * BOARD_HEIGHT];
		let state_b = state_a.clone();

		World {
			state_a,
			state_b,
			current_board: CurrentBoard::A,
		}
	}

	pub fn board(&self) -> &[Cell] {
		match self.current_board {
			CurrentBoard::A => &self.state_a,
			CurrentBoard::B => &self.state_b,
		}
	}

	pub fn board_mut(&mut self) -> &mut [Cell] {
		match self.current_board {
			CurrentBoard::A => &mut self.state_a,
			CurrentBoard::B => &mut self.state_b,
		}
	}

	pub fn this_board_and_next(&mut self) -> (&mut [Cell], &mut [Cell]) {
		match self.current_board {
			CurrentBoard::A => (&mut self.state_a, &mut self.state_b),
			CurrentBoard::B => (&mut self.state_b, &mut self.state_a),
		}
	}

	pub fn randomize(&mut self) {
		for cell in self.board_mut() {
			*cell = cell.ruleset.random();
		}
	}

	pub fn clear(&mut self) {
		for cell in self.board_mut() {
			*cell = cell.ruleset.off();
		}
	}

	pub fn reset(&mut self) {
		let blank_cell = Ruleset::default().off();
		for cell in &mut self.state_a {
			*cell = blank_cell;
		}
		for cell in &mut self.state_b {
			*cell = blank_cell;
		}
	}

	pub fn generate(&mut self) {
		let (board, next_board) = self.this_board_and_next();
		// next_board
		// 	.par_chunks_exact_mut(BOARD_WIDTH)
		// 	.enumerate()
		// 	.for_each(|(row, next_chunk)| {
		// 		for col in 0..BOARD_WIDTH {
		// 			let idx = row * BOARD_WIDTH + col;
		// 			let cell = board[idx];
		// 			let next_cell = cell.ruleset.next_cell_state(board, row, col);
		// 			next_chunk[col].state = next_cell.state;
		// 		}
		// 	});

		for row in 0..BOARD_HEIGHT {
			for col in 0..BOARD_WIDTH {
				let idx = row * BOARD_WIDTH + col;
				let cell = board[idx];
				let next_cell = cell.ruleset.next_cell_state(board, row, col);
				next_board[idx].state = next_cell.state;
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
