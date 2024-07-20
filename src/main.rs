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

	pub fn any_pressed(&self) -> bool {
		self.forward || self.backward || self.left || self.right
	}
}

pub struct WizardWars {
	sensitivity: f32,
	player_inx: Option<Index>,
	yaw: f32,
	pitch: f32,
	speed: f32,
	pressed_keys: PressedKeys,
	dashing: bool,
	movement_force: f32,
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
			dashing: false,
			movement_force: 1600.0,
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
		let texture = Texture::new("assets/wall_medium.png");
		let texture_id = state.textures.insert(texture);

		let size = 200.0;

		let mut wall_mesh = plane(size, size);
		wall_mesh.tex_coords = vec![
			[0.0, 0.0],
			[1.0, 0.0],
			[1.0, 1.0],
			[0.0, 1.0],
		];
		wall_mesh.texture = Some(texture_id);

		let mut forward_wall = Node::new();
		forward_wall.mesh = Some(state.meshes.insert(wall_mesh.clone()));
		forward_wall.translation = Vec3::new(0.0, 0.0, size);
		forward_wall.looking_at(0.0, 0.0, 0.0);
		// rotate 90 degrees
		forward_wall.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 1.5708, 0.0);
		forward_wall.collision_shape = Some(CollisionShape::Box { size: Vec3::new(size, size, 1.0) });
		forward_wall.physics.typ = PhycisObjectType::Static;
		state.nodes.insert(forward_wall);

		let mut back_wall = Node::new();
		back_wall.mesh = Some(state.meshes.insert(wall_mesh.clone()));
		back_wall.translation = Vec3::new(0.0, 0.0, -size);
		back_wall.looking_at(0.0, 0.0, 0.0);
		// rotate 90 degrees
		back_wall.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 1.5708, 0.0);
		back_wall.collision_shape = Some(CollisionShape::Box { size: Vec3::new(size, size, 1.0) });
		back_wall.physics.typ = PhycisObjectType::Static;
		state.nodes.insert(back_wall);

		let mut left_wall = Node::new();
		left_wall.mesh = Some(state.meshes.insert(wall_mesh.clone()));
		left_wall.translation = Vec3::new(-size, 0.0, 0.0);
		left_wall.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 1.5708);
		left_wall.collision_shape = Some(CollisionShape::Box { size: Vec3::new(size, size, 1.0) });
		left_wall.physics.typ = PhycisObjectType::Static;
		// left_wall.looking_at(0.0, 0.0, 0.0);
		state.nodes.insert(left_wall);

		let mut top_wall = Node::new();
		top_wall.mesh = Some(state.meshes.insert(wall_mesh.clone()));
		top_wall.translation = Vec3::new(0.0, size, 0.0);
		top_wall.rotation = Quat::from_euler(EulerRot::YXZ, 1.5708, 0.0, 0.0);
		top_wall.collision_shape = Some(CollisionShape::Box { size: Vec3::new(size, size, 1.0) });
		top_wall.physics.typ = PhycisObjectType::Static;
		state.nodes.insert(top_wall);

		let mut right_wall = Node::new();
		right_wall.mesh = Some(state.meshes.insert(wall_mesh.clone()));
		right_wall.translation = Vec3::new(size, 0.0, 0.0);
		right_wall.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 1.5708);
		right_wall.collision_shape = Some(CollisionShape::Box { size: Vec3::new(size, size, 1.0) });
		right_wall.physics.typ = PhycisObjectType::Static;
		state.nodes.insert(right_wall);

		let mut light_node = Node::new();
		light_node.translation = Vec3::new(0.0, 45.0, 0.0);
		let light_node_id = state.nodes.insert(light_node);

		let mut light = PointLight::new();
		light.node_id = Some(light_node_id);
		let light_id = state.point_lights.insert(light);

		let floor_mesh = plane(size, size);
		let floor_id = state.meshes.insert(floor_mesh);

		let mut floor = Node::new();
		floor.translation = Vec3::new(0.0, -1.0, 0.0);
		floor.mesh = Some(floor_id);
		floor.collision_shape = Some(CollisionShape::Box { size: Vec3::new(size, 1.0, size) });
		floor.physics.typ = PhycisObjectType::Static;
		state.nodes.insert(floor);

		let mut player = Node::new();
		player.translation = Vec3::new(0.0, 2.0, -30.0);
		// player.looking_at(0.0, 0.0, 0.0);
		player.physics.mass = 30.0;
		player.physics.typ = PhycisObjectType::Dynamic;
		player.collision_shape = Some(CollisionShape::Box { size: Vec3::new(1.0, 2.0, 1.0) });
		let player_id = state.nodes.insert(player);
		self.player_inx = Some(player_id);

		let mut camera = Camera::new();
		camera.zfar = 1000.0;
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
			.ui(gui_id)
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
					KeyboardKey::ShiftLeft => self.dashing = true,
					_ => {}
				}
			},
			KeyAction::Released => {
				match key {
					KeyboardKey::W => self.pressed_keys.forward = false,
					KeyboardKey::S => self.pressed_keys.backward = false,
					KeyboardKey::A => self.pressed_keys.left = false,
					KeyboardKey::D => self.pressed_keys.right = false,
					KeyboardKey::ShiftLeft => self.dashing = false,
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
					let current_speed = player.physics.velocity.length();
					if self.pressed_keys.any_pressed() {
						let dir = self.pressed_keys.to_vec3();
						let mut force = player.rotation * dir;

						if force.x > 0.0 && player.physics.velocity.x < 0.0 {
							force.x += -player.physics.velocity.x * self.movement_force;
						} else if force.x < 0.0 && player.physics.velocity.x > 0.0 {
							force.x += -player.physics.velocity.x * self.movement_force;
						} else if current_speed < 25.0 {
							force.x *= self.movement_force;
						}

						if force.z > 0.0 && player.physics.velocity.z < 0.0 {
							force.z += -player.physics.velocity.z * self.movement_force;
						} else if force.z < 0.0 && player.physics.velocity.z > 0.0 {
							force.z += -player.physics.velocity.z * self.movement_force;
						} else if current_speed < 25.0 {
							force.z *= self.movement_force;
						}

						force.y = 0.0;

						player.physics.force = force;
						log::info!("force: {:?}", player.physics.force);
					} else {
						// We calculate force opposite of momevement to slow down the player
						let force = -player.physics.velocity.xz() * self.movement_force;
						player.physics.force = Vec3::new(force.x, 0.0, force.y);
						//player.physics.force = glam::Vec3::ZERO;
					}

					if self.dashing {
						let dir = player.rotation * Vec3::new(0.0, 0.0, 1.0);
						player.physics.velocity = dir * 100.0;
					}
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
