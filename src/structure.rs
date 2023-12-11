use cgmath::Quaternion;
use crate::types::Vertex;

#[derive(Clone)]
pub struct Structure {
    pub vertexes: Vec<Vertex>,
    pub indexes: Vec<u16>,
    pub location: [f32; 3],
    pub rotation: Quaternion<f32>,
}

pub fn create_map() -> Structure {
    Structure {
        vertexes: vec![
            Vertex::new([-10.0, 3.0, -10.0]),
            Vertex::new([-10.0, -3.0, -10.0]),
            Vertex::new([-10.0, -3.0, 10.0]),
            Vertex::new([10.0, -3.0, 10.0]),
            Vertex::new([10.0, -3.0, -10.0]),
            Vertex::new([10.0, 3.0, -10.0]),
            Vertex::new([10.0, 3.0, 10.0]),
            Vertex::new([-10.0, 3.0, 10.0]),
        ],
        indexes: vec![
            0, 1, 2, 0, 2, 7, 7, 2, 3, 6, 7, 3, 6, 3, 4, 5, 6, 4,
            5, 4, 1, 0, 5, 1, 0, 5, 7, 6, 7, 5, 1, 4, 2, 3, 2, 4
        ],
        location: [0.0, 0.0, 0.0],
        rotation: cgmath::Quaternion::new(0.0, 0.0, 0.0, 0.0),
    }
}