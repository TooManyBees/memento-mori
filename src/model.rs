use crate::graphics::Graphics;
use crate::rules::Ruleset;
use crate::world::World;
use nannou::prelude::*;
use std::time::Instant;

#[derive(Default)]
pub struct ColRow {
	pub col: usize,
	pub row: usize,
}

#[derive(Default)]
pub struct Brush {
	pub size: u32,
	pub pos: Vec2,
	pub ruleset: Ruleset,
	pub col_row: ColRow,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AnimationState {
	Running,
	Paused,
	AdvanceFrame,
}

impl AnimationState {
	pub fn toggle(self) -> Self {
		match self {
			AnimationState::Running => AnimationState::Paused,
			_ => AnimationState::Running,
		}
	}

	pub fn frame_step(self) -> Self {
		match self {
			AnimationState::Running => AnimationState::Paused,
			_ => AnimationState::AdvanceFrame,
		}
	}

	pub fn next(self) -> Self {
		match self {
			AnimationState::Running => AnimationState::Running,
			_ => AnimationState::Paused,
		}
	}
}

pub struct Model {
	pub world: World,
	pub brush: Brush,
	pub draw_brush: bool,
	pub graphics: Graphics,
	pub animation_state: AnimationState,
	pub last_generation_at: Instant,
}

impl Model {
	pub fn is_running(&self) -> bool {
		match self.animation_state {
			AnimationState::Running => true,
			AnimationState::AdvanceFrame => true,
			AnimationState::Paused => false,
		}
	}
}
