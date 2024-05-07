use std::fs;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use crate::byte_eater::ByteEater;
use crate::types::PrimitiveTopology;
use crate::types::Vertex;

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    pub generator: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
    pub name: String,
    pub nodes: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub mesh: usize,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PBRMetallicRoughness {
    base_color_factor: [f32; 4],
    metallic_factor: f32,
    roughness_factor: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    double_sided: bool,
    name: String,
    pbr_metallic_roughness: PBRMetallicRoughness,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Atributes {
    #[serde(rename = "POSITION")]
    pub position: usize,
    #[serde(rename = "NORMAL")]
    pub normal: usize,
    #[serde(rename = "TEXCOORD_0")]
    pub tex_coord: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Primitive {
    pub attributes: Atributes,
    pub indices: Option<usize>,
    pub material: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mesh {
    pub name: Option<String>,
	// TODO: weights
    pub primitives: Vec<Primitive>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Acessor {
    pub buffer_view: usize,
    pub component_type: usize,
    pub count: usize,
    pub min: Option<Vec<f32>>,
    pub max: Option<Vec<f32>>,
    #[serde(rename = "type")]
    pub ty: AccessorType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BufferView {
    pub buffer: usize,
    pub byte_length: usize,
    pub byte_offset: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Buffer {
    pub byte_length: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GLTFHeader {
    pub asset: Asset,
    pub scene: u32,
    pub scenes: Vec<Scene>,
    pub nodes: Vec<Node>,
    pub meshes: Vec<Mesh>,
    pub accessors: Vec<Acessor>,
    pub buffer_views: Vec<BufferView>,
    pub buffers: Vec<Buffer>,
}

static MAGIC: [u8; 4] = [b'g', b'l', b'T', b'F'];
static JSON: u32 = 0x4E4F534A; // "JSON"
static BIN: u32 = 0x004E4942; // "BIN"

enum ComponentType {
	SignedByte,		// 8 bits
	UnsignedByte,   // 8 bits
	Short,		    // 16 bits
	UnsignedShort,  // 16 bits
	UnsignedInt,    // 32 bits
	Float,		    // 32 bits
}

impl From<u32> for ComponentType {
	fn from(value: u32) -> Self {
		match value {
			5120 => ComponentType::SignedByte,
			5121 => ComponentType::UnsignedByte,
			5122 => ComponentType::Short,
			5123 => ComponentType::UnsignedShort,
			5125 => ComponentType::UnsignedInt,
			5126 => ComponentType::Float,
			_ => panic!("Invalid component type")
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
enum AccessorType {
	#[serde(rename = "SCALAR")]
	Scalar,
	#[serde(rename = "VEC2")]
	Vec2,
	#[serde(rename = "VEC3")]
	Vec3,
	#[serde(rename = "VEC4")]
	Vec4,
	#[serde(rename = "MAT2")]
	Mat2,
	#[serde(rename = "MAT3")]
	Mat3,
	#[serde(rename = "MAT4")]
	Mat4
}

impl From<&str> for AccessorType {
	fn from(value: &str) -> Self {
		match value {
			"SCALAR" => AccessorType::Scalar,
			"VEC2" => AccessorType::Vec2,
			"VEC3" => AccessorType::Vec3,
			"VEC4" => AccessorType::Vec4,
			"MAT2" => AccessorType::Mat2,
			"MAT3" => AccessorType::Mat3,
			"MAT4" => AccessorType::Mat4,
			_ => panic!("Invalid accessor type")
		}
	}
}

pub struct ParseResult {
	pub header: GLTFHeader,
	pub binary: Vec<u8>,
}

pub fn parse_glb<P: AsRef<Path>>(path: P) -> ParseResult {
    let data = fs::read(path).unwrap();

    if data.len() < 12 {
        panic!("Invalid glTF file");
    }

	let mut byte_eater = ByteEater::new(&data);

    if byte_eater.read_bytes(4) != MAGIC {
        panic!("Invalid glTF file");
    }

	byte_eater.set_index(12);

	let json_length = byte_eater.read_u32() as usize;
	println!("json length: {}", json_length);

    if byte_eater.read_bytes(4) != JSON.to_le_bytes() {
        panic!("Invalid glTF file");
    }

	byte_eater.set_index(20);
	let json = String::from_utf8(byte_eater.read_bytes(json_length).to_vec()).unwrap();
    let header: GLTFHeader = serde_json::from_str(&json).unwrap();

	println!("json: {:#?}", header);

	println!("currentInx {}", byte_eater.index());

	let bin_length = byte_eater.read_u32() as usize;

	println!("bin length: {}", bin_length);

	println!("currentInx {}", byte_eater.index());

	let byts = byte_eater.read_bytes(4);
	println!("byts: {:X?}", byts);

	if byts != BIN.to_le_bytes() {
		panic!("Invalid glTF file");
	}
	
	let binary = byte_eater.read_bytes(bin_length);

	println!("jsong length: {}", json_length);
	println!("json: {:#?}", header);
	// println!("binary length: {}", bin_length);
	// println!("binary: {:X?}", binary);

	ParseResult {
		header,
		binary: binary.to_vec(),
	}
}

#[cfg(test)]
mod tests {
    use gltf::buffer;

    use super::parse_glb;

    #[test]
    fn test_parse_box() {
        let result = parse_glb("./models/box.glb");
    
		for node in result.header.nodes.iter() {
			println!("node: {:#?}", node);

			let mesh = &result.header.meshes[node.mesh];

			for primitive in mesh.primitives.iter() {
				println!("primitive: {:#?}", primitive);
				let accessor = &result.header.accessors[primitive.attributes.position];
				println!("accessor: {:#?}", accessor);

				let buffer_view = &result.header.buffer_views[accessor.buffer_view];
				println!("buffer_view: {:#?}", buffer_view);

				let buffer = &result.binary[buffer_view.byte_offset..buffer_view.byte_offset + buffer_view.byte_length];
				println!("buffer: {:X?}", buffer);

				
			}
		}
	}
}

pub fn load_meshes<P: AsRef<Path>>(path: P, meshes: &mut Vec<crate::types::Mesh>) {
	let path = path.as_ref();
	let (gltf, buffers, _) = gltf::import(&path).unwrap();

	for node in gltf.nodes() {
		match node.mesh() {
			Some(mesh) => {
				println!("Mesh: {}", mesh.name().unwrap_or("Unnamed"));
				for primitive in mesh.primitives() {
					let mut new_mesh = crate::types::Mesh::new(PrimitiveTopology::from_mode(primitive.mode()));
					println!("- Primitive #{}", primitive.index());

					for (semantic, acc) in primitive.attributes() {
						println!("Semantic: {:?}", semantic);
					}

					let reader = primitive.reader(|buffer| {
						let buffer_data = &buffers[buffer.index()];
						Some(&buffer_data.0[..])
					});
					if let Some(iter) = reader.read_positions() {
						for vertex_position in iter {
							new_mesh.vertices.push(Vertex::new([vertex_position[0], vertex_position[1], vertex_position[2]]));
							// println!("{:?}", vertex_position);
						}
					}

					reader.read_indices().map(|iter| {
						for index in iter.into_u32() {
							// println!("{:?}", index);
							new_mesh.indices.push(index);
						}
					});

					meshes.push(new_mesh);
				}
			},
			None => {}
		}
	}
}