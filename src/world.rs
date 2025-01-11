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

		let mut all_live_neighboring_rulests = Vec::with_capacity(9);
		let mut deduped_live_neighboring_rulesets = Vec::with_capacity(9);
		let mut possible_next_cells = Vec::with_capacity(9);
		let mut live_rulesets_by_population = Vec::with_capacity(9); // FIXME we don't need 9

		for row in 0..BOARD_HEIGHT {
			for col in 0..BOARD_WIDTH {
				adjacent_live_rulesets(
					&mut all_live_neighboring_rulests,
					board,
					row,
					col,
					BOARD_WIDTH,
					BOARD_HEIGHT,
				);
				deduped_live_neighboring_rulesets.clear();
				deduped_live_neighboring_rulesets.extend_from_slice(&all_live_neighboring_rulests);
				deduped_live_neighboring_rulesets.dedup();

				let idx = row * BOARD_WIDTH + col;

				let next_cell = if deduped_live_neighboring_rulesets.len() > 1 {
					possible_next_cells.clear();

					for &ruleset in &deduped_live_neighboring_rulesets {
						let possible_next = ruleset.next_cell_state(board, row, col);
						if possible_next.state & 0b01 > 0 {
							possible_next_cells.push(possible_next);
						}
					}

					sort_rulesets_by_population(
						&mut live_rulesets_by_population,
						&all_live_neighboring_rulests,
					);

					let next_state = live_rulesets_by_population
						.iter()
						.find_map(|(ruleset, _)| {
							possible_next_cells
								.iter()
								.find(|next_cell| next_cell.ruleset == *ruleset)
						})
						.copied()
						.unwrap_or_else(|| board[idx].ruleset.next_cell_state(board, row, col));

					if next_state.ruleset != board[idx].ruleset {
						board[idx].ruleset = next_state.ruleset;
					}

					next_state
				} else {
					board[idx].ruleset.next_cell_state(board, row, col)
				};
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

fn adjacent_live_rulesets(
	output: &mut Vec<Ruleset>,
	board: &[Cell],
	row: usize,
	col: usize,
	width: usize,
	height: usize,
) {
	output.clear();

	if row > 0 {
		adjacent_live_rulesets_row(output, board, row - 1, col, width);
	}

	adjacent_live_rulesets_row(output, board, row, col, width);

	if row < height - 1 {
		adjacent_live_rulesets_row(output, board, row + 1, col, width);
	}

	output.sort_unstable();
}

fn adjacent_live_rulesets_row(
	output: &mut Vec<Ruleset>,
	board: &[Cell],
	row: usize,
	col: usize,
	width: usize,
) {
	let idx = row * width + col;

	if col > 0 {
		if board[idx - 1].state & 0b01 > 0 {
			output.push(board[idx - 1].ruleset);
		}
	}

	if board[idx].state & 0b01 > 0 {
		output.push(board[idx].ruleset);
	}

	if col < width - 1 {
		if board[idx + 1].state & 0b01 > 0 {
			output.push(board[idx + 1].ruleset);
		}
	}
}

fn sort_rulesets_by_population(result: &mut Vec<(Ruleset, u8)>, rulesets: &[Ruleset]) {
	result.clear();
	for &ruleset in rulesets {
		if let Some((_, count)) = result.iter_mut().find(|r| ruleset == r.0) {
			*count += 1;
		} else {
			result.push((ruleset, 1));
		}
	}
	result.sort_unstable_by(|(_, count_a), (_, count_b)| count_b.cmp(count_a));
}

#[cfg(test)]
mod test {
	use super::{adjacent_live_rulesets, Cell, Ruleset};

	#[test]
	fn adjacent_live_rulesets_clusters_rulesets() {
		let board = [
			Ruleset::Life,
			Ruleset::Life,
			Ruleset::LatticeGas,
			Ruleset::BriansBrain,
			Ruleset::Diamoeba,
			Ruleset::Seeds,
			Ruleset::AntiLife,
			Ruleset::AntiLife,
			Ruleset::Seeds,
		]
		.into_iter()
		.map(|ruleset| Cell {
			ruleset,
			state: 0b01,
		})
		.collect::<Vec<_>>();

		let expected = vec![
			Ruleset::Life,
			Ruleset::Life,
			Ruleset::AntiLife,
			Ruleset::AntiLife,
			Ruleset::BriansBrain,
			Ruleset::Seeds,
			Ruleset::Seeds,
			Ruleset::Diamoeba,
			Ruleset::LatticeGas,
		];

		let mut result = Vec::with_capacity(9);
		adjacent_live_rulesets(&mut result, &board, 1, 1, 3, 3);
		assert_eq!(result, expected);
	}

	// #[test]
	// fn adjacent_live_rulesets_ignores_dead_cells() {
	// 	let board = vec![

	// 	];

	// 	let mut result = Vec::with_capacity(9);
	// 	adjacent_live_rulesets(result, &board);
	// 	assert_eq!(result, vec![]);
	// }
}
