use crate::game::EngineState;
use crate::instance::Instance;
use crate::matrix::Matrix4x4;
use crate::types::SerializedState;
use crate::types::ShapeDesc;
use crate::types::Vertex;

#[derive(Debug)]
struct MeshRange {
	start: usize,
	end: usize,
}

pub fn serialize(engine_state: &EngineState) -> SerializedState {
	let mut index_buffer = Vec::new();
	let mut vertex_buffer = Vec::new();
	let mut instance_buffer = Vec::new();
	let mut shapes = Vec::new();

	let mut index_mesh_ranges: Vec<MeshRange>  = Vec::with_capacity(engine_state.meshes.len());
	let mut vertex_mesh_ranges: Vec<MeshRange> = Vec::with_capacity(engine_state.meshes.len());

	for mesh in &engine_state.meshes {
		let inx_start = index_buffer.len();
		let vtx_start = vertex_buffer.len();
		index_buffer.extend(mesh.indices.iter().map(|i| *i as u16));
		vertex_buffer.extend(mesh.vertices.iter().cloned());
		let inx_end = index_buffer.len();
		let vtx_end = vertex_buffer.len();
		index_mesh_ranges.push(MeshRange { start: inx_start, end: inx_end });
		vertex_mesh_ranges.push(MeshRange { start: vtx_start, end: vtx_end });
	}
	
	for entity in &engine_state.entities {
		instance_buffer.push(Instance::new(Matrix4x4::from_translation(&entity.loc)));
		let inx_range = &index_mesh_ranges[entity.mesh];
		let vtx_range = &vertex_mesh_ranges[entity.mesh];
		shapes.push(ShapeDesc {
			index_buffer_index: inx_range.start * 2,
			vertex_buffer_index: vtx_range.start * Vertex::size(),
			index_buffer_len: (inx_range.end - inx_range.start) * 2,
			vertex_buffer_len: (vtx_range.end - vtx_range.start) * Vertex::size(),
			instance_buffer_index: instance_buffer.len() - 1,
			instance_buffer_len: 1,
		});
	}

	SerializedState {
		camera: engine_state.camera.clone(),
		index_buffer,
		vertex_buffer,
		instance_buffer,
		shapes,
	}
}