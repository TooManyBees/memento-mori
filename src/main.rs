mod mesh;
mod rules;
mod world;

use crate::mesh::make_mesh_data;
use crate::rules::Ruleset;
use crate::world::{World, BOARD_HEIGHT, BOARD_WIDTH};
use nannou::prelude::*;
use nannou::wgpu;

const CELL_SIZE: usize = 16;

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
	col_row: (usize, usize),
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Uniforms {
	rows: u32,
	cols: u32,
}

struct Graphics {
	vertex_buffer: wgpu::Buffer,
	vertex_count: u32,
	instance_count: u32,
	color_buffer: wgpu::Buffer,
	bind_group: wgpu::BindGroup,
	render_pipeline: wgpu::RenderPipeline,
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
		.build()
		.unwrap();

	let mesh_data = make_mesh_data(BOARD_WIDTH, BOARD_HEIGHT);
	let vertex_bytes = unsafe { wgpu::bytes::from_slice(&mesh_data.vertices) };
	let instance_bytes = unsafe { wgpu::bytes::from_slice(&mesh_data.instances) };
	let color_bytes = unsafe { wgpu::bytes::from_slice(&mesh_data.colors) };
	let uniforms = Uniforms {
		rows: BOARD_HEIGHT as u32,
		cols: BOARD_WIDTH as u32,
	};
	let uniforms_bytes = unsafe { wgpu::bytes::from(&uniforms) };

	let window = app.main_window();
	let device = window.device();
	let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
		label: Some("Vertex buffer"),
		contents: vertex_bytes,
		usage: wgpu::BufferUsages::VERTEX,
	});
	let instance_buffer = device.create_buffer_init(&BufferInitDescriptor {
		label: Some("Instance position buffer"),
		contents: instance_bytes,
		usage: wgpu::BufferUsages::STORAGE,
	});
	let color_buffer = device.create_buffer_init(&BufferInitDescriptor {
		label: Some("Instance color buffer"),
		contents: color_bytes,
		usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
	});
	let uniforms_buffer = device.create_buffer_init(&BufferInitDescriptor {
		label: Some("Uniform buffer"),
		contents: uniforms_bytes,
		usage: wgpu::BufferUsages::UNIFORM,
	});

	let shader_desc = wgpu::include_wgsl!("shaders/shader.wgsl");
	let shader_mod = device.create_shader_module(shader_desc);

	let bind_group_layout = wgpu::BindGroupLayoutBuilder::new()
		.uniform_buffer(wgpu::ShaderStages::VERTEX, false)
		.storage_buffer(wgpu::ShaderStages::VERTEX, false, true)
		.storage_buffer(wgpu::ShaderStages::VERTEX, false, true)
		.build(device);
	let bind_group = wgpu::BindGroupBuilder::new()
		.buffer::<Uniforms>(&uniforms_buffer, 0..1)
		.buffer::<mesh::Vertex>(&instance_buffer, 0..mesh_data.instances.len())
		.buffer::<mesh::Color>(&color_buffer, 0..mesh_data.colors.len())
		.build(device, &bind_group_layout);
	let pipeline_layout = wgpu::create_pipeline_layout(device, None, &[&bind_group_layout], &[]); // TODO what??
	let render_pipeline = wgpu::RenderPipelineBuilder::from_layout(&pipeline_layout, &shader_mod)
		.fragment_shader(&shader_mod)
		.vertex_entry_point("vert_main")
		.fragment_entry_point("frag_main")
		.color_format(Frame::TEXTURE_FORMAT)
		.add_vertex_buffer::<mesh::Vertex>(&wgpu::vertex_attr_array![0 => Float32x2])
		.sample_count(4) // FIXME I hate this
		.build(device);

	let graphics = Graphics {
		vertex_buffer,
		vertex_count: mesh_data.vertices.len() as u32,
		instance_count: mesh_data.instances.len() as u32,
		color_buffer,
		bind_group,
		render_pipeline,
	};

	Model {
		world: World::new(),
		brush: Brush {
			size: 8.0,
			pos: Vec2::ZERO,
			// ruleset: Default::default(),
			ruleset: crate::rules::Ruleset::BriansBrain,
			col_row: (0, 0),
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
			WindowEvent::MouseWheel(MouseScrollDelta::LineDelta(_, delta), _) => {
				model.brush.size = (model.brush.size + delta).max(1.0).min(16.0)
			}
			_ => {}
		},
		_ => {}
	}
}

fn get_cell_pos_under_pointer(pos: Vec2) -> (usize, usize) {
	const WINDOW_WIDTH: f32 = BOARD_WIDTH as f32 * CELL_SIZE as f32;
	const WINDOW_HEIGHT: f32 = BOARD_HEIGHT as f32 * CELL_SIZE as f32;
	let brush_px_y = (pos.y - WINDOW_HEIGHT * 0.5) * -1.0;
	let brush_px_x = pos.x + WINDOW_WIDTH * 0.5;
	let brush_row = (brush_px_y / CELL_SIZE as f32).floor().max(0.0) as usize;
	let brush_col = (brush_px_x / CELL_SIZE as f32).floor().max(0.0) as usize;

	(brush_col, brush_row)
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
		fn paint_liveness(world: &mut World, _brush: &Brush, idx: usize) {
			let board = world.next_board_mut();
			board[idx] = board[idx].ruleset.on();
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
	let cell_colors: Vec<_> = board
		.iter()
		.map(|cell| {
			let color = cell.ruleset.color(*cell);
			mesh::Color(
				color.red as f32 / 255f32,
				color.green as f32 / 255f32,
				color.blue as f32 / 255f32,
				1.0,
			)
		})
		.collect();
	let colors_bytes = unsafe { wgpu::bytes::from_slice(&cell_colors) };

	frame
		.device_queue_pair()
		.queue()
		.write_buffer(&model.graphics.color_buffer, 0, &colors_bytes);

	// encoder must be dropped before the pointer draw calls to avoid
	// a double borrow runtime panic
	{
		let mut encoder = frame.command_encoder();
		let mut render_pass = wgpu::RenderPassBuilder::new()
			.color_attachment(frame.texture_view(), |color| color)
			.begin(&mut encoder);

		render_pass.set_pipeline(&model.graphics.render_pipeline);
		render_pass.set_bind_group(0, &model.graphics.bind_group, &[]);
		render_pass.set_vertex_buffer(0, model.graphics.vertex_buffer.slice(..));
		render_pass.draw(
			0..model.graphics.vertex_count,
			0..model.graphics.instance_count,
		);
	}


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
			let (col, row) = model.brush.col_row;
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
