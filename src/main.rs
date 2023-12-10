use std::time::Instant;

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

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Wizard Wars")
        .build(&event_loop).unwrap();
    window.set_cursor_visible(false);
    let mut renderer = Renderer::new(window).await;

    let time = Instant::now();
    let mut game = Game::new(time.elapsed().as_millis() as u64);
    let mut cursor_grabbed = true;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { window_id, event } => {
                if window_id == renderer.window().id() {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                         
                            renderer.resize(physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            renderer.resize(*new_inner_size);
                        }
                        WindowEvent::CursorMoved { device_id, position, modifiers:_ } => {
                            if !cursor_grabbed {
                                return;
                            }
                            let middle_x = renderer.size.width as f64 / 2.0;
                            let middle_y = renderer.size.height as f64 / 2.0;
                            let dx = middle_x - position.x;
                            let dy = middle_y - position.y;
                            game.handle_cursor_moved(dx as f32, dy as f32);
                            renderer.window().set_cursor_position(PhysicalPosition::new(middle_x, middle_y)).unwrap();
                        }
                        WindowEvent::MouseInput { device_id, state, button, modifiers } => {
                            if !cursor_grabbed {
                                renderer.window().set_cursor_visible(false);
                                cursor_grabbed = true;
                                return;
                            }

                            game.handle_mouse_input(state, button);
                        }
                        WindowEvent::MouseWheel { device_id, delta, phase, modifiers } => {
                            game.handle_mouse_wheel(phase, delta);
                        }
                        WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {
                            game.handle_keyboard_input(input);

                            if let (Some(virtual_keycode), true) = (input.virtual_keycode, input.state == winit::event::ElementState::Pressed) {
                                match virtual_keycode {
                                    // Change to your desired key if not Escape
                                    winit::event::VirtualKeyCode::Escape => {
                                        renderer.window().set_cursor_grab(CursorGrabMode::None).unwrap();
                                        renderer.window().set_cursor_visible(true);
                                        cursor_grabbed = false;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(_) => {
                game.update(time.elapsed().as_millis() as u64);
                renderer.update(game.serialize());

                // renderer.render();
                match renderer.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        renderer.resize(renderer.size)
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // We're ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                renderer.window().request_redraw();
            }
            _ => {}
        }
    });
}
