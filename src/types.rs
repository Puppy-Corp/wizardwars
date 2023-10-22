use cgmath::Quaternion;
use cgmath::Vector3;


pub struct GameDiff {

}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3]
}

impl Vertex {
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
}

pub enum GameState {
    Lobby,
    InGame,
    GameOver
}

pub enum Shape {

}

pub struct Structure {
    material: Material,
    rotation: Quaternion<f32>,

}

pub enum Material {
    Wood,
    Stone
}


pub struct Player {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
}