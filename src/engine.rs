use std::collections::HashMap;
use std::sync::Arc;
use wgpu::Backends;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowId;

use crate::buffer::Buffer;
use crate::gltf::load_meshes;
use crate::matrix::Matrix4x4;
use crate::renderer::RenderArgs;
use crate::renderer::Renderer;
use crate::types::DrawInstruction;
use crate::types::Node;
use crate::wgpu_buffers::*;
use crate::wgpu_types::*;

fn extract_positions(node: &Node, positions: &mut Vec<[f32; 3]>) {
	for mesh in node.meshes.iter() {
		for vertex in mesh.vertices.iter() {
			positions.push(vertex.pos);
		}
	}

	for child in node.children.iter() {
		extract_positions(child, positions);
	}
}

pub struct Layouts {
	pub camera_bind_group_layout: wgpu::BindGroupLayout,
}

impl Layouts {
	pub fn new(device: &wgpu::Device) -> Self {
		Self {
			camera_bind_group_layout: create_camera_bind_group_layout(device),
		}
	}
}

pub struct Engine<'a> {
	device: Arc<wgpu::Device>,
	queue: Arc<wgpu::Queue>,
	adapter: Arc<wgpu::Adapter>,
	instance: Arc<wgpu::Instance>,
	renderers: HashMap<WindowId, Renderer<'a>>,
	position_buffer: Buffer,
	normal_buffer: Buffer,
	tex_coord_buffer: Buffer,
	instance_buffer: Buffer,
}

impl<'a> Engine<'a> {
	pub async fn new() -> anyhow::Result<Self> {
		let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

		let adapters = instance.enumerate_adapters(Backends::all());

		for adapter in adapters {
			println!("Adapter: {:?}", adapter.get_info());
		}

		let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default())
			.await.expect("Failed to find an appropriate adapter");
		let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None)
			.await.expect("Failed to create device");
		
		let device = Arc::new(device);
		let queue = Arc::new(queue);

		device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("Camera Buffer"),
			size: std::mem::size_of::<CameraUniform>() as u64,
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			mapped_at_creation: false,
		});

		let position_buffer = Buffer::new(device.clone(), queue.clone());
		let normal_buffer = Buffer::new(device.clone(), queue.clone());
		let tex_coord_buffer = Buffer::new(device.clone(), queue.clone());
		let instance_buffer = Buffer::new(device.clone(), queue.clone());

		Ok(Self {
			renderers: HashMap::new(),
			device,
			queue,
			adapter: Arc::new(adapter),
			instance: Arc::new(instance),
			position_buffer,
			normal_buffer,
			tex_coord_buffer,
			instance_buffer,
		})
	}

	pub fn display(&mut self, path: &str) {
		log::info!("Displaying: {}", path);

		let mut nodes = Vec::<Node>::new();
		load_meshes(path, &mut nodes);

		println!("Nodes: {:?}", nodes);

		for node in nodes {
			println!("Node: {:?}", node);
			let mut positions: Vec<[f32; 3]> = Vec::with_capacity(node.meshes.iter().map(|m| m.vertices.len()).sum());
			extract_positions(&node, &mut positions);

			let pointer = self.position_buffer.store(bytemuck::cast_slice(&positions));
			let m = Matrix4x4::from_translation(&[0.0, 0.0, 0.0]);

			// let instance = DrawInstruction {
			// 	instance: ,
			// 	position_pointer: pointer,
			// };
		}
	}

	pub fn run(mut self) -> anyhow::Result<()> {
		let event_loop = EventLoop::new()?;
		Ok(event_loop.run_app(&mut self)?)
	}
}


impl ApplicationHandler for Engine<'_> {
	fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
		log::info!("Resumed");

		if self.renderers.is_empty() {
			let window_attributes = Window::default_attributes()
            	.with_title("Wizardwars");
			let window = event_loop.create_window(window_attributes).unwrap();
			let window_id = window.id();
			let window = Arc::new(window);

			let renderer = Renderer::new(RenderArgs {
				window: window.clone(),
				instance: self.instance.clone(),
				adapter: self.adapter.clone(),
				device: self.device.clone(),
			});
			self.renderers.insert(window_id, renderer);
		}
	}

	fn window_event(
		&mut self,
		event_loop: &winit::event_loop::ActiveEventLoop,
		window_id: winit::window::WindowId,
		event: WindowEvent,
	) {
		match event {
			WindowEvent::CloseRequested => {
				self.renderers.remove(&window_id);
				event_loop.exit();
			}
			WindowEvent::RedrawRequested => {
				let renderer = self.renderers.get_mut(&window_id).unwrap();
				match renderer.render() {
					Ok(_) => {}
					Err(err) => {
						log::error!("Error rendering: {:?} window {:?}", err, window_id);
					}
				}
			}
			_ => {}
		}
	}
}