pub use pge::*;

pub fn load_model(path: &str, state: &mut pge::State) -> ArenaId<Node> {
	let node = Node::new();
	let node_id = state.nodes.insert(node);
	let model_id = state.load_3d_model(path);
	let model = state.models.get(&model_id).unwrap();
	let scene_id = model.scenes[0];
	for (_, node) in &mut state.nodes.iter_mut() {
		if node.parent == NodeParent::Scene(scene_id) {
			node.parent = NodeParent::Node(node_id);
		}
	}

	node_id
}

#[derive(Debug, Clone)]
pub struct PressedKeys {
	pub forward: bool,
	pub backward: bool,
	pub left: bool,
	pub right: bool,
}

impl PressedKeys {
	pub fn new() -> Self {
		Self {
			forward: false,
			backward: false,
			left: false,
			right: false,
		}
	}

	pub fn to_vec3(&self) -> Vec3 {
        let mut direction = Vec3::ZERO;

        if self.forward {
            direction += Vec3::Z;
        }
        if self.backward {
            direction -= Vec3::Z;
        }
        if self.left {
            direction -= Vec3::X;
        }
        if self.right {
            direction += Vec3::X;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        direction
    }

	pub fn any_pressed(&self) -> bool {
		self.forward || self.backward || self.left || self.right
	}
}