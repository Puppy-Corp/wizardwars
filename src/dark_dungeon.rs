use pge::*;
use pge::Vec3;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::Rng;

use crate::types::SurvivalMap;

pub struct DarkDungeon {
	rng: ThreadRng
}

impl DarkDungeon {
	pub fn create(state: &mut pge::State, main_scene_id: ArenaId<Scene>) -> Self {
		let texture = Texture::new("assets/wall_medium.png");
		let texture_id = state.textures.insert(texture);
		let wall_material = Material {
			name: Some("wall_material".to_string()),
			base_color_texture: Some(texture_id),
			metallic_factor: 0.0,
			roughness_factor: 0.0,
			..Default::default()
		};
		let wall_material_id = state.materials.insert(wall_material);

		let size = 200.0;

		let mut wall_mesh = plane(size, size);
		// wall_mesh.tex_coords = vec![
		// 	[0.0, 0.0],
		// 	[1.0, 0.0],
		// 	[1.0, 1.0],
		// 	[0.0, 1.0],
		// ];
		wall_mesh.primitives[0].material = Some(wall_material_id);

		let mut forward_wall = Node::new();
		forward_wall.mesh = Some(state.meshes.insert(wall_mesh.clone()));
		forward_wall.translation = Vec3::new(0.0, 0.0, size);
		forward_wall.looking_at(0.0, 0.0, 0.0);
		// rotate 90 degrees
		forward_wall.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 1.5708, 0.0);
		forward_wall.collision_shape = Some(CollisionShape::Box { size: Vec3::new(size, size, 1.0) });
		forward_wall.physics.typ = PhycisObjectType::Static;
		forward_wall.parent = NodeParent::Scene(main_scene_id);
		state.nodes.insert(forward_wall);

		let mut back_wall = Node::new();
		back_wall.mesh = Some(state.meshes.insert(wall_mesh.clone()));
		back_wall.translation = Vec3::new(0.0, 0.0, -size);
		back_wall.looking_at(0.0, 0.0, 0.0);
		// rotate 90 degrees
		back_wall.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 1.5708, 0.0);
		back_wall.collision_shape = Some(CollisionShape::Box { size: Vec3::new(size, size, 1.0) });
		back_wall.physics.typ = PhycisObjectType::Static;
		back_wall.parent = NodeParent::Scene(main_scene_id);
		state.nodes.insert(back_wall);

		let mut left_wall = Node::new();
		left_wall.mesh = Some(state.meshes.insert(wall_mesh.clone()));
		left_wall.translation = Vec3::new(-size, 0.0, 0.0);
		left_wall.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 1.5708);
		left_wall.collision_shape = Some(CollisionShape::Box { size: Vec3::new(1.0, size, size) });
		left_wall.physics.typ = PhycisObjectType::Static;
		left_wall.parent = NodeParent::Scene(main_scene_id);
		// left_wall.looking_at(0.0, 0.0, 0.0);
		state.nodes.insert(left_wall);

		let mut top_wall = Node::new();
		top_wall.mesh = Some(state.meshes.insert(wall_mesh.clone()));
		top_wall.translation = Vec3::new(0.0, size, 0.0);
		top_wall.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 0.0);
		// top_wall.collision_shape = Some(CollisionShape::Box { size: Vec3::new(size, size, 1.0) });
		top_wall.physics.typ = PhycisObjectType::Static;
		top_wall.parent = NodeParent::Scene(main_scene_id);
		state.nodes.insert(top_wall);

		let mut right_wall = Node::new();
		right_wall.mesh = Some(state.meshes.insert(wall_mesh.clone()));
		right_wall.translation = Vec3::new(size, 0.0, 0.0);
		right_wall.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 1.5708);
		right_wall.collision_shape = Some(CollisionShape::Box { size: Vec3::new(1.0, size, size) });
		right_wall.physics.typ = PhycisObjectType::Static;
		right_wall.parent = NodeParent::Scene(main_scene_id);
		state.nodes.insert(right_wall);

		let orange_texture = Texture {
			name: "orange_texture".to_string(),
			source: TextureSource::Buffer {
				data: vec![138, 98, 25, 255],
				width: 1,
				height: 1
			},
			..Default::default()
		};
		let orange_texture_id = state.textures.insert(orange_texture);

		let ground_material = Material {
			name: Some("ground_material".to_string()),
			base_color_factor: [1.0, 1.0, 1.0, 1.0],
			base_color_texture: Some(orange_texture_id),
			..Default::default()
		};
		let ground_material_id = state.materials.insert(ground_material);

		let mut floor_mesh = plane(size, size);
		floor_mesh.primitives[0].material = Some(ground_material_id);
		let floor_mesh_id = state.meshes.insert(floor_mesh);
		

		let mut floor = Node::new();
		floor.translation = Vec3::new(0.0, -1.0, 0.0);
		floor.mesh = Some(floor_mesh_id);
		floor.collision_shape = Some(CollisionShape::Box { size: Vec3::new(size, 1.0, size) });
		floor.physics.typ = PhycisObjectType::Static;
		floor.parent = NodeParent::Scene(main_scene_id);
		state.nodes.insert(floor);

		let mut rng = rand::thread_rng();

		//spawn random lights
		for i in 0..5 {
			let rand_x = rng.gen_range(-25.0..25.0);
			let rand_z = rng.gen_range(-25.0..25.0);
			let mut light_node = Node::new();
			light_node.translation = Vec3::new(rand_x, 45.0, rand_z);
			light_node.parent = NodeParent::Scene(main_scene_id);
			let light_node_id = state.nodes.insert(light_node);

			let mut light = PointLight::new();
			light.node_id = Some(light_node_id);
			light.color = [1.0, 1.0, 1.0];
			light.intensity = 1.0;
			light.node_id = Some(light_node_id);
			let light_id = state.point_lights.insert(light);
		}

		Self {
			rng: thread_rng()
		}
	}
}

impl SurvivalMap for DarkDungeon {
	fn get_mob_spawn_point(&mut self) -> pge::Vec3 {
		let x = self.rng.gen_range(-25.0..25.0);
		let z = self.rng.gen_range(-25.0..25.0);
		pge::Vec3::new(x, 2.0, z)
	}

	fn get_player_spawn_point(&mut self) -> pge::Vec3 {
		pge::Vec3::new(0.0, 2.0, 0.0)
	}

	fn process(&mut self, state: &mut pge::State) {
		// Do nothing
	}
}