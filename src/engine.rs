use std::collections::HashMap;
use std::sync::Arc;

use tokio::time::Instant;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalPosition;
use winit::dpi::PhysicalSize;
use winit::event;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window;
use winit::window::CursorGrabMode;
use winit::window::Window;
use winit::window::WindowId;

use crate::wgpu_buffers::*;
use crate::wgpu_types::*;
// use crate::game::EngineState;
// use crate::renderer::Renderer;
// use crate::serializer;
// use crate::serializer::serialize;

// pub async fn run_engine() {
//     let event_loop = EventLoop::new();
//     let window = WindowBuilder::new()
//         .with_title("Wizard Wars")
//         .build(&event_loop).unwrap();
//     window.set_cursor_visible(false);
//     let mut renderer = Renderer::new(window).await;

//     let time = Instant::now();
//     let mut game = EngineState::new(time.elapsed().as_millis() as u64);
//     let mut cursor_grabbed = true;

//     event_loop.run(move |event, _, control_flow| {
//         match event {
//             Event::WindowEvent { window_id, event } => {
//                 if window_id == renderer.window().id() {
//                     match event {
//                         WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
//                         WindowEvent::Resized(physical_size) => {
                         
//                             renderer.resize(physical_size);
//                         }
//                         WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
//                             renderer.resize(*new_inner_size);
//                         }
//                         WindowEvent::CursorMoved { device_id, position, modifiers:_ } => {
//                             if !cursor_grabbed {
//                                 return;
//                             }
//                             let middle_x = renderer.size.width as f64 / 2.0;
//                             let middle_y = renderer.size.height as f64 / 2.0;
//                             let dx = middle_x - position.x;
//                             let dy = middle_y - position.y;
//                             game.handle_cursor_moved(dx as f32, dy as f32);
//                             renderer.window().set_cursor_position(PhysicalPosition::new(middle_x, middle_y)).unwrap();
//                         }
//                         WindowEvent::MouseInput { device_id, state, button, modifiers } => {
//                             if !cursor_grabbed {
//                                 renderer.window().set_cursor_visible(false);
//                                 cursor_grabbed = true;
//                                 return;
//                             }

//                             game.handle_mouse_input(state, button);
//                         }
//                         WindowEvent::MouseWheel { device_id, delta, phase, modifiers } => {
//                             game.handle_mouse_wheel(phase, delta);
//                         }
//                         WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {
//                             game.handle_keyboard_input(input);

//                             if let (Some(virtual_keycode), true) = (input.virtual_keycode, input.state == winit::event::ElementState::Pressed) {
//                                 match virtual_keycode {
//                                     // Change to your desired key if not Escape
//                                     winit::event::VirtualKeyCode::Escape => {
//                                         renderer.window().set_cursor_grab(CursorGrabMode::None).unwrap();
//                                         renderer.window().set_cursor_visible(true);
//                                         cursor_grabbed = false;
//                                     }
//                                     _ => {}
//                                 }
//                             }
//                         }
//                         _ => {}
//                     }
//                 }
//             }
//             Event::RedrawRequested(_) => {
//                 game.update(time.elapsed().as_millis() as u64);
// 				let serialized = serialize(&game);
//                 renderer.update(serialized);

//                 // renderer.render();
//                 match renderer.render() {
//                     Ok(_) => {}
//                     // Reconfigure the surface if it's lost or outdated
//                     Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
//                         renderer.resize(renderer.size)
//                     }
//                     // The system is out of memory, we should probably quit
//                     Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
//                     // We're ignoring timeouts
//                     Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
//                 }
//             }
//             Event::MainEventsCleared => {
//                 // RedrawRequested will only trigger once, unless we manually
//                 // request it.
//                 renderer.window().request_redraw();
//             }
//             _ => {}
//         }
//     });
// }

struct BufferManager {

}

fn create_pipeline(adapter: &wgpu::Adapter, device: &wgpu::Device, surface: &wgpu::Surface, size: PhysicalSize<u32>) -> wgpu::RenderPipeline {
	let surface_caps = surface.get_capabilities(&adapter);
	let surface_format = surface_caps
		.formats
		.iter()
		.copied()
		.find(|f| f.is_srgb())
		.unwrap_or_else(|| surface_caps.formats[0]);

	let config = wgpu::SurfaceConfiguration {
		usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
		format: surface_format,
		width: size.width,
		height: size.height,
		present_mode: surface_caps.present_modes[0],
		alpha_mode: surface_caps.alpha_modes[0],
		view_formats: vec![],
		desired_maximum_frame_latency: 1
	};

	let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
		label: Some("Shader"),
		source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
	});

	let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
		label: Some("Render Pipeline Layout"),
		bind_group_layouts: &[],
		push_constant_ranges: &[],
	});
	let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
		label: Some("Render Pipeline"),
		layout: Some(&render_pipeline_layout),
		vertex: wgpu::VertexState {
			module: &shader,
			entry_point: "vs_main",
			buffers: &[Position::desc(), Normal::desc(), TexCoords::desc()],
			compilation_options: Default::default()
		},
		fragment: Some(wgpu::FragmentState {
			module: &shader,
			entry_point: "fs_main",
			targets: &[Some(wgpu::ColorTargetState {
				format: config.format,
				blend: Some(wgpu::BlendState {
					color: wgpu::BlendComponent::REPLACE,
					alpha: wgpu::BlendComponent::REPLACE,
				}),
				write_mask: wgpu::ColorWrites::ALL,
			})],
			compilation_options: Default::default()
		}),
		primitive: wgpu::PrimitiveState {
			topology: wgpu::PrimitiveTopology::TriangleList,
			strip_index_format: None,
			front_face: wgpu::FrontFace::Ccw,
			cull_mode: None,
			// Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
			// or Features::POLYGON_MODE_POINT
			polygon_mode: wgpu::PolygonMode::Fill,
			// Requires Features::DEPTH_CLIP_CONTROL
			unclipped_depth: false,
			// Requires Features::CONSERVATIVE_RASTERIZATION
			conservative: false,
		},
		depth_stencil: None,
		multisample: wgpu::MultisampleState {
			count: 1,
			mask: !0,
			alpha_to_coverage_enabled: false,
		},
		// If the pipeline will be used with a multiview render pass, this
		// indicates how many array layers the attachments will have.
		multiview: None,
	});

	render_pipeline
}

struct Renderer<'a> {
	window: Arc<Window>,
	surface: wgpu::Surface<'a>,
	pipeline: wgpu::RenderPipeline
}

impl<'a> Renderer<'a> {
	pub fn new(window: Arc<Window>, adapter: &wgpu::Adapter, device: &wgpu::Device) -> Self {
		let size = window.inner_size();
		let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
			backends: wgpu::Backends::all(),
			dx12_shader_compiler: Default::default(),
			..Default::default()
		});
		let surface = instance.create_surface(window.clone()).unwrap();
		let pipeline = create_pipeline(&adapter, &device, &surface, size);

		window.set_title("hello");

		Self {
			window,
			surface,
			pipeline
		}
	}

	pub fn window(&self) -> &Window {
		&self.window
	}

	pub fn render(&self, adapter: &wgpu::Adapter, device: &wgpu::Device) -> anyhow::Result<()> {
		let output = self.surface.get_current_texture()?;
		let view  = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
		let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("Render Encoder"),
		});
		
		{
			let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("Render Pass"),
				color_attachments: &[],
				depth_stencil_attachment: None,
				..Default::default()
			});

			render_pass.set_pipeline(&self.pipeline);

		}
		Ok(())
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
	device: wgpu::Device,
	queue: wgpu::Queue,
	adapter: wgpu::Adapter,
	renderers: HashMap<WindowId, Renderer<'a>>
}

impl<'a> Engine<'a> {
	pub async fn new() -> anyhow::Result<Self> {
		let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
		let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default())
			.await.expect("Failed to find an appropriate adapter");
		let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None)
			.await.expect("Failed to create device");

		Ok(Self {
			renderers: HashMap::new(),
			device,
			queue,
			adapter
		})
	}

	pub fn display(&mut self, path: &str) {
		log::info!("Displaying: {}", path);
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

			let renderer = Renderer::new(window, &self.adapter, &self.device);
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
				match renderer.render(&self.adapter, &self.device) {
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