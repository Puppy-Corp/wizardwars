use crate::types::SurvivalMap;

pub struct DarkDungeon {

}

impl SurvivalMap for DarkDungeon {
	fn get_mob_spawn_point(&self) -> pge::Vec3 {
		pge::Vec3::ZERO
	}

	fn get_player_spawn_point(&self) -> pge::Vec3 {
		pge::Vec3::ZERO
	}

	fn process(&mut self, state: &mut pge::State) {
		// Do nothing
	}
}