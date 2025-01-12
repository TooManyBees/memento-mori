mod graphics;
mod rules;
mod world;

use crate::graphics::{make_graphics, render_graphics, Graphics};
use crate::rules::Ruleset;
use crate::world::{World, BOARD_HEIGHT, BOARD_WIDTH};
use nannou::prelude::*;
use std::time::{Duration, Instant};

const CELL_SIZE: usize = 4;
const GENERATION_RATE: Duration = Duration::from_millis(1000 / 15);

fn main() {
	nannou::app(model)
		.loop_mode(LoopMode::RefreshSync)
		.event(event)
		.update(update)
		.view(view)
		.run();
}

#[derive(Default)]
struct ColRow {
	col: usize,
	row: usize,
}

#[derive(Default)]
struct Brush {
	size: f32,
	pos: Vec2,
	ruleset: Ruleset,
	col_row: ColRow,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum AnimationState {
	Running,
	Paused,
	AdvanceFrame,
}

impl AnimationState {
	fn toggle(self) -> Self {
		match self {
			AnimationState::Running => AnimationState::Paused,
			_ => AnimationState::Running,
		}
	}

	fn frame_step(self) -> Self {
		match self {
			AnimationState::Running => AnimationState::Paused,
			_ => AnimationState::AdvanceFrame,
		}
	}

	fn next(self) -> Self {
		match self {
			AnimationState::Running => AnimationState::Running,
			_ => AnimationState::Paused,
		}
	}
}

struct Model {
	world: World,
	brush: Brush,
	draw_brush: bool,
	graphics: Graphics,
	animation_state: AnimationState,
	last_generation_at: Instant,
}

impl Model {
	fn is_running(&self) -> bool {
		match self.animation_state {
			AnimationState::Running => true,
			AnimationState::AdvanceFrame => true,
			AnimationState::Paused => false,
		}
	}
}

fn model(app: &App) -> Model {
	app.new_window()
		.size(
			BOARD_WIDTH as u32 * CELL_SIZE as u32,
			BOARD_HEIGHT as u32 * CELL_SIZE as u32,
		)
		.msaa_samples(1)
		.build()
		.unwrap();

	app.set_exit_on_escape(false);

	let graphics = make_graphics(app, BOARD_WIDTH, BOARD_HEIGHT);

	Model {
		world: World::new(),
		brush: Brush {
			size: 8.0,
			..Default::default()
		},
		draw_brush: false,
		graphics,
		animation_state: AnimationState::Running,
		last_generation_at: Instant::now() - GENERATION_RATE,
	}
}

fn event(app: &App, model: &mut Model, event: Event) {
	match event {
		Event::WindowEvent {
			simple: Some(window_event),
			..
		} => match window_event {
			WindowEvent::Focused => app.set_loop_mode(LoopMode::RefreshSync),
			WindowEvent::Unfocused => app.set_loop_mode(LoopMode::loop_ntimes(0)),
			WindowEvent::MouseEntered => model.draw_brush = true,
			WindowEvent::MouseExited => model.draw_brush = false,
			WindowEvent::MouseMoved(pos) => {
				model.brush.pos = pos;
				model.brush.col_row = get_cell_pos_under_pointer(pos);
			}
			// WindowEvent::MousePressed(MouseButton::Left) => println("Mouse pressed: Left"),
			// WindowEvent::MouseReleased(MouseButton::Left) => println("Mouse released: Left"),
			// WindowEvent::MousePressed(MouseButton::Right) => println!("Mouse pressed: Right"),
			// WindowEvent::MouseReleased(MouseButton::Right) => println!("Mouse released: Right"),
			// WindowEvent::MousePressed(MouseButton::Middle) => println!("Mouse pressed: Middle"),
			// WindowEvent::MouseReleased(MouseButton::Middle) => println!("Mouse released: Middle"),
			WindowEvent::KeyPressed(Key::Escape) => model.world.reset(),
			WindowEvent::KeyPressed(Key::C) => model.world.clear(),
			WindowEvent::KeyPressed(Key::R) => model.world.randomize(),
			WindowEvent::KeyPressed(Key::Tab) => model.brush.ruleset = model.brush.ruleset.next(),
			WindowEvent::KeyPressed(Key::Space) => {
				model.animation_state = model.animation_state.frame_step()
			}
			WindowEvent::KeyPressed(Key::Return) => {
				model.animation_state = model.animation_state.toggle()
			}
			WindowEvent::MouseWheel(MouseScrollDelta::LineDelta(_, delta), _) => {
				model.brush.size = (model.brush.size + delta).max(1.0).min(16.0)
			}
			_ => {}
		},
		_ => {}
	}
}

fn get_cell_pos_under_pointer(pos: Vec2) -> ColRow {
	const WINDOW_WIDTH: f32 = BOARD_WIDTH as f32 * CELL_SIZE as f32;
	const WINDOW_HEIGHT: f32 = BOARD_HEIGHT as f32 * CELL_SIZE as f32;
	let brush_px_y = (pos.y - WINDOW_HEIGHT * 0.5) * -1.0;
	let brush_px_x = pos.x + WINDOW_WIDTH * 0.5;
	let brush_row = (brush_px_y / CELL_SIZE as f32).floor().max(0.0) as usize;
	let brush_col = (brush_px_x / CELL_SIZE as f32).floor().max(0.0) as usize;

	// You would think you wouldn't have to clamp the row and col to the board
	// dimensions because when the mouse is off the board it doesn't produce
	// mouse events and you can't click on it, but you'd be wrong, because when
	// you click and drag you can maintain focus on the window while pulling
	// the pointer out of the window.
	ColRow {
		col: brush_col.max(0).min(BOARD_WIDTH - 1),
		row: brush_row.max(0).min(BOARD_HEIGHT - 1),
	}
}

fn paint(model: &mut Model, f: fn(&mut World, &Brush, usize)) {
	const WINDOW_WIDTH: f32 = BOARD_WIDTH as f32 * CELL_SIZE as f32;
	const WINDOW_HEIGHT: f32 = BOARD_HEIGHT as f32 * CELL_SIZE as f32;
	let brush_px_y = (model.brush.pos.y - WINDOW_HEIGHT * 0.5) * -1.0;
	let brush_px_x = model.brush.pos.x + WINDOW_WIDTH * 0.5;
	let brush_row = (brush_px_y / CELL_SIZE as f32).floor().max(0.0) as usize;
	let brush_col = (brush_px_x / CELL_SIZE as f32).floor().max(0.0) as usize;

	let min_row = if brush_row > model.brush.size as usize {
		brush_row - model.brush.size as usize
	} else {
		0
	};
	let max_row = (brush_row + model.brush.size as usize).min(BOARD_HEIGHT - 1);

	let min_col = if brush_col > model.brush.size as usize {
		brush_col - model.brush.size as usize
	} else {
		0
	};
	let max_col = (brush_col + model.brush.size as usize).min(BOARD_WIDTH - 1);

	for check_row in min_row..=max_row {
		let check_px_y = (check_row as f32 + 0.5) * CELL_SIZE as f32;
		for check_col in min_col..=max_col {
			let check_px_x = (check_col as f32 + 0.5) * CELL_SIZE as f32;
			let inside = (check_px_x - brush_px_x).pow(2) + (check_px_y - brush_px_y).pow(2)
				< (model.brush.size * CELL_SIZE as f32).pow(2);
			if inside {
				let idx = check_row * BOARD_WIDTH + check_col;
				f(&mut model.world, &model.brush, idx);
			}
		}
	}
}

fn update(app: &App, model: &mut Model, _update: Update) {
	let advance_simulation =
		model.is_running() && model.last_generation_at.elapsed() >= GENERATION_RATE;

	fn paint_liveness(world: &mut World, brush: &Brush, idx: usize) {
		let board = world.board_mut();
		let brush_idx = brush.col_row.row * BOARD_WIDTH + brush.col_row.col;
		let on = board[brush_idx].ruleset.on();
		if board[idx].ruleset == board[brush_idx].ruleset {
			board[idx] = on;
		}
	}

	fn paint_ruleset(world: &mut World, brush: &Brush, idx: usize) {
		let (board, next_board) = world.this_board_and_next();
		board[idx].ruleset = brush.ruleset;
		next_board[idx].ruleset = brush.ruleset;
	}

	if app.mouse.buttons.left().is_down() {
		paint(model, paint_liveness);
	} else if app.mouse.buttons.right().is_down() {
		paint(model, paint_ruleset);
	}

	if advance_simulation {
		model.world.generate();
		model.world.swap();
		model.last_generation_at = Instant::now();
		model.animation_state = model.animation_state.next();
	}
}

fn view(app: &App, model: &Model, frame: Frame) {
	let board = model.world.board();

	render_graphics(&frame, &model.graphics, board, app.keys.mods.ctrl());

	if model.draw_brush {
		let draw = app.draw();
		draw.ellipse()
			.radius(model.brush.size * CELL_SIZE as f32)
			.xy(model.brush.pos)
			.stroke_weight(2.0)
			.stroke(RED)
			.no_fill();

		let wr = app.main_window().rect();
		if wr.contains(model.brush.pos) {
			let ColRow { col, row } = model.brush.col_row;
			let brush_idx = row * BOARD_WIDTH + col;
			if brush_idx < BOARD_WIDTH * BOARD_HEIGHT {
				let cell = board[brush_idx];
				let wr = wr.pad(20.0);

				{
					let text = cell.ruleset.write_debug(cell);
					let text_width = (text.len() * 6) as f32;
					draw.rect()
						.color(BLACK)
						.x_y(wr.left() + text_width * 0.5, wr.bottom())
						.w_h(text_width, 20.0);
					draw.text(&text)
						.x_y(wr.left() + text_width * 0.5, wr.bottom());
				}

				{
					let text = format!("Painting {:?}", model.brush.ruleset);
					let text_width = (text.len() * 6) as f32;
					draw.rect()
						.color(BLACK)
						.x_y(wr.right() - text_width * 0.5, wr.bottom())
						.w_h(text_width, 20.0);
					draw.text(&text)
						.x_y(wr.right() - text_width * 0.5, wr.bottom());
				}
			}
		}
		draw.to_frame(app, &frame).unwrap();
	}
}
