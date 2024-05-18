
pub struct Shape {
	pub index_buffer_index: u64,
    pub vertex_buffer_index: usize,
    pub index_buffer_len: usize,
    pub vertex_buffer_len: usize,
    pub instance_buffer_index: usize,
    pub instance_buffer_len: usize,
}

pub struct DrawInstructions {
	pub shapes: Vec<Shape>,
}