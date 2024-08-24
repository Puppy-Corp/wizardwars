use pge::Quat;

use crate::player;
use crate::player::Player;


pub struct Npc {
	pub player: Player
}

impl Npc {
	pub fn new(player: Player) -> Self {
		Self {
			player
		}
	}

	pub fn process(&mut self, state: &mut pge::State, enemy: &Player) {
		
		// let node = state.nodes.get_mut(&self.player.node_id).unwrap();
		// self.player.rotate(10.0, 0.0);
		// 

		let translation = state.nodes.get(&enemy.node_id).unwrap().translation;
		let player_node = state.nodes.get_mut(&self.player.node_id).unwrap();
		log::info!("look at me {:?}", translation);
		player_node.looking_at(translation.x, translation.y, translation.z);
		let dir = translation - player_node.translation;
		player_node.translation += dir.normalize() * 0.1;
		self.player.process(state);
	}
}