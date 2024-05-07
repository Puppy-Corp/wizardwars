use cgmath::InnerSpace;
use cgmath::Quaternion;
use cgmath::Vector3;
use winit::event::ElementState;
use winit::event::KeyboardInput;
use winit::event::MouseButton;
use winit::event::MouseScrollDelta;
use winit::event::TouchPhase;
use winit::event::VirtualKeyCode;
use crate::camera::CameraPos;
use crate::instance::Instance;
use crate::matrix::Matrix4x4;
use crate::structure::create_map;
use crate::types::*;

#[derive(Default, Clone)]
pub struct EngineState {
    pub entities: Vec<Entity>,
	pub meshes: Vec<Mesh>,
    player: Player,
    pub camera: CameraPos,
    other_players: Vec<Player>,
    speed: f32,
    state: PlayerState,
    last_update: u64,
    mouse_sensitivity: f32,
}

impl EngineState {
    pub fn new(time: u64) -> Self {
        let map = create_map();
		let entity = Entity {
			mesh: 0,
			loc: [0.0, 0.0, 0.0],
			rot: Quaternion::new(0.0, 0.0, 0.0, 0.0)
		};
		let entity2 = Entity {
			mesh: 0,
			loc: [50.0, 0.0, 0.0],
			rot: Quaternion::new(0.0, 0.0, 0.0, 0.0)
		};

        Self {
            entities: vec![entity, entity2],
			meshes: vec![map],
            speed: 5.0,
            player: Player {
                position: Vector3::new(0.0, 0.0, 0.0),
                rotation: Quaternion::new(0.0, 0.0, 0.0, 0.0)
            },
            camera: CameraPos::new(),
            other_players: Vec::new(),
            state: PlayerState::default(),
            last_update: time,
            mouse_sensitivity: 0.5,
        }
    }

    pub fn handle_cursor_moved(&mut self, dx: f32, dy: f32) {
        let delta_x = dx * self.mouse_sensitivity;
        let delta_y = dy * self.mouse_sensitivity;
        self.camera.rotate(delta_x, delta_y);
    }

    pub fn add_player() {
        
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
        let time_delta = time - self.last_update;
        self.last_update = time;

        let mut direction = Vector3::new(0.0, 0.0, 0.0);
        if self.state.forward {
            direction += Vector3::new(0.0, 0.0, 1.0);
        }
        if self.state.backward {
            direction += Vector3::new(0.0, 0.0, -1.0);
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

        direction = direction * self.speed * (time_delta as f32 / 1000.0);
        self.camera.move_eye(direction);
    }
}