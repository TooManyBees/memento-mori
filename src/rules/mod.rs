mod anti_life;
mod brians_brain;
mod life;

use crate::world::Cell;
pub use anti_life::AntiLife;
pub use brians_brain::BriansBrain;
pub use life::Life;
use nannou::color::{encoding::Srgb, rgb::Rgb};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Ruleset {
	Life,
	AntiLife,
	BriansBrain,
}

impl Default for Ruleset {
	fn default() -> Ruleset {
		Ruleset::Life
	}
}

impl Ruleset {
	pub fn on(&self) -> Cell {
		match self {
			Ruleset::Life => Life::alive(),
			Ruleset::AntiLife => AntiLife::dead(),
			Ruleset::BriansBrain => BriansBrain::firing(),
		}
	}

	pub fn random(&self) -> Cell {
		match self {
			Ruleset::Life => Life::random(),
			Ruleset::AntiLife => AntiLife::random(),
			Ruleset::BriansBrain => BriansBrain::random(),
		}
	}

	pub fn color(&self, cell: Cell) -> Rgb<Srgb, u8> {
		match self {
			Ruleset::Life => Life::color(cell),
			Ruleset::AntiLife => AntiLife::color(cell),
			Ruleset::BriansBrain => BriansBrain::color(cell),
		}
	}

	pub fn next_cell_state(&self, board: &[Cell], row: usize, col: usize) -> Cell {
		match self {
			Ruleset::Life => Life::next_cell_state(board, row, col),
			Ruleset::AntiLife => AntiLife::next_cell_state(board, row, col),
			Ruleset::BriansBrain => BriansBrain::next_cell_state(board, row, col),
		}
	}
}
