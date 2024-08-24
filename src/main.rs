mod args;
mod inventory;
mod ak47;
mod katana;
mod utility;
mod player;
mod types;
mod pulse_rifle;
mod survival;
mod npc;
mod mobs;
mod dark_dungeon;
mod controller;

use std::time::Instant;
use ak47::AK47;
use inventory::Inventory;
use katana::Katana;
use log::LevelFilter;
use pge::*;
use player::Player;
use rand::Rng;
use simple_logger::SimpleLogger;
use survival::Survival;
use utility::MoveDirection;

enum GameMode {
	Loading,
	Survival(Survival)
}

struct Bullet {
	spawned: Instant,
	node_id: ArenaId<Node>,
}

pub struct WizardWars {
	main_scene: Option<ArenaId<Scene>>,
	sensitivity: f32,
	player_id: Option<ArenaId<Node>>,
	yaw: f32,
	pitch: f32,
	speed: f32,
	pressed_keys: MoveDirection,
	dashing: bool,
	movement_force: f32,
	rng: rand::rngs::ThreadRng,
	player_ray: Option<ArenaId<RayCast>>,
	gripping: bool,
	gripping_node: Option<ArenaId<Node>>,
	bullets: Vec<Bullet>,
	firing_rate: Instant,
	shooting: bool,
	bullet_mesh: Option<ArenaId<Mesh>>,
	current_player: Option<Player>,
	players: Vec<Player>,
	game_mode: GameMode,
}

impl WizardWars {
	pub fn new() -> Self {
		Self {
			main_scene: None,
			sensitivity: 0.001,
			player_id: None,
			yaw: 0.0,
			pitch: 0.0,
			speed: 10.0,
			pressed_keys: MoveDirection::new(),
			dashing: false,
			movement_force: 1600.0,
			rng: rand::thread_rng(),
			player_ray: None,
			gripping: false,
			gripping_node: None,
			bullets: Vec::new(),
			firing_rate: Instant::now(),
			shooting: false,
			bullet_mesh: None,
			current_player: None,
			players: Vec::new(),
			game_mode: GameMode::Loading
		}
	}

	fn handle_rays(&mut self, state: &mut State) {
		if !self.gripping {
			return;
		}

		let player_ray = match self.player_ray {
			Some(index) => match state.raycasts.get_mut(&index) {
				Some(ray) => ray,
				None => return,
			},
			None => return,
		};

		if player_ray.intersects.len() == 0 {
			return;
		}

		let translation = {
			let player_inx = match self.player_id {
				Some(index) => index,
				None => return,
			};

			let player = match state.nodes.get_mut(&player_inx) {
				Some(node) => node,
				None => return,
			};

			let dir = player.rotation * Vec3::new(0.0, 0.0, 1.0);
			player.translation + dir * 5.0
		};

		let node = match self.gripping_node {
			Some(index) => match state.nodes.get_mut(&index) {
				Some(node) => node,
				None => return,
			},
			None =>  {
				match player_ray.intersects.first() {
					Some(inx) => {
						self.gripping_node = Some(*inx);
						match state.nodes.get_mut(inx) {
							Some(node) => node,
							None => return,
						}
					},
					None => return,
				}
			},
		};

		// let first_node = match player_ray.intersects.first() {
		// 	Some(inx) => {
		// 		self.gripping_node = Some(*inx);
		// 		match state.nodes.get_mut(inx) {
		// 			Some(node) => node,
		// 			None => return,
		// 		}
		// 	},
		// 	None => return,
		// };

		if node.physics.typ != PhycisObjectType::Dynamic {
			return;
		}

		node.translation = translation;
	}

	fn handle_dashing(&mut self, state: &mut State) {
		if self.dashing {
			let player_inx = match self.player_id {
				Some(index) => index,
				None => return,
			};
			let player = match state.nodes.get_mut(&player_inx) {
				Some(node) => node,
				None => return,
			};
			let dir = player.rotation * Vec3::new(0.0, 0.0, 1.0);
			player.physics.velocity = dir * 100.0;
		}
	}

	fn handle_shooting(&mut self, state: &mut State) {
		if self.firing_rate.elapsed().as_secs_f32() < 0.1 {
			return;
		}
		self.firing_rate = Instant::now();

		if !self.shooting {
			return;
		}

		let player_inx = match self.player_id {
			Some(index) => index,
			None => return,
		};

		if let Some(bullet_mesh_id) = self.bullet_mesh {
			log::info!("spawn bullet");
			let mut bullet = Node::new();
			bullet.mesh = Some(bullet_mesh_id);
			bullet.physics.typ = PhycisObjectType::Dynamic;
			bullet.physics.mass = 1.0;
			bullet.collision_shape = Some(CollisionShape::Box { size: Vec3::new(0.3, 0.3, 0.3) });
			bullet.parent = NodeParent::Scene(self.main_scene.unwrap());
			let rotation = state.nodes.get(&player_inx).unwrap().rotation;
			let mut translation = state.nodes.get(&player_inx).unwrap().translation;
			// location in fron of player
			translation += rotation * Vec3::new(0.0, 0.0, 3.0);
			bullet.translation = translation;
			
			let dir = rotation * Vec3::new(0.0, 0.0, 1.0);
			bullet.physics.velocity = dir * 50.0;
			let bullet_id = state.nodes.insert(bullet);
			self.bullets.push(Bullet {
				spawned: Instant::now(),
				node_id: bullet_id,
			});
		}

		let player = match state.nodes.get_mut(&player_inx) {
			Some(node) => node,
			None => return,
		};

		// let dir = player.rotation * Vec3::new(0.0, 0.0, 0.3);
		// self.recoil_force = dir * -100.0;

		// rotate comera up
		self.pitch -= 0.05;
		// let rot = glam::Quat::from_euler(glam::EulerRot::YXZ, 0.0, 0.3, 0.0);
		// player.rotation = rot * player.rotation;
	}

	fn handle_moving(&mut self, state: &mut State) {
		let player = match self.player_id {
			Some(index) => match state.nodes.get_mut(&index) {
				Some(player) => player,
				None => return,
			},
			None => return,
		};

		let current_speed = player.physics.velocity.length();
		if self.pressed_keys.is_moving() {
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
	}

	pub fn rotate_player(&mut self, x: f32, y: f32) {
		self.yaw += x * self.sensitivity;
		self.pitch += y * self.sensitivity;
		self.pitch = self.pitch.clamp(-1.5, 1.5);
	}

	pub fn hide_node(&mut self, state: &mut State, node_id: ArenaId<Node>) {
		let node = match state.nodes.get_mut(&node_id) {
			Some(node) => node,
			None => return,
		};

		node.parent = NodeParent::Orphan;
	}
}

impl pge::App for WizardWars {
	fn on_create(&mut self, state: &mut pge::State) {
		let window = Window::new()
			.title("Wizard Wars")
			.lock_cursor(true);
		let window_id = state.windows.insert(window);

		// let mut camera = pge::Camera::new();
		// camera.zfar = 1000.0;
		// camera.node_id = Some(player.node_id);
		// let camera_id = state.cameras.insert(camera);

		self.game_mode = GameMode::Survival(Survival::new(state, window_id));

		let main_scene = Scene::new();
		let main_scene_id = state.scenes.insert(main_scene);
		self.main_scene = Some(main_scene_id);

		// let player = Player::spawn(state);

		let bullet_mesh = cube(0.3);
		let bullet_mesh_id = state.meshes.insert(bullet_mesh);
		self.bullet_mesh = Some(bullet_mesh_id);

		// self.ak47 = Some(load_model("assets/ak47.glb", state));
		// self.katana = Some(load_model("assets/katana.glb", state));

		// self.inventory.prepare(state);

		// let mut light_node = Node::new();
		// light_node.translation = Vec3::new(0.0, 45.0, 0.0);
		// let light_node_id = state.nodes.insert(light_node);



		let mut player = Node::new();
		player.translation = Vec3::new(0.0, 2.0, -30.0);
		// player.looking_at(0.0, 0.0, 0.0);
		player.physics.mass = 30.0;
		player.physics.typ = PhycisObjectType::Dynamic;
		player.collision_shape = Some(CollisionShape::Box { size: Vec3::new(1.0, 2.0, 1.0) });
		player.parent = NodeParent::Scene(main_scene_id);
		let player_id = state.nodes.insert(player);
		self.player_id = Some(player_id);

		let raycast = RayCast::new(player_id, 10.0);
		let player_ray_inx = state.raycasts.insert(raycast);
		self.player_ray = Some(player_ray_inx);

		let mut camera = Camera::new();
		camera.zfar = 1000.0;
		camera.node_id = Some(player_id);
		let camera_id = state.cameras.insert(camera);
		// let gui = camera_view(camera_id);
		// let gui_id = state.guis.insert(gui);

		// let cube_mesh = cube(1.0);
		// let cube_mesh_id = state.meshes.insert(cube_mesh);
		

		// // Generate cubes in random locations
		// for i in 0..10 {
		// 	let rand_x = self.rng.gen_range(-size / 2.0..size / 2.0);
		// 	let rand_z = self.rng.gen_range(-size / 2.0..size / 2.0);

		// 	let mut cube = Node::new();
		// 	cube.mesh = Some(cube_mesh_id);
		// 	cube.translation = Vec3::new(
		// 		rand_x,
		// 		10.0,
		// 		rand_z,
		// 	);
		// 	cube.scale = Vec3::new(2.0, 2.0, 2.0);
		// 	cube.collision_shape = Some(CollisionShape::Box { size: Vec3::new(2.0, 2.0, 2.0) });
		// 	cube.physics.typ = PhycisObjectType::Dynamic;
		// 	cube.physics.mass = 1.0;
		// 	cube.parent = NodeParent::Scene(main_scene_id);
		// 	let cube_id = state.nodes.insert(cube);
		// }

		let ui = stack(&[
			camera_view(camera_id),
			row(&[
				rect().background_color(Color::BLUE),
				rect().background_color(Color::RED),
				rect().background_color(Color::CYAN),
				rect().background_color(Color::WHITE)
			]).height(0.1).anchor_bottom()
		]);
		let gui_id = state.guis.insert(ui);
	}

	fn on_keyboard_input(&mut self, key: KeyboardKey, action: KeyAction, state: &mut State) {
		match self.game_mode {
			GameMode::Survival(ref mut survival) => {
				survival.on_keyboard_input(key, action, state);
			},
			_ => {}
		}
	}

	fn on_mouse_input(&mut self, event: MouseEvent, state: &mut State) {
		match self.game_mode {
			GameMode::Survival(ref mut survival) => {
				survival.on_mouse_input(event, state);
			},
			_ => {}
		}

		// match event {
		// 	MouseEvent::Moved { dx, dy } => {
		// 		let player_inx = match self.player_id {
		// 			Some(index) => index,
		// 			None => return,
		// 		};
		// 		self.rotate_player(dx, dy);
		// 		let player = state.nodes.get_mut(&player_inx).unwrap();
		// 		player.rotation = Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, 0.0);
		// 	},
		// 	MouseEvent::Pressed { button } => {
		// 		match button {
		// 			MouseButton::Left => {
		// 				match self.gripping_node.take() {
		// 					Some(gripping_node_id) => {
		// 						self.gripping = false;
		// 						let push_vel = {
		// 							let player_inx = match self.player_id {
		// 								Some(index) => index,
		// 								None => return,
		// 							};
	
		// 							let player = match state.nodes.get_mut(&player_inx) {
		// 								Some(node) => node,
		// 								None => return,
		// 							};
	
		// 							let dir = player.rotation * Vec3::new(0.0, 0.0, 1.0);
		// 							dir * 100.0
		// 						};
	
		// 						if let Some(node) = state.nodes.get_mut(&gripping_node_id) {
		// 							node.physics.velocity = push_vel;
		// 						}
		// 					},
		// 					None => {
		// 						self.shooting = true;
		// 					}
		// 				}
						
		// 				self.shooting = true
		// 			},
		// 			_ => {}
		// 		}
		// 	},
		// 	MouseEvent::Released { button } => {
		// 		match button {
		// 			MouseButton::Left => self.shooting = false,
		// 			_ => {}
		// 		}
		// 	},
		// 	_ => {}
		// }
	}

	fn on_process(&mut self, state: &mut State, delta: f32) {
		match self.game_mode {
			GameMode::Survival(ref mut survival) => {
				survival.on_process(state, delta);
			},
			_ => {}
		};

		// if let Some(index) = self.light_inx {
		// 	let light = state.nodes.get_mut(index).unwrap();
		// 	self.light_circle_i += delta;
		// 	let x = 10.0 * self.light_circle_i.cos();
		// 	let z = 10.0 * self.light_circle_i.sin();
		// 	light.set_translation(x, 10.0, z);
		// }

		// self.handle_rays(state);
		// self.handle_dashing(state);
		// self.handle_shooting(state);
		// self.handle_moving(state);

		// for player in &mut self.players {
		// 	player.process(state);
		// }
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
