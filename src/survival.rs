use pge::*;

use crate::player::Player;
use crate::utility::PressedKeys;

pub struct Survival {
	player: Player,
	pressed_keys: PressedKeys,
}

impl Survival {
	pub fn new(state: &mut State) -> Self {
		Self {
			player: Player::spawn(state),
			pressed_keys: PressedKeys::new(),
		}
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

	}
}