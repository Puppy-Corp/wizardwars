use pge::*;
use crate::inventory::Inventory;

pub struct PlayerBuilder {
	inventory_size: usize,
	team: u32,
	mana: u32,
	health: u32
}

impl PlayerBuilder {
	pub fn new() -> Self {
		Self {
			inventory_size: 10,
			team: 0,
			mana: 100,
			health: 100
		}
	}

	pub fn team(mut self, team: u32) -> Self {
		self.team = team;
		self
	}

	pub fn inventory_size(mut self, size: usize) -> Self {
		self.inventory_size = size;
		self
	}

	pub fn mana(mut self, mana: u32) -> Self {
		self.mana = mana;
		self
	}

	pub fn health(mut self, health: u32) -> Self {
		self.health = health;
		self
	}

	pub fn build(self, state: &mut State) -> Player {
		let node = Node::new();
		let node_id = state.nodes.insert(node);

		Player {
			node_id,
			crouching: false,
			inventory: Inventory::new(self.inventory_size),
			spriting: false,
			jumping: false,
			prone: false,
			mana: 100,
			team: self.team,
			death: false
		}
	}
}

pub struct Player {
	node_id: ArenaId<Node>,
	mana: u32,
	pub inventory: Inventory,
	spriting: bool,
	pub jumping: bool,
	crouching: bool,
	prone: bool,
	team: u32,
	pub death: bool
}

impl Player {
	// pub fn spawn(state: &mut State) -> Self {
	// 	let node = Node::new();
	// 	let node_id = state.nodes.insert(node);

	// 	Self {
	// 		node_id,
	// 		mana: 100,
	// 		inventory: Inventory::new(),
	// 		spriting: false,
	// 		jumping: false,
	// 		crouching: false,
	// 		prone: false,
	// 		team: 0
	// 	}
	// }

	pub fn process(&mut self, state: &mut State) {

	}

	pub fn equip(&mut self, index: usize, state: &mut State) {
		self.inventory.equip(index, state, self.node_id);
	}

	pub fn switch_item(&mut self, state: &mut State, index: usize) {
		self.inventory.equip(index, state, self.node_id);
	}

	pub fn start_primary_action(&mut self, state: &mut State) {
		if let Some(item) = self.inventory.get_current_item() {
			item.start_primary_action(state);
		}
	}

	pub fn stop_primary_action(&mut self, state: &mut State) {
		if let Some(item) = self.inventory.get_current_item() {
			item.stop_primary_action(state);
		}
	}

	pub fn start_secondary_action(&mut self, state: &mut State) {
		if let Some(item) = self.inventory.get_current_item() {
			item.start_secondary_action(state);
		}
	}

	pub fn stop_secondary_action(&mut self, state: &mut State) {
		if let Some(item) = self.inventory.get_current_item() {
			item.stop_secondary_action(state);
		}
	}

	pub fn start_third_action(&mut self, state: &mut State) {
		if let Some(item) = self.inventory.get_current_item() {
			item.start_third_action(state);
		}
	}

	pub fn stop_third_action(&mut self, state: &mut State) {
		if let Some(item) = self.inventory.get_current_item() {
			item.stop_third_action(state);
		}
	}

	pub fn drop(&mut self, state: &mut State) {
		self.inventory.drop(state);
	}

	pub fn start_grap(&mut self, state: &mut State) {
		// self.inventory.grap(state);
	}

	pub fn stop_grap(&mut self, state: &mut State) {
		// self.inventory.stop_grap(state);
	}
	
	pub fn start_sprinting(&mut self, state: &mut State) {
		self.spriting = true;
	}

	pub fn stop_sprinting(&mut self, state: &mut State) {
		self.spriting = false;
	}

	pub fn start_jumping(&mut self) {
		self.jumping = true;
	}

	pub fn stop_jumping(&mut self, state: &mut State) {
		self.jumping = false;
	}

	pub fn start_crouching(&mut self, state: &mut State) {
		self.crouching = true;
	}

	pub fn stop_crouching(&mut self, state: &mut State) {
		self.crouching = false;
	}

	pub fn start_prone(&mut self, state: &mut State) {
		self.prone = true;
	}

	pub fn stop_prone(&mut self, state: &mut State) {
		self.prone = false;
	}

	pub fn rotate(&mut self, dx: f32, fy: f32) {
		
	}
}