pub struct BufferData {
	pub vertices: [Vertex; 6],
	pub instances: Vec<Vertex>,
	pub colors: Vec<Color>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex(f32, f32);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

pub fn make_mesh_data(width: usize, height: usize) -> BufferData {
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
			colors.push(Color(col as f32 / width as f32, row as f32 / height as f32, 0.5, 1.0));
		}
	}

	BufferData {
		vertices,
		instances,
		colors,
	}
}
