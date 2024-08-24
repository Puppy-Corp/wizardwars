
pub struct PlayerController {
	node_id: pge::ArenaId<pge::Node>,
	camera_id: pge::ArenaId<pge::Camera>
}

impl PlayerController {
	pub fn new(state: &mut pge::State, node_id: pge::ArenaId<pge::Node>) -> Self {
		let mut camera = pge::Camera::new();
		camera.zfar = 1000.0;
		camera.node_id = Some(node_id);
		let camera_id = state.cameras.insert(camera);

		Self {
			node_id,
			camera_id
		}
	}
}