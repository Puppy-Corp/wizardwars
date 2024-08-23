use pge::State;
use pge::Vec3;

use crate::npc::Npc;
use crate::player::PlayerBuilder;

pub fn spawn_mob(state: &mut State, location: Vec3) -> Npc {
	let player = PlayerBuilder::new().build(state);
	Npc::new(player)
}