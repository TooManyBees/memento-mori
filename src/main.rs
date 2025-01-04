mod rules;
mod world;

use crate::rules::Ruleset;
use crate::world::{Board, World, BOARD_HEIGHT, BOARD_WIDTH};
use nannou::prelude::*;

const CELL_SIZE: u32 = 16;

fn main() {
	nannou::app(model)
		.loop_mode(LoopMode::RefreshSync)
		.event(event)
		.update(update)
		.view(view)
		.run();
}

struct Brush {
	size: f32,
	pos: Vec2,
	ruleset: Ruleset,
}

struct Model {
	world: World,
	brush: Brush,
	draw_brush: bool,
	drawing: bool,
}

fn model(app: &App) -> Model {
	app.new_window()
		.size(
			BOARD_WIDTH as u32 * CELL_SIZE,
			BOARD_HEIGHT as u32 * CELL_SIZE,
		)
		.build()
		.unwrap();

	Model {
		world: World::new(),
		brush: Brush {
			size: 8.0,
			pos: Vec2::ZERO,
			// ruleset: Default::default(),
			ruleset: crate::rules::Ruleset::BriansBrain,
		},
		draw_brush: false,
		drawing: false,
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
			WindowEvent::MouseMoved(pos) => model.brush.pos = pos,
			WindowEvent::MousePressed(MouseButton::Left) => model.drawing = true,
			WindowEvent::MouseReleased(MouseButton::Left) => model.drawing = false,
			WindowEvent::MousePressed(MouseButton::Right) => println!("Mouse pressed: Right"),
			WindowEvent::MouseReleased(MouseButton::Right) => println!("Mouse released: Right"),
			WindowEvent::MousePressed(MouseButton::Middle) => println!("Mouse pressed: Middle"),
			WindowEvent::MouseReleased(MouseButton::Middle) => println!("Mouse released: Middle"),
			WindowEvent::KeyPressed(Key::R) => model.world.randomize(),
			WindowEvent::MouseWheel(MouseScrollDelta::LineDelta(_, delta), _) => {
				model.brush.size = (model.brush.size + delta).max(1.0).min(16.0)
			}
			_ => {}
		},
		_ => {}
	}
}

fn paint(model: &mut Model, f: fn(&mut Board, &Brush, Ruleset, usize)) {
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

	let next_board = model.world.next_board_mut();

	// If you're clicking the app, you are by definition clicking a valid cell,
	// so the following slice index will be safe. However, if you click and drag,
	// you are able to can keep the app running while you move the mouse outside
	// of the valid area, so we must still clamp the mouse's position.
	let valid_brush_row = brush_row.max(min_row).min(max_row);
	let valid_brush_col = brush_col.max(min_col).min(max_col);
	let brush_ruleset = next_board[valid_brush_row * BOARD_WIDTH + valid_brush_col].ruleset;

	for check_row in min_row..=max_row {
		let check_px_y = (check_row as f32 + 0.5) * CELL_SIZE as f32;
		for check_col in min_col..=max_col {
			let check_px_x = (check_col as f32 + 0.5) * CELL_SIZE as f32;
			let inside = (check_px_x - brush_px_x).pow(2) + (check_px_y - brush_px_y).pow(2)
				< (model.brush.size * CELL_SIZE as f32).pow(2);
			if inside {
				let idx = check_row * BOARD_WIDTH + check_col;
				f(next_board, &model.brush, brush_ruleset, idx);
			}
		}
	}
}

fn update(app: &App, model: &mut Model, _update: Update) {
	model.world.generate();

	if model.drawing {
		fn paint_liveness(board: &mut Board, _brush: &Brush, _ruleset: Ruleset, idx: usize) {
			board[idx] = board[idx].ruleset.on();
		}

		fn paint_ruleset(board: &mut Board, brush: &Brush, _ruleset: Ruleset, idx: usize) {
			board[idx].ruleset = brush.ruleset;
		}

		if app.keys.mods.ctrl() {
			paint(model, paint_ruleset);
		} else {
			paint(model, paint_liveness);
		}
	}

	model.world.swap();
}

fn view(app: &App, model: &Model, frame: Frame) {
	frame.clear(BLACK);

	// Turn cartesian coordinates into graphics coordinates
	let mut draw = app
		.draw()
		.x_y(
			(BOARD_WIDTH * CELL_SIZE as usize) as f32 * -0.5,
			(BOARD_HEIGHT * CELL_SIZE as usize) as f32 * 0.5,
		)
		.scale_y(-1.0);

	let board = model.world.board();
	for (i, cell) in board.iter().enumerate() {
		let row = i / BOARD_WIDTH;
		let col = i % BOARD_WIDTH;

		if let Some(color) = cell.ruleset.color(*cell) {
			draw.rect()
				.width(CELL_SIZE as f32)
				.height(CELL_SIZE as f32)
				.x_y(
					(col * CELL_SIZE as usize) as f32 + 0.5 * CELL_SIZE as f32,
					(row * CELL_SIZE as usize) as f32 + 0.5 * CELL_SIZE as f32,
				)
				.color(color);
		}
	}

	if model.draw_brush {
		draw = draw.scale_y(-1.0).x_y(
			(BOARD_WIDTH * CELL_SIZE as usize) as f32 * 0.5,
			(BOARD_HEIGHT * CELL_SIZE as usize) as f32 * -0.5,
		);
		draw.ellipse()
			.radius(model.brush.size * CELL_SIZE as f32)
			.xy(model.brush.pos)
			.stroke_weight(2.0)
			.stroke(RED)
			.no_fill();
	}

	draw.to_frame(app, &frame).unwrap();
}
