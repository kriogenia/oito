use oito_core::key::Key;

/// Returns the Oito Key related to the JS code
pub fn map_key(code: &str) -> Option<Key> {
	match code {
		"Digit1" => Some(Key::One),
		"Digit2" => Some(Key::Two),
		"Digit3" => Some(Key::Three),
		"Digit4" => Some(Key::C),
		"KeyQ" => Some(Key::Four),
		"KeyW" => Some(Key::Five),
		"KeyE" => Some(Key::Six),
		"KeyR" => Some(Key::D),
		"KeyA" => Some(Key::Seven),
		"KeyS" => Some(Key::Eight),
		"KeyD" => Some(Key::Nine),
		"KeyF" => Some(Key::E),
		"KeyZ" => Some(Key::A),
		"KeyX" => Some(Key::Zero),
		"KeyC" => Some(Key::B),
		"KeyV" => Some(Key::F),
		_ => None
	}
}