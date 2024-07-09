mod args;

use log::LevelFilter;
use pge::*;
use rand::Rng;
use simple_logger::SimpleLogger;

#[derive(Debug, Clone)]
struct PressedKeys {
	forward: bool,
	backward: bool,
	left: bool,
	right: bool,
}

impl PressedKeys {
	pub fn new() -> Self {
		Self {
			forward: false,
			backward: false,
			left: false,
			right: false,
		}
	}

	pub fn to_vec3(&self) -> Vec3 {
        let mut direction = Vec3::ZERO;

        if self.forward {
            direction += Vec3::Z;
        }
        if self.backward {
            direction -= Vec3::Z;
        }
        if self.left {
            direction -= Vec3::X;
        }
        if self.right {
            direction += Vec3::X;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        direction
    }
}

pub struct WizardWars {
	sensitivity: f32,
	player_inx: Option<Index>,
	yaw: f32,
	pitch: f32,
	speed: f32,
	pressed_keys: PressedKeys,
}

impl WizardWars {
	pub fn new() -> Self {
		Self {
			sensitivity: 0.001,
			player_inx: None,
			yaw: 0.0,
			pitch: 0.0,
			speed: 10.0,
			pressed_keys: PressedKeys::new(),
		}
	}

	pub fn rotate_player(&mut self, x: f32, y: f32) {
		self.yaw += x * self.sensitivity;
		self.pitch += y * self.sensitivity;
		self.pitch = self.pitch.clamp(-1.5, 1.5);
	}
}

impl pge::App for WizardWars {
	fn on_create(&mut self, state: &mut pge::State) {
		log::info!("Creating game");

		let mut light_node = Node::new();
		light_node.translation = Vec3::new(0.0, 10.0, 0.0);
		let light_node_id = state.nodes.insert(light_node);

		let mut light = PointLight::new();
		light.node_id = Some(light_node_id);
		let light_id = state.point_lights.insert(light);

		let floor_mesh = plane(50.0, 50.0);
		let floor_id = state.meshes.insert(floor_mesh);

		let mut floor = Node::new();
		floor.translation = Vec3::new(0.0, -1.0, 0.0);
		floor.mesh = Some(floor_id);
		floor.collision_shape = Some(CollisionShape::Box { size: Vec3::new(50.0, 1.0, 50.0) });
		floor.physics.typ = PhycisObjectType::Static;
		state.nodes.insert(floor);

		let mut player = Node::new();
		player.translation = Vec3::new(0.0, 2.0, -30.0);
		// player.looking_at(0.0, 0.0, 0.0);
		let player_id = state.nodes.insert(player);
		self.player_inx = Some(player_id);

		let mut camera = Camera::new();
		camera.node_id = Some(player_id);
		let camera_id = state.cameras.insert(camera);
		let gui = camera_view(camera_id);
		let gui_id = state.guis.insert(gui);

		let cube_mesh = cube(1.0);
		let cube_mesh_id = state.meshes.insert(cube_mesh);
		let mut rng = rand::thread_rng();

		// Generate cubes in random locations
		for i in 0..5 {
			let rand_x = rng.gen_range(-25.0..25.0);
			let rand_z = rng.gen_range(-25.0..25.0);

			let mut cube = Node::new();
			cube.mesh = Some(cube_mesh_id);
			cube.translation = Vec3::new(
				rand_x,
				10.0,
				rand_z,
			);
			cube.scale = Vec3::new(1.0, 1.0, 1.0);
			cube.collision_shape = Some(CollisionShape::Box { size: Vec3::new(1.0, 1.0, 1.0) });
			cube.physics.typ = PhycisObjectType::Dynamic;
			cube.physics.mass = 1.0;
			let cube_id = state.nodes.insert(cube);
		}

		let window = Window::new()
			.title("Wizard Wars")
			.cam(gui_id)
			.lock_cursor(true);

		state.windows.insert(window);
	}

	fn on_keyboard_input(&mut self, key: KeyboardKey, action: KeyAction, state: &mut State) {
		match action {
			KeyAction::Pressed => {
				match key {
					KeyboardKey::W => self.pressed_keys.forward = true,
					KeyboardKey::S => self.pressed_keys.backward = true,
					KeyboardKey::A => self.pressed_keys.left = true,
					KeyboardKey::D => self.pressed_keys.right = true,
					_ => {}
				}
			},
			KeyAction::Released => {
				match key {
					KeyboardKey::W => self.pressed_keys.forward = false,
					KeyboardKey::S => self.pressed_keys.backward = false,
					KeyboardKey::A => self.pressed_keys.left = false,
					KeyboardKey::D => self.pressed_keys.right = false,
					_ => {}
				}
			},
		};
	}

	fn on_mouse_input(&mut self, event: MouseEvent, state: &mut State) {
		match event {
			MouseEvent::Moved { dx, dy } => {
				let player_inx = match self.player_inx {
					Some(index) => index,
					None => return,
				};
				self.rotate_player(dx, dy);
				let player = state.nodes.get_mut(player_inx).unwrap();
				player.rotation = Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, 0.0);
			},
		}
	}

	fn on_process(&mut self, state: &mut State, delta: f32) {
		// if let Some(index) = self.light_inx {
		// 	let light = state.nodes.get_mut(index).unwrap();
		// 	self.light_circle_i += delta;
		// 	let x = 10.0 * self.light_circle_i.cos();
		// 	let z = 10.0 * self.light_circle_i.sin();
		// 	light.set_translation(x, 10.0, z);
		// }

		match self.player_inx {
			Some(index) => match state.nodes.get_mut(index) {
				Some(player) => {
					let amount = self.pressed_keys.to_vec3() * delta * self.speed;
					player.translation += player.rotation * amount;
				},
				None => return,
			},
			None => return,
		};
	}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    // let args = Args::parse();

    Ok(pge::run(WizardWars::new()).await?)
}
