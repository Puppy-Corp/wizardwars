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

	pub fn process(&mut self, state: &mut pge::State) {
		self.player.process(state);
	}
}