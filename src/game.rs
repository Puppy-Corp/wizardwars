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
use crate::structure::Structure;
use crate::structure::create_map;
use crate::types::GameState;
use crate::types::Player;
use crate::types::PlayerState;
use crate::types::SerializedGame;
use crate::types::ShapeDesc;
use crate::types::Vertex;

#[derive(Default, Clone)]
pub struct Game {
    game_state: GameState,
    strctures: Vec<Structure>,
    player: Player,
    camera: CameraPos,
    other_players: Vec<Player>,
    speed: f32,
    state: PlayerState,
    last_update: u64,
    mouse_sensitivity: f32,
}

impl Game {
    pub fn new(time: u64) -> Self {
        let game = create_map();

        Self {
            game_state: GameState::Lobby,
            strctures: vec![game],
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

    pub fn serialize(&self) -> SerializedGame {
        let mut vertex_buffer = vec![];
        let mut index_buffer = vec![];

        let mut index_buffer_index = 0;
        let mut vertex_buffer_index = 0;
        let mut instance_buffer_index = 0;

        let mut shapes = vec![];
        let mut instance_buffer = vec![];

        for structure in &self.strctures {
            let desc = ShapeDesc {
                index_buffer_index,
                vertex_buffer_index,
                index_buffer_len: structure.indexes.len() * std::mem::size_of::<u16>(),
                vertex_buffer_len: structure.vertexes.len() * std::mem::size_of::<Vertex>(),
                instance_buffer_index: instance_buffer_index,
                instance_buffer_len: 1
            };
            index_buffer_index += desc.index_buffer_len;
            vertex_buffer_index += desc.vertex_buffer_len;
            instance_buffer_index += desc.instance_buffer_len;

            shapes.push(desc);
            instance_buffer.push(Instance::new(Matrix4x4::from_translation(&structure.location)));

            index_buffer.extend(structure.indexes.iter());
            vertex_buffer.extend(structure.vertexes.iter());

        }

        SerializedGame {
            index_buffer: index_buffer,
            vertex_buffer: vertex_buffer,
            shapes: shapes,
            instance_buffer: instance_buffer,
            camera: self.camera.clone(),
        }
    }
}