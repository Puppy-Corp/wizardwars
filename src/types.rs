use pge::MouseEvent;
use pge::State;
use pge::Vec3;

pub trait Item {
	fn prepare(&mut self, state: &mut State) {}
	fn activate(&mut self, state: &mut State, parent_id: pge::ArenaId<pge::Node>) {}
	fn hide(&mut self, state: &mut State) {}
	fn start_primary_action(&mut self, state: &mut State) {}
	fn stop_primary_action(&mut self, state: &mut State) {}
	fn start_secondary_action(&mut self, state: &mut State) {}
	fn stop_secondary_action(&mut self, state: &mut State) {}
	fn start_third_action(&mut self, state: &mut State) {}
	fn stop_third_action(&mut self, state: &mut State) {}
	fn process(&mut self, state: &mut State, dt: f32) {}
	fn on_mouse_moved(&mut self, dx: f32, dy: f32, state: &mut State) {}
}

pub trait SurvivalMap {
	fn get_mob_spawn_point(&mut self) -> Vec3;
	fn get_player_spawn_point(&mut self) -> Vec3;
	fn process(&mut self, state: &mut State);
}