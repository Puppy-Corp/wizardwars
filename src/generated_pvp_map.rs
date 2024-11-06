use pge::*;
use rand::Rng;

use crate::types::SurvivalMap;


pub struct GeneratedPVPMap {
    rng: rand::rngs::ThreadRng,
}

impl GeneratedPVPMap {
	pub fn new(state: &mut pge::State, scene_id: ArenaId<Scene>, width: f32, height: f32) -> Self {
        let mut grid: Vec<bool> = Vec::with_capacity((width * height) as usize);

        let object_count = (width * height * 0.025) as usize;
        log::info!("Generating {} objects", object_count);
        let cube_mesh = cube(1.0);
        let cube_mesh = state.meshes.insert(cube_mesh);

        let mut rng = rand::thread_rng();
        let half_width = width / 2.0;
        let half_height = height / 2.0;
        // Initialize grid with false values
        grid.resize((width * height) as usize, false);

        for _ in 0..object_count {
            'placement: for _ in 0..5 {
                let x = rng.gen_range(-half_width as i32..half_width as i32);
                let z = rng.gen_range(-half_height as i32..half_height as i32);

                // Check if we can place a 3x3 grid centered on chosen coordinate
                let mut valid = true;
                'check_coords: for dx in -1..=1 {
                    for dz in -1..=1 {
                        let grid_x = (x + dx + half_width as i32) as usize;
                        let grid_z = (z + dz + half_height as i32) as usize;
                        
                        // Check if coordinates are within bounds
                        if grid_x >= width as usize || grid_z >= height as usize {
                            valid = false;
                            break 'check_coords;
                        }
                        
                        // Calculate index in flattened grid array
                        let index = grid_x + grid_z * width as usize;
                        
                        if grid[index] {
                            continue 'placement;
                        }
                    }
                    if !valid {
                        break;
                    }
                }

                if !valid {
                    continue;
                }

                // Mark grid cells as occupied
                'mark_grid: for dx in -1..=1 {
                    for dz in -1..=1 {
                        let grid_x = (x + dx + half_width as i32) as usize;
                        let grid_z = (z + dz + half_height as i32) as usize;
                        let index = grid_x + grid_z * width as usize;
                        grid[index] = true;
                    }
                }

                let mut box_node = Node::new();
                box_node.mesh = Some(cube_mesh);
                box_node.set_translation(x as f32, 2.0, z as f32);
                box_node.physics.typ = PhycisObjectType::Static;
                box_node.physics.mass = 10.0;
                box_node.collision_shape = Some(CollisionShape::Box { size: Vec3::new(1.0, 1.0, 1.0) });
                box_node.parent = NodeParent::Scene(scene_id);
                state.nodes.insert(box_node);
                break; // Successfully placed object, break inner loop
            }
        }

        let floor_mesh = plane(width as f32, height as f32);
        let floor_mesh = state.meshes.insert(floor_mesh);
        let mut floor_node = Node::new();
		floor_node.name = Some("Floor".to_string());
		floor_node.set_translation(0.0, 1.0, 0.0);
		floor_node.mesh = Some(floor_mesh);
		floor_node.parent = NodeParent::Scene(scene_id);
		floor_node.physics.typ = PhycisObjectType::Static;
		floor_node.collision_shape = Some(CollisionShape::Box { size: Vec3::new(width, 0.1, height) });
		state.nodes.insert(floor_node);

        //spawn random lights
		for i in 0..5 {
			let rand_x = rng.gen_range(-25.0..25.0);
			let rand_z = rng.gen_range(-25.0..25.0);
			let mut light_node = Node::new();
			light_node.translation = Vec3::new(rand_x, 45.0, rand_z);
			light_node.parent = NodeParent::Scene(scene_id);
			let light_node_id = state.nodes.insert(light_node);

			let mut light = PointLight::new();
			light.node_id = Some(light_node_id);
			light.color = [1.0, 1.0, 1.0];
			light.intensity = 1.0;
			light.node_id = Some(light_node_id);
			let light_id = state.point_lights.insert(light);
		}

		Self {
            rng,
		}
	}

    pub fn process(&mut self, state: &mut pge::State) {
        // Do nothing
    }
}

impl SurvivalMap for GeneratedPVPMap {
	fn get_mob_spawn_point(&mut self) -> pge::Vec3 {
		let x = self.rng.gen_range(-25.0..25.0);
		let z = self.rng.gen_range(-25.0..25.0);
		pge::Vec3::new(x, 10.0, z)
	}

	fn get_player_spawn_point(&mut self) -> pge::Vec3 {
		pge::Vec3::new(0.0, 10.0, 0.0)
	}

	fn process(&mut self, state: &mut pge::State) {
		// Do nothing
	}
}