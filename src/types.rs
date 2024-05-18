use cgmath::Quaternion;
use cgmath::Vector3;

use crate::buffer::Buffer;
use crate::buffer::Pointer;
use crate::instance::Instance;
use crate::matrix::Matrix4x4;

// #[derive(Debug)]
// pub struct ShapeDesc {
//     pub index_buffer_index: usize,
//     pub vertex_buffer_index: usize,
//     pub index_buffer_len: usize,
//     pub vertex_buffer_len: usize,
//     pub instance_buffer_index: usize,
//     pub instance_buffer_len: usize,
// }

// #[derive(Debug)]
// pub struct SerializedState {
//     pub index_buffer: Vec<u16>,
//     pub vertex_buffer: Vec<Vertex>,
//     pub instance_buffer: Vec<Instance>,
//     pub shapes: Vec<ShapeDesc>,
//     pub camera: CameraPos
// }

pub struct DrawInstruction {
	pub position_pointer: Pointer,
	pub instances: Pointer
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
	pub normal: [f32; 3],
	pub uv: [f32; 2]
}

impl Vertex {
    pub fn new(pos: [f32; 3]) -> Self {
        Self {
            pos,
			normal: [0.0, 0.0, 0.0],
			uv: [0.0, 0.0]
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }

	pub fn size() -> usize {
		std::mem::size_of::<Self>()
	}    
}

#[derive(Clone)]
pub enum GameState {
    Lobby,
    InGame,
    GameOver
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Lobby
    }
}

#[derive(Clone)]
pub struct Triangle {
    pub a_len: f32,
    pub b_len: f32,
    pub c_len: f32,
    pub a_thick: f32,
    pub b_thick: f32,
    pub c_thick: f32,

    // pub material: Material,
    pub rotation: Quaternion<f32>,
    pub position: Vector3<f32>,
}

pub struct Shape {
    pub vertexes: Vec<Vertex>,
    pub indexes: Vec<u16>,
}

// #[derive(Clone)]
// pub enum Material {
//     Wood,
//     Stone
// }


#[derive(Clone)]
pub struct Player {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(0.0, 0.0, 0.0, 0.0)
        }
    }
}

#[derive(Default, Clone)]
pub struct PlayerState {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub primary: bool,
    pub secondary: bool,
    pub third: bool,
}

#[derive(Debug, Clone)]
pub struct Entity {
	pub mesh: usize,
	pub loc: [f32; 3],
	pub rot: Quaternion<f32>,
}

pub enum Indices {
	U16(Vec<u16>),
	U32(Vec<u32>)
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Mesh {
	pub topology: PrimitiveTopology,
	pub indices: Vec<u32>,
	pub vertices: Vec<Vertex>,
	pub normals: Vec<[f32; 3]>,
}

impl Mesh {
	pub fn new(topology: PrimitiveTopology) -> Self {
		Self {
			topology,
			indices: Vec::new(),
			vertices: Vec::new(),
			normals: Vec::new(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Node {
	pub name: String,
	pub meshes: Vec<Mesh>,
	pub children: Vec<Node>,
}

impl Node {
	pub fn new(name: String) -> Self {
		Self {
			name,
			meshes: Vec::new(),
			children: Vec::new(),
		}
	}
}



pub struct Material {
	pub base_color: [f32; 4],

}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct MaterialUniform {
    pub base_color: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
    pub emissive: [f32; 3],
    pub has_base_color_texture: u32, // 1 if texture is used, otherwise 0
    pub _padding: u32,
}