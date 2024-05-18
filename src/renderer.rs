use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::buffer::BufferManager;
use crate::types::DrawInstruction;
use crate::wgpu_types::*;

fn create_pipeline(adapter: &wgpu::Adapter, device: &wgpu::Device, surface: &wgpu::Surface, size: PhysicalSize<u32>) -> wgpu::RenderPipeline {
	log::info!("creating pipeline");
	let surface_caps = surface.get_capabilities(&adapter);
	log::info!("surface caps: {:?}", surface_caps);
	let surface_format = surface_caps
		.formats
		.iter()
		.copied()
		.find(|f| f.is_srgb())
		.unwrap_or_else(|| surface_caps.formats[0]);
	log::info!("surface format: {:?}", surface_format);
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
	log::info!("config {:?}", config);
	surface.configure(&device, &config);
	log::info!("configured surface");
	let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
		label: Some("Shader"),
		source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
	});
	let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
		label: Some("Render Pipeline Layout"),
		bind_group_layouts: &[],
		push_constant_ranges: &[],
	});
	log::info!("creating render pipeline");
	let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
		label: Some("Render Pipeline"),
		layout: Some(&render_pipeline_layout),
		vertex: wgpu::VertexState {
			module: &shader,
			entry_point: "vs_main",
			buffers: &[Position::desc(), Normal::desc(), TexCoords::desc(), Instance::desc()],
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
	log::info!("created render pipeline");

	render_pipeline
}

pub struct RenderArgs {
	pub window: Arc<Window>,
	pub instance: Arc<wgpu::Instance>,
	pub adapter: Arc<wgpu::Adapter>,
	pub device: Arc<wgpu::Device>,
	pub positions_buffer: BufferManager
}

pub struct Renderer<'a> {
	window: Arc<Window>,
	surface: wgpu::Surface<'a>,
	device: Arc<wgpu::Device>,
	adapter: Arc<wgpu::Adapter>,
	pipeline: wgpu::RenderPipeline
}

impl<'a> Renderer<'a> {
	pub fn new(args: RenderArgs) -> Self {
		let size = args.window.inner_size();
		let surface = args.instance.create_surface(args.window.clone()).unwrap();
		let pipeline = create_pipeline(&args.adapter, &args.device, &surface, size);
		Self {
			window: args.window,
			surface,
			device: args.device,
			adapter: args.adapter,
			pipeline
		}
	}

	pub fn window(&self) -> &Window {
		&self.window
	}

	pub fn render(&self, instructions: &[DrawInstruction]) -> anyhow::Result<()> {
		let output = self.surface.get_current_texture()?;
		let view  = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
		let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
            // render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            // for shape in &self.shapes {
            //     let index_start = shape.index_buffer_index as u64;
            //     let index_end = shape.index_buffer_index as u64 + shape.index_buffer_len as u64;
            //     let vertex_start = shape.vertex_buffer_index as u64;
            //     let vertex_end = shape.vertex_buffer_index as u64 + shape.vertex_buffer_len as u64;
            //     let instance_start = shape.instance_buffer_index as u32;
            //     let instance_end = shape.instance_buffer_index as u32 + shape.instance_buffer_len as u32;
            //     render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(vertex_start..vertex_end));
            //     render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            //     render_pass.set_index_buffer(self.index_buffer.slice(index_start..index_end), wgpu::IndexFormat::Uint16);
            //     render_pass.draw_indexed(0..(shape.index_buffer_len / 2) as u32, 0, instance_start..instance_end);
            // }

			render_pass.set_vertex_buffer(0, buffer_slice)

			for instruction in instructions {

			}
		}
		Ok(())
	}
}