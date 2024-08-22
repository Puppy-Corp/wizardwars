use pge::State;

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
}