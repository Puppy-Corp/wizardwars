use std::f32::consts::PI;
use crate::types::Item;
use crate::utility::get_root_node;
use crate::utility::load_model;

use pge::*;

pub struct AK47 {
	node_id: ArenaId<Node>,
	shooting: bool,
	since_last_shot: f32,
	bullet_mesh_id: ArenaId<Mesh>,
	scene_id: ArenaId<Scene>,
}

impl AK47 {
	pub fn new(state: &mut State, scene_id: ArenaId<Scene>) -> Self {
		let cube = cube(0.3);
		let mesh_id = state.meshes.insert(cube);
		let node_id = load_model("assets/ak47.glb", state);
		let node = state.nodes.get_mut(&node_id).unwrap();
		node.translation = Vec3::new(0.3, -1.0, 1.0);
		// rotate 180 degrees
		node.rotation = Quat::from_euler(EulerRot::YXZ, PI * 1.5, 0.0, 0.0);
		node.scale = Vec3::new(60.0, 60.0, 60.0);

		Self {
			node_id,
			scene_id,
			shooting: false,
			since_last_shot: 0.0,
			bullet_mesh_id: mesh_id,
		}
	}
}

impl Item for AK47 {
	fn prepare(&mut self, state: &mut pge::State) {
	}

	fn activate(&mut self, state: &mut pge::State, parent_id: ArenaId<Node>) {
		let node = state.nodes.get_mut(&self.node_id).unwrap();
		node.parent = NodeParent::Node(parent_id);
	}

	fn hide(&mut self, state: &mut pge::State) {
		let node = state.nodes.get_mut(&self.node_id).unwrap();
		node.parent = NodeParent::Orphan;
	}

	fn start_primary_action(&mut self, state: &mut State) {
		log::info!("start shooting");
		self.shooting = true;
	}

	fn stop_primary_action(&mut self, state: &mut State) {
		log::info!("stop shooting");
		self.shooting = false;
	}

	fn process(&mut self, state: &mut State, dt: f32) {
		if !self.shooting {
			return;
		}

		self.since_last_shot += dt;

		if self.since_last_shot > 0.1 {
			self.since_last_shot = 0.0;
			log::info!("shoot");
			let root_node_id = get_root_node(&state, self.node_id);
			let root_node = state.nodes.get_mut(&root_node_id).unwrap();
			let (a, mut b, c) = root_node.rotation.to_euler(EulerRot::YXZ);
			b += -2.0_f32.to_radians();
			let new_rotation = Quat::from_euler(EulerRot::YXZ, a, b, c);
			root_node.rotation = new_rotation;
			let mut bullet_node = Node::new();
			bullet_node.mesh = Some(self.bullet_mesh_id);
			bullet_node.parent = NodeParent::Scene(self.scene_id);
			let mut translation = root_node.translation;
			translation += root_node.rotation * Vec3::new(0.0, 0.0, 3.0);
			bullet_node.translation = translation;
			//bullet_node.translation = ak_node.translation;
			// let mut translation = ak_node.translation;
			// translation += ak_node.rotation * Vec3::new(0.0, 0.0, 3.0);
			let dir = root_node.rotation * Vec3::new(0.0, 0.0, 1.0);
			bullet_node.physics.velocity = dir * 100.0;
			bullet_node.physics.mass = 1.0;
			bullet_node.physics.typ = PhycisObjectType::Dynamic;
			bullet_node.collision_shape = Some(CollisionShape::Sphere { radius: 0.1 });
			state.nodes.insert(bullet_node);

			// bullet_node.translation = node.translation * 
		}
	}
}