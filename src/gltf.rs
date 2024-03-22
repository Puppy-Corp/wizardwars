use std::fs;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

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
    pub indices: usize,
    pub material: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mesh {
    pub name: String,
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
    pub ty: String,
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

pub fn parse_glb<P: AsRef<Path>>(path: P) {
    let data = fs::read(path).unwrap();

    if data.len() < 12 {
        panic!("Invalid glTF file");
    }

    if data[0..4] != MAGIC {
        panic!("Invalid glTF file");
    }

    if data[16..20] != JSON.to_le_bytes() {
        panic!("Invalid glTF file");
    }

    let length = u32::from_le_bytes([data[12], data[13], data[14], data[15]]) as usize;

    let json = String::from_utf8(data[20..20+length].to_vec()).unwrap();
    let json: GLTFHeader = serde_json::from_str(&json).unwrap();


    println!("length: {}", length);
    println!("json: {:?}", json);
}

#[cfg(test)]
mod tests {
    use super::parse_glb;

    #[test]
    fn test_parse_box() {
        parse_glb("./models/box.glb")
    }
}