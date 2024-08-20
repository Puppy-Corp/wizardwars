use pge::*;

use crate::inventory::InvetoryItem;
use crate::utility::load_model;

pub struct Katana {
	node_id: Option<ArenaId<Node>>,
}

impl Katana {
	pub fn new() -> Self {
		Self {
			node_id: None,
		}
	}
}

impl InvetoryItem for Katana {
	fn prepare(&mut self, state: &mut pge::State) {
		let node_id = load_model("assets/katana.glb", state);
		let node = state.nodes.get_mut(&node_id).unwrap();
		node.translation = Vec3::new(0.3, -1.0, 3.0);
		node.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 220.0_f32.to_radians());
		self.node_id = Some(node_id);
	}

	fn activate(&mut self, state: &mut pge::State, parent_id: ArenaId<Node>) {
		let node_id = self.node_id.unwrap();
		let node = state.nodes.get_mut(&node_id).unwrap();
		node.parent = NodeParent::Node(parent_id);
	}

	fn deactivate(&mut self, state: &mut pge::State) {
		let node_id = self.node_id.unwrap();
		let node = state.nodes.get_mut(&node_id).unwrap();
		node.parent = NodeParent::Orphan;
	}
}