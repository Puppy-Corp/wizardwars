use std::any::Any;
use std::fs;
use std::io;
use std::time::Instant;

use args::Args;
use args::Command;
use clap::Parser;
use engine::run_engine;
use game::Game;
use gltf::accessor::sparse::IndexType;
use log::LevelFilter;
use renderer::Renderer;
use simple_logger::SimpleLogger;
use winit::dpi::PhysicalPosition;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::CursorGrabMode;
use winit::window::WindowBuilder;

use crate::mesh::Mesh;
use crate::mesh::PrimitiveTopology;

mod renderer;
mod game;
mod types;
mod game_tests;
mod instance;
mod matrix;
mod camera;
mod structure;
// mod gltf;
mod args;
mod engine;
mod byte_eater;
mod mesh;

#[tokio::main]
async fn main() {
    let filter_level = match std::env::var("WIZARDWARS_LOG_LEVEL") {
        Ok(lev) => {
            match lev.as_str() {
                "info" => LevelFilter::Info,
                "debug" => LevelFilter::Debug,
                "error" => LevelFilter::Error,
                _ => LevelFilter::Info
            }
        },
        Err(_) => {
            LevelFilter::Info
        }
    };

    SimpleLogger::new()
        .with_level(filter_level)
        .without_timestamps()
        .init()
        .unwrap();

    let args = Args::parse();

    let command = match args.command {
        Some(command) => command,
        None => {
            run_engine().await;
            return;
        }
    }; 

    match command {
        Command::Inspect { path } => {
			// let file = fs::File::open(path).unwrap();
			// let reader = io::BufReader::new(file);
			// let gltf = gltf::Gltf::from_reader(reader).unwrap();

			let (gltf, buffers, _) = gltf::import(&path).unwrap();

			// if let Some(b) = gltf.blob {
			// 	println!("Blob size: {}", b.len());
			// }

			// let buffers = gltf.buffers();

			// println!("{:#?}", gltf);
			for node in gltf.nodes() {
				match node.mesh() {
					Some(mesh) => {
						println!("Mesh: {}", mesh.name().unwrap_or("Unnamed"));
						
						// println!("{:#?}", mesh);
						for primitive in mesh.primitives() {
							let mut new_mesh = Mesh::new(PrimitiveTopology::from_mode(primitive.mode()));
				// println!("{:#?}", primitive);
							// primitive.indices().
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
									new_mesh.positions.push([vertex_position[0], vertex_position[1], vertex_position[2]]);
									// println!("{:?}", vertex_position);
								}
							}

							reader.read_indices().map(|iter| {
								for index in iter.into_u32() {
									// println!("{:?}", index);
									new_mesh.indices.push(index);
								}
							});

							// for (s, a) in primitive.attributes() {
							// 	let v = a.view().unwrap();
							// 	v.
							// 	println!("Accessor: {:?}", a.data_type());
							// 	println!("component type: {:?}", a.type_);
							// 	if let Some(v) = a.view() {
							// 		let offset = v.offset();
							// 		println!("Offset: {}", offset);
							// 		// println!("Buffer: {:#?}", v.buffer());
							// 		let buffer = v.buffer();
							// 		let source = buffer.source();
							// 		match source {
							// 			gltf::buffer::Source::Bin => {
							// 				println!("Binary");

							// 				// buffer.
							// 			},
							// 			gltf::buffer::Source::Uri(uri) => {
							// 				println!("URI: {}", uri);
							// 			}
							// 		}
							// 	}
							// }
							println!("Mesh.topology {:?}", new_mesh.topology);
							println!("Mesh.indices {}", new_mesh.indices.len());
							println!("Mesh.positions {}", new_mesh.positions.len());
						}


					},
					None => {}
				}
			}

			// for accessor in gltf.accessors() {
			// 	match accessor.data_type() {
			// 		gltf::accessor::DataType::F32 => {
			// 			println!("F32: {:#?}", accessor);
			// 			let sparse = accessor.sparse();

			// 			if let Some(sparse) = sparse {
			// 				let indices = sparse.indices();

			// 				match indices.index_type() {
			// 					IndexType::U32 => {
			// 						println!("U32");
			// 					},
			// 					_ => {}
			// 				}

			// 				let v = sparse.values();
			// 				v.
			// 			}
			// 		},
			// 		_ => {}
			// 	}
			// }
            // let structure = gltf::load_glb(path).await;
            // println!("{:?}", structure);
        }
    }
}
