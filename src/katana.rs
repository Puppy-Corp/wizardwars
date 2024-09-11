use std::f32::consts::PI;
use pge::*;

use crate::types::Item;
use crate::utility::load_model;

pub struct Katana {
    node_id: ArenaId<Node>,
    attacking: bool,

	x_rotation: f32,
	y_rotation: f32,
	z_rotation: f32,
    rotation_speed: f32, // Add a rotation speed factor
}

impl Katana {
    pub fn new(state: &mut State, scene_id: ArenaId<Scene>) -> Self {
        let node_id = load_model("assets/katana.glb", state);
        let node = state.nodes.get_mut(&node_id).unwrap();
        node.translation = Vec3::new(0.3, -1.0, 3.0);
        // node.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 220.0_f32.to_radians());
        Self {
            node_id,
            attacking: false,
            x_rotation: 0.0,
			y_rotation: 90.0_f32.to_radians(),
			z_rotation: 0.0,
            rotation_speed: 10.0, // Initialize the rotation speed
        }
    }
}

impl Item for Katana {
    fn prepare(&mut self, state: &mut pge::State) {}

    fn activate(&mut self, state: &mut pge::State, parent_id: ArenaId<Node>) {
        let node = state.nodes.get_mut(&self.node_id).unwrap();
        node.parent = NodeParent::Node(parent_id);
    }

    fn hide(&mut self, state: &mut pge::State) {
        let node = state.nodes.get_mut(&self.node_id).unwrap();
        node.parent = NodeParent::Orphan;
    }

    fn start_primary_action(&mut self, state: &mut State) {
        log::info!("start shooting");
        self.attacking = true;
		self.x_rotation = 0.0;
		self.y_rotation = 180.0_f32.to_radians();
		//self.y_rotation = 90.0_f32.to_radians();
		self.z_rotation = 0.0;
    }

    fn stop_primary_action(&mut self, state: &mut State) {
        log::info!("stop shooting");
        self.attacking = false;
		self.y_rotation = 0.0;
		self.x_rotation = 90.0_f32.to_radians();
		//self.z_rotation = 220.0_f32.to_radians();
		self.z_rotation = 0.0;
    }

    fn process(&mut self, state: &mut State, dt: f32) {
        let node = state.nodes.get_mut(&self.node_id).unwrap();
		let target_rotation = Quat::from_euler(EulerRot::YXZ, self.x_rotation, self.y_rotation, self.z_rotation);
        let new_rotation = node.rotation.slerp(target_rotation, dt * self.rotation_speed);
        node.rotation = new_rotation;
    }

	fn on_mouse_moved(&mut self, dx: f32, dy: f32, state: &mut State) {
		if !self.attacking {
			return;
		}
		// if dx > 0.0 {
		// 	self.x_rotation = 90.0_f32.to_radians();
		// } else if dx < 0.0 {
		// 	self.x_rotation = 270.0_f32.to_radians();
		// }
	}
}
