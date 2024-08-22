use std::f32::consts::PI;
use crate::types::Item;
use crate::utility::load_model;

use pge::*;

pub struct AK47 {
	node_id: Option<ArenaId<Node>>,
}

impl AK47 {
	pub fn new() -> Self {
		Self {
			node_id: None,
		}
	}
}

impl Item for AK47 {
	fn prepare(&mut self, state: &mut pge::State) {
		let node_id = load_model("assets/ak47.glb", state);
		let node = state.nodes.get_mut(&node_id).unwrap();
		node.translation = Vec3::new(0.3, -1.0, 1.0);
		// rotate 180 degrees
		node.rotation = Quat::from_euler(EulerRot::YXZ, PI, 0.0, 0.0);
		self.node_id = Some(node_id);
	}

	fn activate(&mut self, state: &mut pge::State, parent_id: ArenaId<Node>) {
		let node_id = self.node_id.unwrap();
		let node = state.nodes.get_mut(&node_id).unwrap();
		node.parent = NodeParent::Node(parent_id);
	}

	fn hide(&mut self, state: &mut pge::State) {
		let node_id = self.node_id.unwrap();
		let node = state.nodes.get_mut(&node_id).unwrap();
		node.parent = NodeParent::Orphan;
	}
}