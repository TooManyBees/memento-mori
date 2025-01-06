use crate::world::Cell;
use nannou::prelude::*;
use nannou::wgpu;

struct BufferData {
	vertices: [Vertex; 6],
	instances: Vec<Vertex>,
	colors: Vec<Color>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex(f32, f32);

#[repr(C, align(16))]
#[derive(Clone, Copy)]
struct Color(f32, f32, f32);

#[repr(C)]
#[derive(Copy, Clone)]
struct Uniforms {
	rows: u32,
	cols: u32,
}

fn make_buffer_data(width: usize, height: usize) -> BufferData {
	let vertices = [
		Vertex(0.0, 0.0),
		Vertex(1.0, 0.0),
		Vertex(1.0, 1.0),
		Vertex(1.0, 1.0),
		Vertex(0.0, 1.0),
		Vertex(0.0, 0.0),
	];

	let mut instances: Vec<Vertex> = Vec::with_capacity(width * height);
	let mut colors: Vec<Color> = Vec::with_capacity(width * height);
	for row in 0..height {
		for col in 0..width {
			instances.push(Vertex(col as f32, row as f32));
			colors.push(Color(
				col as f32 / width as f32,
				row as f32 / height as f32,
				0.5,
			));
		}
	}

	BufferData {
		vertices,
		instances,
		colors,
	}
}

pub struct Graphics {
	pub vertex_buffer: wgpu::Buffer,
	pub vertex_count: u32,
	pub instance_count: u32,
	pub color_buffer: wgpu::Buffer,
	pub bind_group: wgpu::BindGroup,
	pub render_pipeline: wgpu::RenderPipeline,
}

pub fn make_graphics(app: &App, width: usize, height: usize) -> Graphics {
	let buffer_data = make_buffer_data(width, height);
	let vertex_bytes = unsafe { wgpu::bytes::from_slice(&buffer_data.vertices) };
	let instance_bytes = unsafe { wgpu::bytes::from_slice(&buffer_data.instances) };
	let color_bytes = unsafe { wgpu::bytes::from_slice(&buffer_data.colors) };
	let uniforms = Uniforms {
		rows: height as u32,
		cols: width as u32,
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
		.buffer::<Vertex>(&instance_buffer, 0..buffer_data.instances.len())
		.buffer::<Color>(&color_buffer, 0..buffer_data.colors.len())
		.build(device, &bind_group_layout);
	let pipeline_layout = wgpu::create_pipeline_layout(device, None, &[&bind_group_layout], &[]); // TODO what??
	let render_pipeline = wgpu::RenderPipelineBuilder::from_layout(&pipeline_layout, &shader_mod)
		.fragment_shader(&shader_mod)
		.vertex_entry_point("vert_main")
		.fragment_entry_point("frag_main")
		.color_format(Frame::TEXTURE_FORMAT)
		.add_vertex_buffer::<Vertex>(&wgpu::vertex_attr_array![0 => Float32x2])
		.sample_count(4) // FIXME I hate this
		.build(device);

	Graphics {
		vertex_buffer,
		vertex_count: buffer_data.vertices.len() as u32,
		instance_count: buffer_data.instances.len() as u32,
		color_buffer,
		bind_group,
		render_pipeline,
	}
}

pub fn render_graphics(frame: &Frame, graphics: &Graphics, board: &[Cell]) {
	let cell_colors: Vec<_> = board
		.iter()
		.map(|cell| {
			let color = cell.ruleset.color(*cell);
			Color(
				color.red as f32 / 255f32,
				color.green as f32 / 255f32,
				color.blue as f32 / 255f32,
			)
		})
		.collect();
	let colors_bytes = unsafe { wgpu::bytes::from_slice(&cell_colors) };

	frame
		.device_queue_pair()
		.queue()
		.write_buffer(&graphics.color_buffer, 0, &colors_bytes);

	let mut encoder = frame.command_encoder();
	let mut render_pass = wgpu::RenderPassBuilder::new()
		.color_attachment(frame.texture_view(), |color| color)
		.begin(&mut encoder);

	render_pass.set_pipeline(&graphics.render_pipeline);
	render_pass.set_bind_group(0, &graphics.bind_group, &[]);
	render_pass.set_vertex_buffer(0, graphics.vertex_buffer.slice(..));
	render_pass.draw(0..graphics.vertex_count, 0..graphics.instance_count);
}
