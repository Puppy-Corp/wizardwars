use std::time::Instant;

use pge::*;

use crate::dark_dungeon::DarkDungeon;
use crate::mobs::spawn_mob;
use crate::npc::Npc;
use crate::player::Player;
use crate::player::PlayerBuilder;
use crate::types::SurvivalMap;
use crate::utility::PressedKeys;

pub struct Survival {
	player: Player,
	wave: u32,
	pressed_keys: PressedKeys,
	enemies: Vec<Npc>,
	max_enemies: u32,
	enemies_spawned: u32,
	since_last_spawn: Instant,
	map: Box<dyn SurvivalMap>
}

impl Survival {
	pub fn new(state: &mut State) -> Self {
		Self {
			wave: 0,
			player: PlayerBuilder::new().build(state),
			pressed_keys: PressedKeys::new(),
			enemies: Vec::new(),
			max_enemies: 10,
			enemies_spawned: 0,
			since_last_spawn: Instant::now(),
			map: Box::new(DarkDungeon {})
		}
	}

	fn start_next_wave(&mut self, state: &mut State) {
		// How to despawn enemies ??
		self.wave += 1;
		self.max_enemies += 5;
		self.enemies_spawned = 0;
	}

	pub fn on_mouse_input(&mut self, event: MouseEvent, state: &mut State) {
		match event {
			MouseEvent::Moved { dx, dy } => self.player.rotate(dx, dy),
			MouseEvent::Pressed { button } => {
				match button {
					MouseButton::Left => self.player.start_primary_action(state),
					MouseButton::Right => self.player.start_secondary_action(state),
					MouseButton::Middle => self.player.start_third_action(state),
					_ => {}
				}
			},
			MouseEvent::Released { button } => {
				match button {
					MouseButton::Left => self.player.stop_primary_action(state),
					MouseButton::Right => self.player.stop_secondary_action(state),
					MouseButton::Middle => self.player.stop_third_action(state),
					_ => {}
				}
			},
			_ => {}
		}
	}

	pub fn on_keyboard_input(&mut self, key: KeyboardKey, action: KeyAction, state: &mut State) {
		match action {
			KeyAction::Pressed => {
				match key {
					KeyboardKey::W => self.pressed_keys.forward = true,
					KeyboardKey::S => self.pressed_keys.backward = true,
					KeyboardKey::A => self.pressed_keys.left = true,
					KeyboardKey::D => self.pressed_keys.right = true,
					KeyboardKey::ShiftLeft => self.player.start_sprinting(state),
					KeyboardKey::Space => self.player.jumping = true,
					KeyboardKey::G => self.player.drop(state),
					KeyboardKey::F => self.player.start_grap(state),
					KeyboardKey::Digit1 => self.player.equip(0, state),
					KeyboardKey::Digit2 => self.player.equip(1, state),
					KeyboardKey::Digit3 => self.player.equip(2, state),
					KeyboardKey::Digit4 => self.player.equip(3, state),
					KeyboardKey::Digit5 => self.player.equip(4, state),
					KeyboardKey::Digit6 => self.player.equip(5, state),
					_ => {}
				}
			},
			KeyAction::Released => {
				match key {
					KeyboardKey::W => self.pressed_keys.forward = false,
					KeyboardKey::S => self.pressed_keys.backward = false,
					KeyboardKey::A => self.pressed_keys.left = false,
					KeyboardKey::D => self.pressed_keys.right = false,
					KeyboardKey::ShiftLeft => self.player.stop_sprinting(state),
					KeyboardKey::Space => self.player.jumping = false,
					KeyboardKey::F => self.player.stop_grap(state),
					_ => {}
				}
			},
		};
	}

	pub fn on_process(&mut self, state: &mut State, delta: f32) {
		self.player.process(state);
		let mut all_enemies_dead = true;
		for enemy in &mut self.enemies {
			enemy.process(state);
			if !enemy.player.death {
				all_enemies_dead = false;
			}
		}

		if all_enemies_dead && self.enemies_spawned >= self.max_enemies {
			self.start_next_wave(state);
		}

		if self.enemies_spawned < self.max_enemies {
			let time_since_last_spawn = self.since_last_spawn.elapsed().as_secs_f32();
			if time_since_last_spawn > 2.0 {
				self.enemies.push(spawn_mob(state, self.map.get_mob_spawn_point()));
				self.enemies_spawned += 1;
				self.since_last_spawn = Instant::now();
			}
		}
		self.map.process(state);
	}
}