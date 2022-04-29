use std::ops::Index;

use crate::Byte;

const NUMBER_OF_KEYS: usize = 16;

/// Mapping of the keys and their state as pressed or not pressed
#[derive(Debug)]
pub struct KeyMap {
    key_pressed: [bool; NUMBER_OF_KEYS],
}

impl KeyMap {

	/// Returns the first key pressed if any
	pub fn get_key_pressed(&self) -> Option<usize> {
		for (i, key) in self.key_pressed.into_iter().enumerate() {
			if key {
				return Some(i)
			}
		}
		None
	}

    #[cfg(test)]
    pub fn press_key(&mut self, index: usize) {
        self.key_pressed[index] = true;
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        Self {
            key_pressed: [false; NUMBER_OF_KEYS],
        }
    }
}

impl Index<Byte> for KeyMap {
    type Output = bool;

    fn index(&self, index: Byte) -> &Self::Output {
        &self.key_pressed[index as usize]
    }
}

#[cfg(test)]
mod test {
    use super::KeyMap;

	#[test]
	fn get_key_pressed() {
		let mut map = KeyMap::default();
		// No key pressed
		assert!(map.get_key_pressed().is_none());
		// Key pressed, get lowest
		map.press_key(5);
		map.press_key(2);
		assert_eq!(2, map.get_key_pressed().unwrap());
	}

    #[test]
    fn index() {
        let mut map = KeyMap::default();
        map.key_pressed[0] = true;

        assert!(map[0]);
    }
}
