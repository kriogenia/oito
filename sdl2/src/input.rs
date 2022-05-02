use oito_core::key::Key;
use sdl2::keyboard::Scancode;

pub fn map_key(scancode: Option<Scancode>) -> Option<Key> {
	if scancode.is_none() {
		return None;
	}
	match scancode.unwrap() {
		Scancode::Num1 => Some(Key::One),
		Scancode::Num2 => Some(Key::Two),
		Scancode::Num3 => Some(Key::Three),
		Scancode::Num4 => Some(Key::C),
		Scancode::Q => Some(Key::Four),
		Scancode::W => Some(Key::Five),
		Scancode::E => Some(Key::Six),
		Scancode::R => Some(Key::D),
		Scancode::A => Some(Key::Seven),
		Scancode::S => Some(Key::Eight),
		Scancode::D => Some(Key::Nine),
		Scancode::F => Some(Key::E),
		Scancode::Z => Some(Key::A),
		Scancode::X => Some(Key::Zero),
		Scancode::C => Some(Key::B),
		Scancode::V => Some(Key::F),
		_ => None
	}
}