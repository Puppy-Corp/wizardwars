use pge::*;
use crate::inventory::Inventory;
use crate::utility::MoveDirection;

pub struct PlayerBuilder {
	scene_id: ArenaId<Scene>,
	inventory_size: usize,
	team: u32,
	mana: u32,
	health: u32,
	mass: f32,
	translation: Vec3,
}

impl PlayerBuilder {
	pub fn new(scene_id: ArenaId<Scene>) -> Self {
		Self {
			scene_id,
			inventory_size: 10,
			team: 0,
			mana: 100,
			health: 100,
			mass: 45.0,
			translation: Vec3::ZERO,
		}
	}

	pub fn team(mut self, team: u32) -> Self {
		self.team = team;
		self
	}

	pub fn inventory_size(mut self, size: usize) -> Self {
		self.inventory_size = size;
		self
	}

	pub fn mana(mut self, mana: u32) -> Self {
		self.mana = mana;
		self
	}

	pub fn health(mut self, health: u32) -> Self {
		self.health = health;
		self
	}

	pub fn mass(mut self, mass: f32) -> Self {
		self.mass = mass;
		self
	}

	pub fn translation(mut self, translation: Vec3) -> Self {
		self.translation = translation;
		self
	}

	pub fn build(self, state: &mut State) -> Player {
		let mut node = Node::new();
		node.physics.typ = PhycisObjectType::Dynamic;
		node.physics.mass = self.mass;
		node.collision_shape = Some(CollisionShape::Capsule {
			radius: 0.5,
			height: 1.8
		});
		node.translation = self.translation;
		node.parent = NodeParent::Scene(self.scene_id);
		let node_id = state.nodes.insert(node);

		Player {
			node_id,
			crouching: false,
			inventory: Inventory::new(self.inventory_size),
			spriting: false,
			jumping: false,
			prone: false,
			mana: 100,
			team: self.team,
			death: false,
			// yaw: 0.0,
			// pitch: 0.0,
			movdir: MoveDirection::new(),
			movement_force: 1600.0,
		}
	}
}

pub struct Player {
	pub node_id: ArenaId<Node>,
	mana: u32,
	pub inventory: Inventory,
	spriting: bool,
	pub jumping: bool,
	crouching: bool,
	prone: bool,
	team: u32,
	// yaw: f32,
	// pitch: f32,
	pub death: bool,
	pub movdir: MoveDirection,
	movement_force: f32,
}

impl Player {
	// pub fn spawn(state: &mut State) -> Self {
	// 	let node = Node::new();
	// 	let node_id = state.nodes.insert(node);

	// 	Self {
	// 		node_id,
	// 		mana: 100,
	// 		inventory: Inventory::new(),
	// 		spriting: false,
	// 		jumping: false,
	// 		crouching: false,
	// 		prone: false,
	// 		team: 0
	// 	}
	// }

	pub fn process(&mut self, state: &mut State) {
		let node = state.nodes.get_mut(&self.node_id).unwrap();
		// node.rotation = Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, 0.0);

		let current_speed = node.physics.velocity.length();
		if self.movdir.is_moving() {
			let dir = self.movdir.to_vec3();
			let mut force = node.rotation * dir;

			if force.x > 0.0 && node.physics.velocity.x < 0.0 {
				force.x += -node.physics.velocity.x * self.movement_force;
			} else if force.x < 0.0 && node.physics.velocity.x > 0.0 {
				force.x += -node.physics.velocity.x * self.movement_force;
			} else if current_speed < 25.0 {
				force.x *= self.movement_force;
			}

			if force.z > 0.0 && node.physics.velocity.z < 0.0 {
				force.z += -node.physics.velocity.z * self.movement_force;
			} else if force.z < 0.0 && node.physics.velocity.z > 0.0 {
				force.z += -node.physics.velocity.z * self.movement_force;
			} else if current_speed < 25.0 {
				force.z *= self.movement_force;
			}

			force.y = 0.0;

			node.physics.force = force;
			//log::info!("force: {:?}", node.physics.force);
		} else {
			// We calculate force opposite of momevement to slow down the player
			let force = -node.physics.velocity.xz() * self.movement_force;
			node.physics.force = Vec3::new(force.x, 0.0, force.y);
			//player.physics.force = glam::Vec3::ZERO;
		}

		if self.spriting {
			let dir = node.rotation * Vec3::new(0.0, 0.0, 1.0);
			node.physics.velocity = dir * 100.0;
		}
	}

	pub fn equip(&mut self, index: usize, state: &mut State) {
		self.inventory.equip(index, state, self.node_id);
	}

	pub fn switch_item(&mut self, state: &mut State, index: usize) {
		self.inventory.equip(index, state, self.node_id);
	}

	pub fn start_primary_action(&mut self, state: &mut State) {
		if let Some(item) = self.inventory.get_current_item() {
			item.start_primary_action(state);
		}
	}

	pub fn stop_primary_action(&mut self, state: &mut State) {
		if let Some(item) = self.inventory.get_current_item() {
			item.stop_primary_action(state);
		}
	}

	pub fn start_secondary_action(&mut self, state: &mut State) {
		if let Some(item) = self.inventory.get_current_item() {
			item.start_secondary_action(state);
		}
	}

	pub fn stop_secondary_action(&mut self, state: &mut State) {
		if let Some(item) = self.inventory.get_current_item() {
			item.stop_secondary_action(state);
		}
	}

	pub fn start_third_action(&mut self, state: &mut State) {
		if let Some(item) = self.inventory.get_current_item() {
			item.start_third_action(state);
		}
	}

	pub fn stop_third_action(&mut self, state: &mut State) {
		if let Some(item) = self.inventory.get_current_item() {
			item.stop_third_action(state);
		}
	}

	pub fn drop(&mut self, state: &mut State) {
		self.inventory.drop(state);
	}

	pub fn start_grap(&mut self, state: &mut State) {
		// self.inventory.grap(state);
	}

	pub fn stop_grap(&mut self, state: &mut State) {
		// self.inventory.stop_grap(state);
	}
	
	pub fn start_sprinting(&mut self, state: &mut State) {
		self.spriting = true;
	}

	pub fn stop_sprinting(&mut self, state: &mut State) {
		self.spriting = false;
	}

	pub fn jump(&mut self, state: &mut State) {
		let node = state.nodes.get_mut(&self.node_id).unwrap();
		node.physics.velocity.y = 10.0;
	}

	// pub fn start_jumping(&mut self) {
	// 	self.jumping = true;
	// }

	// pub fn stop_jumping(&mut self, state: &mut State) {
	// 	self.jumping = false;
	// }

	pub fn start_crouching(&mut self, state: &mut State) {
		self.crouching = true;
	}

	pub fn stop_crouching(&mut self, state: &mut State) {
		self.crouching = false;
	}

	pub fn start_prone(&mut self, state: &mut State) {
		self.prone = true;
	}

	pub fn stop_prone(&mut self, state: &mut State) {
		self.prone = false;
	}

	pub fn rotate(&mut self, dx: f32, dy: f32) {
		// self.yaw += dx * 0.001;
		// self.pitch += dy * 0.001;
		// self.pitch = self.pitch.clamp(-1.5, 1.5);
	}
}