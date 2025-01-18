use crate::rules::Ruleset;

#[derive(Copy, Clone, Default, Debug)]
pub struct Cell {
	pub ruleset: Ruleset,
	pub state: u8,
}

#[derive(Debug)]
struct Growth {
	all_live_neighboring_rulests: Vec<Ruleset>,
	deduped_live_neighboring_rulesets: Vec<Ruleset>,
	possible_next_cells: Vec<Cell>,
	live_rulesets_by_population: Vec<(Ruleset, u8)>,
}

impl Default for Growth {
	fn default() -> Self {
		Growth {
			all_live_neighboring_rulests: Vec::with_capacity(9),
			deduped_live_neighboring_rulesets: Vec::with_capacity(9),
			possible_next_cells: Vec::with_capacity(9),
			live_rulesets_by_population: Vec::with_capacity(9),
		}
	}
}

impl Growth {
	fn find_neighboring_rulesets(&mut self, board: &Board, row: usize, col: usize) {
		adjacent_live_rulesets(&mut self.all_live_neighboring_rulests, board, row, col);
		self.deduped_live_neighboring_rulesets.clear();
		self.deduped_live_neighboring_rulesets
			.extend_from_slice(&self.all_live_neighboring_rulests);
		self.deduped_live_neighboring_rulesets.dedup();

		if self.has_competing_rulesets() {
			sort_rulesets_by_population(
				&mut self.live_rulesets_by_population,
				&self.all_live_neighboring_rulests,
			);

			self.possible_next_cells.clear();

			for ruleset in &self.deduped_live_neighboring_rulesets {
				let possible_next = ruleset.next_cell_state(board, row, col);
				if possible_next.state & 0b01 > 0 {
					self.possible_next_cells.push(possible_next);
				}
			}
		}
	}

	fn has_competing_rulesets(&self) -> bool {
		self.deduped_live_neighboring_rulesets.len() > 1
	}

	fn next_live_state(&self) -> Option<Cell> {
		self.live_rulesets_by_population
			.iter()
			.find_map(|(ruleset, _)| {
				self.possible_next_cells
					.iter()
					.find(|next_cell| next_cell.ruleset == *ruleset)
			})
			.copied()
	}
}

#[derive(Debug, Clone)]
pub struct Board {
	pub cells: Vec<Cell>,
	pub width: usize,
	pub height: usize,
}

impl std::ops::Index<usize> for Board {
	type Output = Cell;
	fn index(&self, idx: usize) -> &Cell {
		&self.cells[idx]
	}
}

impl std::ops::IndexMut<usize> for Board {
	fn index_mut(&mut self, idx: usize) -> &mut Cell {
		&mut self.cells[idx]
	}
}

#[derive(Debug)]
pub struct World {
	state_a: Board,
	state_b: Board,
	growth: Growth,
	current_board: CurrentBoard,
	pub temporary_rulesets: Vec<Option<Ruleset>>,
	pub temporary_states: Vec<Option<u8>>,
}

impl World {
	pub fn new(width: usize, height: usize) -> Self {
		let state_a = Board {
			cells: vec![Cell::default(); width * height],
			width,
			height,
		};
		let state_b = state_a.clone();
		let temporary_rulesets = vec![None; width * height];
		let temporary_states = vec![None; width * height];

		World {
			state_a,
			state_b,
			growth: Default::default(),
			current_board: CurrentBoard::A,
			temporary_rulesets,
			temporary_states,
		}
	}

	pub fn board(&self) -> &Board {
		match self.current_board {
			CurrentBoard::A => &self.state_a,
			CurrentBoard::B => &self.state_b,
		}
	}

	pub fn board_mut(&mut self) -> &mut Board {
		match self.current_board {
			CurrentBoard::A => &mut self.state_a,
			CurrentBoard::B => &mut self.state_b,
		}
	}

	pub fn this_board_and_next_and_temporary(
		&mut self,
	) -> (&mut Board, &mut Board, &[Option<Ruleset>], &[Option<u8>]) {
		match self.current_board {
			CurrentBoard::A => (
				&mut self.state_a,
				&mut self.state_b,
				&self.temporary_rulesets,
				&self.temporary_states,
			),
			CurrentBoard::B => (
				&mut self.state_b,
				&mut self.state_a,
				&self.temporary_rulesets,
				&self.temporary_states,
			),
		}
	}

	pub fn this_board_and_next(&mut self) -> (&mut Board, &mut Board) {
		match self.current_board {
			CurrentBoard::A => (&mut self.state_a, &mut self.state_b),
			CurrentBoard::B => (&mut self.state_b, &mut self.state_a),
		}
	}

	fn boards_and_growth(
		&mut self,
	) -> (
		&mut Board,
		&mut Board,
		&[Option<Ruleset>],
		&[Option<u8>],
		&mut Growth,
	) {
		match self.current_board {
			CurrentBoard::A => (
				&mut self.state_a,
				&mut self.state_b,
				&self.temporary_rulesets,
				&self.temporary_states,
				&mut self.growth,
			),
			CurrentBoard::B => (
				&mut self.state_b,
				&mut self.state_a,
				&self.temporary_rulesets,
				&self.temporary_states,
				&mut self.growth,
			),
		}
	}

	pub fn randomize(&mut self) {
		for cell in &mut self.board_mut().cells {
			*cell = cell.ruleset.random();
		}
	}

	pub fn clear(&mut self) {
		for cell in &mut self.board_mut().cells {
			*cell = cell.ruleset.off();
		}
	}

	pub fn reset(&mut self) {
		let blank_cell = Ruleset::default().off();
		self.state_a.cells.fill(blank_cell);
		self.state_b.cells.fill(blank_cell);
		self.temporary_states.fill(None);
		self.temporary_rulesets.fill(None);
	}

	pub fn generate(&mut self, growth_enabled: bool) {
		let (board, next_board, temporary_rulesets, temporary_states, growth) =
			self.boards_and_growth();
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

		let scratch_cells = board
			.cells
			.iter()
			.zip(temporary_states)
			.zip(temporary_rulesets)
			.map(|((cell, maybe_state), maybe_ruleset)| {
				let state = maybe_state.unwrap_or(cell.state);
				let ruleset = maybe_ruleset.unwrap_or(cell.ruleset);
				Cell { ruleset, state }
			})
			.collect::<Vec<Cell>>();

		let scratch_board = Board {
			cells: scratch_cells,
			width: board.width,
			height: board.height,
		};

		for row in 0..board.height {
			for col in 0..board.width {
				if growth_enabled {
					growth.find_neighboring_rulesets(&scratch_board, row, col);
				}

				let idx = row * board.width + col;

				if let Some(ruleset) = temporary_rulesets[idx] {
					// If operating on a temporary ruleset, bypass growth. The shape of a person
					// shouldn't grow.
					let next_cell = ruleset.next_cell_state(&scratch_board, row, col);
					next_board[idx].state = next_cell.state;
				} else if growth_enabled && growth.has_competing_rulesets() {
					// If growth is enabled and there's more than 1 live ruleset around a cell,
					// compete for growth.
					let next_cell = growth.next_live_state().unwrap_or_else(|| {
						board[idx].ruleset.next_cell_state(&scratch_board, row, col)
					});

					if next_cell.ruleset != board[idx].ruleset {
						board[idx].ruleset = next_cell.ruleset;
					}

					next_board[idx] = next_cell;
				} else {
					// Otherwise there's no need to check for growth. Either it's disabled, or
					// the cell is surrounded by just 1 rule.
					let next_cell = board[idx].ruleset.next_cell_state(&scratch_board, row, col);
					next_board[idx].state = next_cell.state;
				}
				// debug_assert_eq!(next_board[idx].ruleset, board[idx].ruleset);
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

fn adjacent_live_rulesets(output: &mut Vec<Ruleset>, board: &Board, row: usize, col: usize) {
	output.clear();

	if row > 0 {
		adjacent_live_rulesets_row(output, board, row - 1, col);
	}

	adjacent_live_rulesets_row(output, board, row, col);

	if row < board.height - 1 {
		adjacent_live_rulesets_row(output, board, row + 1, col);
	}

	output.sort_unstable();
}

#[inline]
fn push_ruleset_if_live(output: &mut Vec<Ruleset>, board: &Board, idx: usize) {
	// Consider pushing ruleset if cell's state is nonzero, not only if cell's
	// smallest bit is on. This will make the growth algorithm consider the
	// rulesets of cells which aren't alive but have history in its upper bits,
	// like "refractory" in brian's brian, or "was alive" in anti-life.
	if board[idx].state & 0b01 > 0 {
		output.push(board[idx].ruleset);
	}
}

fn adjacent_live_rulesets_row(output: &mut Vec<Ruleset>, board: &Board, row: usize, col: usize) {
	let idx = row * board.width + col;

	if col > 0 {
		push_ruleset_if_live(output, board, idx - 1);
	}

	push_ruleset_if_live(output, board, idx);

	if col < board.width - 1 {
		push_ruleset_if_live(output, board, idx + 1);
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
	use super::{adjacent_live_rulesets, sort_rulesets_by_population, Board, Cell, Ruleset};

	#[test]
	fn adjacent_live_rulesets_clusters_rulesets() {
		let cells = [
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
		let board = Board {
			cells,
			width: 3,
			height: 3,
		};

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
		adjacent_live_rulesets(&mut result, &board, 1, 1);
		assert_eq!(result, expected);
	}

	#[test]
	fn adjacent_live_rulesets_ignores_dead_cells() {
		let cells = [
			(Ruleset::Life, 0b01),
			(Ruleset::Life, 0b00),
			(Ruleset::LatticeGas, 0b01),
			(Ruleset::BriansBrain, 0b11),
			(Ruleset::Diamoeba, 0b10),
			(Ruleset::Seeds, 0b01),
			(Ruleset::AntiLife, 0b10),
			(Ruleset::AntiLife, 0b00),
			(Ruleset::Seeds, 0b01),
		]
		.into_iter()
		.map(|(ruleset, state)| Cell { ruleset, state })
		.collect::<Vec<_>>();

		let board = Board {
			cells,
			width: 3,
			height: 3,
		};

		let expected = vec![
			Ruleset::Life,
			Ruleset::BriansBrain,
			Ruleset::Seeds,
			Ruleset::Seeds,
			Ruleset::LatticeGas,
		];

		let mut result = Vec::with_capacity(9);
		adjacent_live_rulesets(&mut result, &board, 1, 1);
		assert_eq!(result, expected);
	}

	#[test]
	fn sort_rulesets_by_population_sorts_em() {
		let neighboring_rulesets = vec![
			Ruleset::Life,
			Ruleset::Life,
			Ruleset::LatticeGas,
			Ruleset::BriansBrain,
			Ruleset::Diamoeba,
			Ruleset::AntiLife,
			Ruleset::AntiLife,
			Ruleset::AntiLife,
			Ruleset::Seeds,
		];

		let mut result: Vec<(Ruleset, u8)> = Vec::with_capacity(9);

		sort_rulesets_by_population(&mut result, &neighboring_rulesets);

		// Because of an unstable sort, all the rulesets with count=1 will
		// have an undefined order, and also we don't care what it is
		assert_eq!(result[0], (Ruleset::AntiLife, 3));
		assert_eq!(result[1], (Ruleset::Life, 2));
	}
}
