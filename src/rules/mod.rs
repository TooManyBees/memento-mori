mod anti_life;
mod brians_brain;
mod life;
mod seeds;

use crate::world::Cell;
pub use anti_life::AntiLife;
pub use brians_brain::BriansBrain;
pub use life::Life;
use nannou::color::{encoding::Srgb, rgb::Rgb};
pub use seeds::Seeds;
use std::fmt::Write;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Ruleset {
	Life,
	AntiLife,
	BriansBrain,
	Seeds,
}

const VARIANTS: &[Ruleset] = &[
	Ruleset::Life,
	Ruleset::AntiLife,
	Ruleset::BriansBrain,
	Ruleset::Seeds,
];

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
			Ruleset::Seeds => Seeds::alive(),
		}
	}

	pub fn random(&self) -> Cell {
		match self {
			Ruleset::Life => Life::random(),
			Ruleset::AntiLife => AntiLife::random(),
			Ruleset::BriansBrain => BriansBrain::random(),
			Ruleset::Seeds => Seeds::random(),
		}
	}

	pub fn color(&self, cell: Cell) -> Rgb<Srgb, u8> {
		match self {
			Ruleset::Life => Life::color(cell),
			Ruleset::AntiLife => AntiLife::color(cell),
			Ruleset::BriansBrain => BriansBrain::color(cell),
			Ruleset::Seeds => Seeds::color(cell),
		}
	}

	pub fn next_cell_state(&self, board: &[Cell], row: usize, col: usize) -> Cell {
		match self {
			Ruleset::Life => Life::next_cell_state(board, row, col),
			Ruleset::AntiLife => AntiLife::next_cell_state(board, row, col),
			Ruleset::BriansBrain => BriansBrain::next_cell_state(board, row, col),
			Ruleset::Seeds => Seeds::next_cell_state(board, row, col),
		}
	}

	pub fn write_debug(&self, cell: Cell) -> String {
		let mut output = String::new();
		write!(&mut output, "{:?}(", cell.ruleset).unwrap();
		match self {
			Ruleset::Life => Life::write_debug(&mut output, cell.state),
			Ruleset::AntiLife => AntiLife::write_debug(&mut output, cell.state),
			Ruleset::BriansBrain => BriansBrain::write_debug(&mut output, cell.state),
			Ruleset::Seeds => Seeds::write_debug(&mut output, cell.state),
		}.unwrap();
		write!(&mut output, ")").unwrap();
		output
	}

	pub fn next(&self) -> Ruleset {
		VARIANTS[(*self as u8 as usize + 1) % VARIANTS.len()]
	}
}
