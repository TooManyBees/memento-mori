mod graphics;
mod rules;
mod world;

use crate::graphics::{make_graphics, render_graphics, Graphics};
use crate::rules::Ruleset;
use crate::world::{World, BOARD_HEIGHT, BOARD_WIDTH};
use nannou::prelude::*;

const CELL_SIZE: usize = 4;

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

struct Model {
	world: World,
	brush: Brush,
	draw_brush: bool,
	drawing: bool,
	graphics: Graphics,
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

	let graphics = make_graphics(app, BOARD_WIDTH, BOARD_HEIGHT);

	Model {
		world: World::new(),
		brush: Brush {
			size: 8.0,
			..Default::default()
		},
		draw_brush: false,
		drawing: false,
		graphics,
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
			WindowEvent::MousePressed(MouseButton::Left) => model.drawing = true,
			WindowEvent::MouseReleased(MouseButton::Left) => model.drawing = false,
			WindowEvent::MousePressed(MouseButton::Right) => println!("Mouse pressed: Right"),
			WindowEvent::MouseReleased(MouseButton::Right) => println!("Mouse released: Right"),
			WindowEvent::MousePressed(MouseButton::Middle) => println!("Mouse pressed: Middle"),
			WindowEvent::MouseReleased(MouseButton::Middle) => println!("Mouse released: Middle"),
			WindowEvent::KeyPressed(Key::R) => model.world.randomize(),
			WindowEvent::KeyPressed(Key::Tab) => model.brush.ruleset = model.brush.ruleset.next(),
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

	ColRow { col: brush_col, row: brush_row }
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

	// // If you're clicking the app, you are by definition clicking a valid cell,
	// // so the following slice index will be safe. However, if you click and drag,
	// // you are able to can keep the app running while you move the mouse outside
	// // of the valid area, so we must still clamp the mouse's position.
	// let valid_brush_row = brush_row.max(min_row).min(max_row);
	// let valid_brush_col = brush_col.max(min_col).min(max_col);
	// let brush_ruleset = next_board[valid_brush_row * BOARD_WIDTH + valid_brush_col].ruleset;

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
	model.world.generate();

	if model.drawing {
		fn paint_liveness(world: &mut World, brush: &Brush, idx: usize) {
			let board = world.next_board_mut();
			let brush_idx = brush.col_row.row * BOARD_WIDTH + brush.col_row.col;
			if board[idx].ruleset == board[brush_idx].ruleset {
				board[idx] = board[idx].ruleset.on();
			}
		}

		fn paint_ruleset(world: &mut World, brush: &Brush, idx: usize) {
			let (board, next_board) = world.this_board_and_next();
			board[idx].ruleset = brush.ruleset;
			next_board[idx].ruleset = brush.ruleset;
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
	let board = model.world.board();

	render_graphics(&frame, &model.graphics, board);

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
				let text = format!("{:?}({:02b})", cell.ruleset, cell.state);
				let text_width = (text.len() * 6) as f32;
				let wr = wr.pad(20.0);
				draw.rect()
					.color(BLACK)
					.x_y(wr.left() + text_width * 0.5, wr.bottom())
					.w_h(text_width, 20.0);
				draw.text(&text)
					.x_y(wr.left() + text_width * 0.5, wr.bottom());
			}
		}
		draw.to_frame(app, &frame).unwrap();
	}
}
