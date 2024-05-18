use std::sync::Arc;

pub struct Pointer {
	offset: u64,
	length: u64,
	buffer: Arc<wgpu::Buffer>
}

pub struct BufferManager {
	device: Arc<wgpu::Device>,
	queue: Arc<wgpu::Queue>,
	offset: u64,
	buffer: Arc<wgpu::Buffer>
}

impl BufferManager {
	pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
		let buffer = device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("Buffer"),
			size: 1024,
			usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
			mapped_at_creation: false
		});

		Self {
			offset: 0,
			device: device.clone(),
			queue,
			buffer: Arc::new(buffer)
		}
	}

	pub fn store(&self, data: &[u8]) -> Pointer {
		self.queue.write_buffer(&self.buffer, self.offset, data);

		Pointer {
			offset: self.offset,
			length: data.len() as u64,
			buffer: self.buffer.clone()
		}
	}
}