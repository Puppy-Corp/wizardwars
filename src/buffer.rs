use std::sync::Arc;


pub struct Pointer {
	
}

pub struct Buffer {

}

impl Buffer {
	pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
		Self {

		}
	}

	pub fn store(&self, data: &[u8]) -> Pointer {
		Pointer {}
	}
}