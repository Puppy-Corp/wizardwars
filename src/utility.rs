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

pub fn get_root_node(state: &State, node_id: ArenaId<Node>) -> ArenaId<Node> {
	let mut stack = vec![node_id];
	while let Some(node_id) = stack.pop() {
		match state.nodes.get(&node_id) {
			Some(node) => {
				match node.parent {
						NodeParent::Node(n) => stack.push(n),
						NodeParent::Scene(_) => return node_id,
						NodeParent::Orphan => return node_id,
					}
			},
			None => {
				return node_id;
			},
		};
	};
	node_id
}

#[derive(Debug, Clone)]
pub struct MoveDirection {
	pub forward: bool,
	pub backward: bool,
	pub left: bool,
	pub right: bool,
}

impl MoveDirection {
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

	pub fn is_moving(&self) -> bool {
		self.forward || self.backward || self.left || self.right
	}
}