mod anti_life;
mod brians_brain;
mod diamoeba;
mod lattice_gas;
mod life;
mod seeds;

use crate::world::{Board, Cell};
pub use anti_life::AntiLife;
pub use brians_brain::BriansBrain;
pub use diamoeba::Diamoeba;
pub use lattice_gas::LatticeGas;
pub use life::Life;
use nannou::color::LinSrgba;
pub use seeds::Seeds;
use std::fmt::Write;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Ruleset {
	Life,
	AntiLife,
	BriansBrain,
	Seeds,
	Diamoeba,
	LatticeGas,
}

const VARIANTS: &[Ruleset] = &[
	Ruleset::Life,
	Ruleset::AntiLife,
	Ruleset::BriansBrain,
	Ruleset::Seeds,
	Ruleset::Diamoeba,
	Ruleset::LatticeGas,
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
			Ruleset::Diamoeba => Diamoeba::alive(),
			Ruleset::LatticeGas => LatticeGas::random_populated(),
		}
	}

	pub fn off(&self) -> Cell {
		match self {
			Ruleset::Life => Life::dead(),
			Ruleset::AntiLife => AntiLife::alive(),
			Ruleset::BriansBrain => BriansBrain::dead(),
			Ruleset::Seeds => Seeds::dead(),
			Ruleset::Diamoeba => Diamoeba::dead(),
			Ruleset::LatticeGas => LatticeGas::empty(),
		}
	}

	pub fn random(&self) -> Cell {
		match self {
			Ruleset::Life => Life::random(),
			Ruleset::AntiLife => AntiLife::random(),
			Ruleset::BriansBrain => BriansBrain::random(),
			Ruleset::Seeds => Seeds::random(),
			Ruleset::Diamoeba => Diamoeba::random(),
			Ruleset::LatticeGas => LatticeGas::random(),
		}
	}

	pub fn color(&self, cell: Cell) -> LinSrgba {
		match self {
			Ruleset::Life => Life::color(cell),
			Ruleset::AntiLife => AntiLife::color(cell),
			Ruleset::BriansBrain => BriansBrain::color(cell),
			Ruleset::Seeds => Seeds::color(cell),
			Ruleset::Diamoeba => Diamoeba::color(cell),
			Ruleset::LatticeGas => LatticeGas::color(cell),
		}
	}

	pub fn rule_color(&self) -> LinSrgba {
		match self {
			Ruleset::Life => LinSrgba::new(1.0, 0.0, 0.0, 0.125),
			Ruleset::AntiLife => LinSrgba::new(0.0, 1.0, 0.0, 0.125),
			Ruleset::BriansBrain => LinSrgba::new(0.0, 1.0, 1.0, 0.125),
			Ruleset::Seeds => LinSrgba::new(0.0, 1.0, 0.5, 0.125),
			Ruleset::Diamoeba => LinSrgba::new(0.0, 0.0, 1.0, 0.125),
			Ruleset::LatticeGas => LinSrgba::new(1.0, 1.0, 1.0, 0.125),
		}
	}

	pub fn next_cell_state(&self, board: &Board, row: usize, col: usize) -> Cell {
		match self {
			Ruleset::Life => Life::next_cell_state(board, row, col),
			Ruleset::AntiLife => AntiLife::next_cell_state(board, row, col),
			Ruleset::BriansBrain => BriansBrain::next_cell_state(board, row, col),
			Ruleset::Seeds => Seeds::next_cell_state(board, row, col),
			Ruleset::Diamoeba => Diamoeba::next_cell_state(board, row, col),
			Ruleset::LatticeGas => LatticeGas::next_cell_state(board, row, col),
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
			Ruleset::Diamoeba => Diamoeba::write_debug(&mut output, cell.state),
			Ruleset::LatticeGas => LatticeGas::write_debug(&mut output, cell.state),
		}
		.unwrap();
		write!(&mut output, ")").unwrap();
		output
	}

	pub fn next(&self) -> Ruleset {
		VARIANTS[(*self as u8 as usize + 1) % VARIANTS.len()]
	}
}
