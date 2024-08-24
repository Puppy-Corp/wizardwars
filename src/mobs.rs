use std::os::macos::raw::stat;

use pge::cube;
use pge::ArenaId;
use pge::Node;
use pge::NodeParent;
use pge::Scene;
use pge::State;
use pge::Vec3;

use crate::npc::Npc;
use crate::player;
use crate::player::PlayerBuilder;
use crate::utility::load_model;

// pub fn spawn_mob(state: &mut State, main_scene_id: ArenaId<Scene>, location: Vec3) -> Npc {
// 	let path 

// 	let player = PlayerBuilder::new(main_scene_id).build(state);
// 	Npc::new(player)
// }

pub struct MobSpawner {
	main_scene_id: ArenaId<Scene>,
	model_node: ArenaId<Node>
}

impl MobSpawner {
	pub fn new(state: &mut State, main_scene_id: ArenaId<Scene>) -> Self {
		let node_id = load_model("assets/orkki.glb", state);

		Self {
			main_scene_id,
			model_node: node_id
		}
	}

	pub fn spawn(&mut self, state: &mut State, translation: Vec3) -> Npc {
		// let cube = cube(1.5);
		// let mesh_id = state.meshes.insert(cube);
		let player = PlayerBuilder::new(self.main_scene_id)
			.translation(translation)
			.build(state);
		let node_id = state.clone_node(self.model_node);
		let node = state.nodes.get_mut(&node_id).unwrap();
		// let node = state.nodes.get_mut(&player.node_id).unwrap();		
		// node.translation = location;
		// node.mesh = Some(mesh_id);
		node.parent = NodeParent::Node(player.node_id);
		node.physics.mass = 45.0;
		node.physics.typ = pge::PhycisObjectType::Dynamic;
		node.collision_shape = Some(pge::CollisionShape::Capsule {
			radius: 0.5,
			height: 2.5
		});
		Npc::new(player)
	}
}