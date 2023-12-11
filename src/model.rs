use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use cgmath::Quaternion;
use gltf::Document;
use gltf::Glb;
use gltf::Gltf;

use crate::structure::Structure;


pub async fn load_glb<P: AsRef<Path>>(path: P) -> Structure {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    // let glb = Glb::from_reader(&mut reader).unwrap();

    let gltf = Gltf::from_reader(reader).unwrap();

    // println!("{:?}", gltf);

    let meshes = gltf.meshes();

    // for mesh in meshes {
    //     //println!("{:?}", mesh);

    //     let primitimes = mesh.primitives();

    //     for primitive in primitimes {
    //         println!("{:?}", primitive);

    //         primitive.get(semantic)
    //     }
        
    // }

    // gltf.meshes().for_each(|mesh| {
    //     mesh.primitives().for_each(|primitive| {
    //         primitive.reader(|buffer| {
    //             let mut reader = buffer.reader(|_| Some(&gltf.blob)).unwrap();
    //             let vertexes: Vec<f32> = reader.read_vec_f32(primitive.get(&gltf::Semantic::Positions).unwrap()).unwrap();
    //             let indexes: Vec<u32> = reader.read_vec_u32(primitive.indices().unwrap()).unwrap();
    //             let location = primitive.get(&gltf::Semantic::Translation).unwrap().value();
    //             let rotation = primitive.get(&gltf::Semantic::Rotation).unwrap().value();
    //             let rotation = Quaternion::new(rotation[3], rotation[0], rotation[1], rotation[2]);
    //             let structure = Structure {
    //                 vertexes,
    //                 indexes,
    //                 location,
    //                 rotation,
    //             };
    //             println!("{:?}", structure);

    //             Some(structure)
    //         });
    //     });
    // });


    Structure { 
        vertexes: vec![], 
        indexes: vec![], 
        location: [0.0, 0.0, 0.0],
        rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
    }
}