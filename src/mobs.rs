use std::os::macos::raw::stat;

use pge::cube;
use pge::ArenaId;
use pge::Node;
use pge::NodeParent;
use pge::Scene;
use pge::State;
use pge::Vec3;

use crate::inventory::Inventory;
use crate::npc::Npc;
use crate::player;
use crate::utility::load_model;

// pub fn spawn_mob(state: &mut State, main_scene_id: ArenaId<Scene>, location: Vec3) -> Npc {
// 	let path 

// 	let player = PlayerBuilder::new(main_scene_id).build(state);
// 	Npc::new(player)
// }

pub struct MobSpawner {
	main_scene_id: ArenaId<Scene>,
	// model_node: ArenaId<Node>
}

impl MobSpawner {
	pub fn new(state: &mut State, main_scene_id: ArenaId<Scene>) -> Self {
		

		Self {
			main_scene_id,
			// model_node: node_id
		}
	}

	pub fn spawn(&mut self, state: &mut State, translation: Vec3) -> Npc {
		// let cube = cube(1.5);
		// let mesh_id = state.meshes.insert(cube);

		let mut player_node = Node::new();
		player_node.parent = NodeParent::Scene(self.main_scene_id);
		player_node.translation = translation;
		player_node.physics.mass = 10.0;
		player_node.physics.typ = pge::PhycisObjectType::Dynamic;
		player_node.collision_shape = Some(pge::CollisionShape::Box { size: Vec3::new(1.5, 2.0, 1.5) });
		let player_node_id = state.nodes.insert(player_node);
		let inventory = Inventory::new(4);
		let player = player::Player::new(player_node_id, inventory);

		let model_node_id = load_model("assets/orkki.glb", state);
		let node = state.nodes.get_mut(&model_node_id).unwrap();
		node.parent = NodeParent::Node(player.node_id);
		Npc::new(player)
	}
}