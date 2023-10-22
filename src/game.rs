use std::time::Instant;

use cgmath::InnerSpace;
use cgmath::Quaternion;
use cgmath::Vector3;
use winit::dpi::PhysicalPosition;
use winit::event::ElementState;
use winit::event::KeyboardInput;
use winit::event::MouseButton;
use winit::event::MouseScrollDelta;
use winit::event::ScanCode;
use winit::event::TouchPhase;
use winit::event::VirtualKeyCode;

use crate::types::GameDiff;
use crate::types::GameState;
use crate::types::Player;
use crate::types::Structure;

#[derive(Default)]
struct PlayerState {
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
    primary: bool,
    secondary: bool,
    third: bool,
}

pub struct Game {
    game_state: GameState,
    strctures: Vec<Structure>,
    player: Player,
    speed: f32,
    state: PlayerState,
    last_update: u64,
}

impl Game {
    pub fn new(time: u64) -> Self {
        Self {
            game_state: GameState::Lobby,
            strctures: Vec::new(),
            speed: 300.0,
            player: Player {
                position: Vector3::new(0.0, 0.0, 0.0),
                rotation: Quaternion::new(0.0, 0.0, 0.0, 0.0)
            },
            state: PlayerState::default(),
            last_update: time,
        }
    }

    pub fn diff(&self, other: &Game) -> GameDiff {
        GameDiff {

        }
    }

    pub fn handle_cursor_moved(&mut self, position: PhysicalPosition<f64>) {
        
    }

    pub fn handle_mouse_input(&mut self, state: ElementState, button: MouseButton) {
        match button {
            MouseButton::Left => self.state.primary = state == ElementState::Pressed,
            MouseButton::Right => self.state.secondary = state == ElementState::Pressed,
            MouseButton::Middle => self.state.third = state == ElementState::Pressed,
            _ => {}
        }
    }

    pub fn handle_mouse_wheel(&mut self, phase: TouchPhase, delta: MouseScrollDelta) {

    }

    pub fn handle_keyboard_input(&mut self, input: KeyboardInput) {
        let keycode = match input.virtual_keycode {
            None => return,
            Some(keycode) => keycode,
        };

        match keycode {
            VirtualKeyCode::W => self.state.forward = input.state == ElementState::Pressed,
            VirtualKeyCode::A => self.state.left = input.state == ElementState::Pressed,
            VirtualKeyCode::S => self.state.backward = input.state == ElementState::Pressed,
            VirtualKeyCode::D => self.state.right = input.state == ElementState::Pressed,
            _ => {
                log::info!("Unhandled keyboard input: {:?}", keycode);
            }
        }
    }

    pub fn update(&mut self, time: u64) {
        let delta = time - self.last_update;
        self.last_update = time;

        let mut direction = Vector3::new(0.0, 0.0, 0.0);
        if self.state.forward {
            direction += Vector3::new(0.0, 0.0, -1.0);
        }
        if self.state.backward {
            direction += Vector3::new(0.0, 0.0, 1.0);
        }
        if self.state.left {
            direction += Vector3::new(-1.0, 0.0, 0.0);
        }
        if self.state.right {
            direction += Vector3::new(1.0, 0.0, 0.0);
        }

        if direction.magnitude() > 0.0 {
            direction = direction.normalize();
        }

        self.player.position += direction * self.speed * (delta as f32 / 1000.0);

        println!("{:?}", self.player.position);
    }
}