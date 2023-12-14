use std::fs;
use std::path::Path;

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

    let json = String::from_utf8(data[20..length].to_vec()).unwrap();


    println!("length: {}", length);
    println!("json: {}", json);
}

#[cfg(test)]
mod tests {
    use super::parse_glb;

    #[test]
    fn test_parse_box() {
        parse_glb("./models/box.glb")
    }
}