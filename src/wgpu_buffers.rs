use wgpu::util::DeviceExt;

use crate::wgpu_types::CameraUniform;


// fn create_camera_buffer(device: &wgpu::Device, view_proj: [[f32; 4]; 4]) -> wgpu::Buffer {
//     let camera_uniform = CameraUniform { view_proj };
//     device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//         label: Some("Camera Uniform Buffer"),
//         contents: bytemuck::cast_slice(&[camera_uniform]),
//         usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
//     })
// }

// fn create_model_buffer(device: &wgpu::Device, model: [[f32; 4]; 4], normal_matrix: [[f32; 4]; 4]) -> wgpu::Buffer {
//     let model_uniform = ModelUniform { model, normal_matrix };
//     device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//         label: Some("Model Uniform Buffer"),
//         contents: bytemuck::cast_slice(&[model_uniform]),
//         usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
//     })
// }

// fn create_point_light_buffer(device: &wgpu::Device, point_light: PointLight) -> wgpu::Buffer {
// 	device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
// 		label: Some("Point Light Buffer"),
// 		contents: bytemuck::cast_slice(&[point_light]),
// 		usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
// 	})
// }

// fn create_spot_light_buffer(device: &wgpu::Device, spot_light: SpotLight) -> wgpu::Buffer {
// 	device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
// 		label: Some("Spot Light Buffer"),
// 		contents: bytemuck::cast_slice(&[spot_light]),
// 		usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
// 	})
// }


pub fn create_position_buffer(device: &wgpu::Device, contents: &[u8]) -> wgpu::Buffer {
	device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
		label: Some("Position Buffer"),
		contents,
		usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
	})
}

pub fn create_empty_position_buffer(device: &wgpu::Device, size: usize) -> wgpu::Buffer {
	device.create_buffer(&wgpu::BufferDescriptor {
		label: Some("Position Buffer"),
		size: (size * std::mem::size_of::<[f32; 3]>()) as u64,
		usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
		mapped_at_creation: false,
	})
}

pub fn create_normal_buffer(device: &wgpu::Device, contents: &[u8]) -> wgpu::Buffer {
	device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
		label: Some("Normal Buffer"),
		contents,
		usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
	})
}

pub fn create_empty_normal_buffer(device: &wgpu::Device, size: usize) -> wgpu::Buffer {
	device.create_buffer(&wgpu::BufferDescriptor {
		label: Some("Normal Buffer"),
		size: (size * std::mem::size_of::<[f32; 3]>()) as u64,
		usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
		mapped_at_creation: false,
	})
}

pub fn create_tex_coords_buffer(device: &wgpu::Device, contents: &[u8]) -> wgpu::Buffer {
	device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
		label: Some("Tex Coords Buffer"),
		contents,
		usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
	})
}

pub fn create_empty_tex_coords_buffer(device: &wgpu::Device, size: usize) -> wgpu::Buffer {
	device.create_buffer(&wgpu::BufferDescriptor {
		label: Some("Tex Coords Buffer"),
		size: (size * std::mem::size_of::<[f32; 2]>()) as u64,
		usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
		mapped_at_creation: false,
	})
}


pub fn create_camera_empty_buffer(device: &wgpu::Device) -> wgpu::Buffer {
	device.create_buffer(&wgpu::BufferDescriptor {
		label: Some("Camera Buffer"),
		size: std::mem::size_of::<CameraUniform>() as u64,
		usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		mapped_at_creation: false,
	})
}

pub fn create_camera_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
	device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
		label: Some("Camera Bind Group Layout"),
		entries: &[
			wgpu::BindGroupLayoutEntry {
				binding: 0,
				visibility: wgpu::ShaderStages::VERTEX,
				ty: wgpu::BindingType::Buffer { 
                    ty: wgpu::BufferBindingType::Uniform, 
                    has_dynamic_offset: false, 
                    min_binding_size: None
                },
				count: None,
			}
		]
	})
}

pub fn create_camera_bind_group(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, camera_buffer: &wgpu::Buffer) -> wgpu::BindGroup {
	device.create_bind_group(&wgpu::BindGroupDescriptor {
		label: Some("Camera Bind Group"),
		layout,
		entries: &[
			wgpu::BindGroupEntry {
				binding: 0,
				resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
					buffer: camera_buffer,
					offset: 0,
					size: None,
				}),
			}
		]
	})
}

pub fn create_indirect_buffer(device: &wgpu::Device, contents: &[u8]) -> wgpu::Buffer {
	device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
		label: Some("Indirect Buffer"),
		contents,
		usage: wgpu::BufferUsages::INDIRECT | wgpu::BufferUsages::COPY_DST,
	})
}