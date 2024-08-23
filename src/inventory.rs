use pge::ArenaId;
use pge::Node;

use crate::types::Item;


// pub trait InvetoryItem {
// 	fn prepare(&mut self, state: &mut pge::State) {}
// 	fn activate(&mut self, state: &mut pge::State, parent_id: ArenaId<Node>) {}
// 	fn deactivate(&mut self, state: &mut pge::State) {}
// 	fn start_using(&mut self, state: &mut pge::State) {}
// 	fn stop_using(&mut self, state: &mut pge::State) {}
// 	fn process(&mut self, state: &mut pge::State) {}
// }

pub struct Inventory {
	active: Option<usize>,
	items: Vec<Box<dyn Item>>,
}

impl Inventory{
	pub fn new(size: usize) -> Self {
		Self {
			active: None,
			items: Vec::new(),
		}
	}

	pub fn add_item<T: Item + 'static>(&mut self, item: T) {
		self.items.push(Box::new(item));
	}

	pub fn prepare(&mut self, state: &mut pge::State) {
		for item in &mut self.items {
			item.prepare(state);
		}
	}

	pub fn equip(&mut self, index: usize, state: &mut pge::State, parent_id: ArenaId<Node>) {
		if let Some(active) = self.active {
			self.items[active].hide(state);
		}

		let item = match self.items.get_mut(index) {
			Some(item) => item,
			None => {
				log::info!("No item at index {}", index);
				return;
			},
		};

		item.activate(state, parent_id);
		self.active = Some(index);
	}

	pub fn drop(&mut self, state: &mut pge::State) {
		
	}

	pub fn get_current_item(&mut self) -> Option<&mut dyn Item> {
		match self.active {
			Some(index) => Some(&mut *self.items[index]),
			None => None,
		}
	}
}