use cgmath::Quaternion;
use crate::types::Vertex;

#[derive(Clone)]
pub struct Structure {
    pub vertexes: Vec<Vertex>,
    pub location: [f32; 3],
    pub rotation: Quaternion<f32>,
}

pub fn create_map() -> Structure {
    Structure {
        vertexes: vec![
            Vertex::new([-0.9, 0.3, -0.9]),
            Vertex::new([-0.9, -0.3, -0.9]),
            Vertex::new([-0.9, -0.3, 0.9]),
            Vertex::new([0.9, -0.3, 0.9]),
            Vertex::new([0.9, -0.3, -0.9]),
            Vertex::new([0.9, 0.3, -0.9]),
            Vertex::new([0.9, 0.3, 0.9]),
            Vertex::new([-0.9, 0.3, 0.9]),
        ],
        location: [0.0, 0.0, 0.0],
        rotation: cgmath::Quaternion::new(0.0, 0.0, 0.0, 0.0),
    }
}