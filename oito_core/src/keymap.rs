use std::ops::Index;

use crate::Byte;

const NUMBER_OF_KEYS: usize = 16;

pub struct KeyMap {
	key_pressed: [ bool; NUMBER_OF_KEYS ]
}

impl KeyMap {

	#[cfg(test)]
	pub fn press_key(&mut self, index: usize) {
		self.key_pressed[index] = true;
	}

}

impl Default for KeyMap {
    fn default() -> Self {
        Self { key_pressed: [ false; NUMBER_OF_KEYS ] }
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
	fn index() {
		let mut map = KeyMap::default();
		map.key_pressed[0] = true;

		assert!(map[0]);
	}

}