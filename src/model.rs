#[cfg(feature = "nite")]
mod oni_manager;
#[cfg(feature = "nite")]
pub use oni_manager::OniManager;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DrawUserState {
	Draw,
	PaintAndDisappear,
	None,
}

impl DrawUserState {
	pub fn toggle(self) -> Self {
		match self {
			DrawUserState::Draw => DrawUserState::PaintAndDisappear,
			DrawUserState::PaintAndDisappear => DrawUserState::None,
			DrawUserState::None => DrawUserState::Draw,
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
	#[cfg(feature = "nite")]
	pub oni_manager: Option<OniManager>,
	pub draw_user_state: DrawUserState,
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
