use std::time::Instant;

use args::Args;
use args::Command;
use clap::Parser;
use engine::run_engine;
use game::Game;
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

mod renderer;
mod game;
mod types;
mod game_tests;
mod instance;
mod matrix;
mod camera;
mod structure;
mod gltf;
mod args;
mod engine;

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
            // let structure = gltf::load_glb(path).await;
            // println!("{:?}", structure);
        }
    }
}
