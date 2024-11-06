use std::time::Instant;
use pge::*;
use crate::ak47::AK47;
use crate::dark_dungeon::DarkDungeon;
use crate::generated_pvp_map::GeneratedPVPMap;
use crate::inventory::Inventory;
use crate::katana::Katana;
// use crate::mobs::spawn_mob;
use crate::mobs::MobSpawner;
use crate::npc::Npc;
use crate::player::Player;
use crate::types::SurvivalMap;

pub struct Survival {
	player: Player,
	main_scene_id: ArenaId<Scene>,
	wave: u32,
	enemies: Vec<Npc>,
	max_enemies: u32,
	enemies_spawned: u32,
	since_last_spawn: Instant,
	map: Box<dyn SurvivalMap>,
	spawner: MobSpawner
}

impl Survival {
	pub fn new(state: &mut State, window_id: ArenaId<Window>) -> Self {
		let main_scene = Scene::new();
		let main_scene_id = state.scenes.insert(main_scene);
		let map = GeneratedPVPMap::new(state, main_scene_id, 100.0, 100.0);
		// let map = DarkDungeon::create(state, main_scene_id);

		let mut player_node = Node::new();
		player_node.parent = NodeParent::Scene(main_scene_id);
		player_node.physics.mass = 100.0;
		player_node.physics.typ = PhycisObjectType::Dynamic;
		player_node.collision_shape = Some(CollisionShape::Box { size: Vec3::new(0.5, 1.8, 0.5) });
		player_node.translation = Vec3::new(0.0, 1.0, 0.0);
		let player_node_id = state.nodes.insert(player_node);
		let inventory = Inventory::new(4);
		let mut player = Player::new(player_node_id, inventory);
		player.inventory.add_item(AK47::new(state, main_scene_id));
		player.inventory.add_item(Katana::new(state, main_scene_id));

		let mut camera = pge::Camera::new();
		camera.zfar = 1000.0;
		camera.node_id = Some(player_node_id);
		let camera_id = state.cameras.insert(camera);

		let ui = stack(&[
			camera_view(camera_id),
			row(&[
				rect().background_color(Color::BLUE),
				rect().background_color(Color::RED),
				rect().background_color(Color::CYAN),
				rect().background_color(Color::WHITE)
			]).height(0.1).anchor_bottom()
		]);
		let ui_id = state.guis.insert(ui);	
		let window = state.windows.get_mut(&window_id).unwrap();
		window.ui = Some(ui_id);

		let spawner = MobSpawner::new(state, main_scene_id);

		Self {
			wave: 0,
			player,
			main_scene_id,
			enemies: Vec::new(),
			max_enemies: 10,
			enemies_spawned: 0,
			since_last_spawn: Instant::now(),
			map: Box::new(map),
			spawner
		}
	}

	fn start_next_wave(&mut self, state: &mut State) {
		// How to despawn enemies ??
		self.wave += 1;
		self.max_enemies += 5;
		self.enemies_spawned = 0;
	}

	pub fn on_mouse_input(&mut self, event: MouseEvent, state: &mut State) {
		match event {
			MouseEvent::Moved { dx, dy } => {
				let node = state.nodes.get_mut(&self.player.node_id).unwrap();
				let (a, b, c) = node.rotation.to_euler(EulerRot::YXZ);
				let yaw = a + dx * 0.002;
				let pitch = b + dy * 0.002;
				node.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
				self.player.on_mouse_moved(dx, dy, state);
			},
			MouseEvent::Pressed { button } => {
				match button {
					MouseButton::Left => self.player.start_primary_action(state),
					MouseButton::Right => self.player.start_secondary_action(state),
					MouseButton::Middle => self.player.start_third_action(state),
					_ => {}
				}
			},
			MouseEvent::Released { button } => {
				match button {
					MouseButton::Left => self.player.stop_primary_action(state),
					MouseButton::Right => self.player.stop_secondary_action(state),
					MouseButton::Middle => self.player.stop_third_action(state),
					_ => {}
				}
			},
			_ => {}
		}
	}

	pub fn on_keyboard_input(&mut self, key: KeyboardKey, action: KeyAction, state: &mut State) {
		match action {
			KeyAction::Pressed => {
				match key {
					KeyboardKey::W => self.player.movdir.forward = true,
					KeyboardKey::S => self.player.movdir.backward = true,
					KeyboardKey::A => self.player.movdir.left = true,
					KeyboardKey::D => self.player.movdir.right = true,
					KeyboardKey::ShiftLeft => self.player.start_sprinting(state),
					KeyboardKey::Space => self.player.jump(state),
					KeyboardKey::G => self.player.drop(state),
					KeyboardKey::F => self.player.start_grap(state),
					KeyboardKey::Digit1 => self.player.equip(0, state),
					KeyboardKey::Digit2 => self.player.equip(1, state),
					KeyboardKey::Digit3 => self.player.equip(2, state),
					KeyboardKey::Digit4 => self.player.equip(3, state),
					KeyboardKey::Digit5 => self.player.equip(4, state),
					KeyboardKey::Digit6 => self.player.equip(5, state),
					_ => {}
				}
			},
			KeyAction::Released => {
				match key {
					KeyboardKey::W => self.player.movdir.forward = false,
					KeyboardKey::S => self.player.movdir.backward = false,
					KeyboardKey::A => self.player.movdir.left = false,
					KeyboardKey::D => self.player.movdir.right = false,
					KeyboardKey::ShiftLeft => self.player.stop_sprinting(state),
					KeyboardKey::Space => self.player.jumping = false,
					KeyboardKey::F => self.player.stop_grap(state),
					_ => {}
				}
			},
		};

		log::info!("Pressed keys: {:?}", self.player.movdir);
	}

	pub fn on_process(&mut self, state: &mut State, dt: f32) {
		self.player.process(state, dt);
		let mut all_enemies_dead = true;
		for enemy in &mut self.enemies {
			enemy.process(state, &self.player, dt);
			if !enemy.player.death {
				all_enemies_dead = false;
			}
		}

		if all_enemies_dead && self.enemies_spawned >= self.max_enemies {
			self.start_next_wave(state);
		}

		if self.enemies_spawned < self.max_enemies {
			let time_since_last_spawn = self.since_last_spawn.elapsed().as_secs_f32();
			if time_since_last_spawn > 2.0 {
				log::info!("spawn new mob");
				self.enemies.push(self.spawner.spawn(state, self.map.get_mob_spawn_point()));
				self.enemies_spawned += 1;
				self.since_last_spawn = Instant::now();
			}
		}
		self.map.process(state);
	}
}