
pub enum Indices {
	U16(Vec<u16>),
	U32(Vec<u32>)
}

#[derive(Debug)]
pub enum PrimitiveTopology {
	PointList,
	LineList,
	LineStrip,
	TriangleList,
	TriangleStrip
}

impl PrimitiveTopology {
	pub fn from_mode(mode: gltf::mesh::Mode) -> Self {
		match mode {
			gltf::mesh::Mode::Points => PrimitiveTopology::PointList,
			gltf::mesh::Mode::Lines => PrimitiveTopology::LineList,
			gltf::mesh::Mode::LineStrip => PrimitiveTopology::LineStrip,
			gltf::mesh::Mode::Triangles => PrimitiveTopology::TriangleList,
			gltf::mesh::Mode::TriangleStrip => PrimitiveTopology::TriangleStrip,
			_ => panic!("Invalid primitive topology")
		}
	}
}

#[derive(Debug)]
pub struct Mesh {
	pub topology: PrimitiveTopology,
	pub indices: Vec<u32>,
	pub positions: Vec<[f32; 3]>,
}

impl Mesh {
	pub fn new(topology: PrimitiveTopology) -> Self {
		Self {
			topology,
			indices: Vec::new(),
			positions: Vec::new(),
		}
	}
}