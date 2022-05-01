use std::{fmt::Debug, ops::Index};

use crate::{key::Key, Byte};

/// Mapping of the keys and their state as pressed or not pressed
pub struct KeyMap {
    key_pressed: [bool; Key::SIZE],
}

impl KeyMap {
    /// Returns the first key pressed if any
    pub fn get_key_pressed(&self) -> Option<usize> {
        for (i, key) in self.key_pressed.into_iter().enumerate() {
            if key {
                return Some(i);
            }
        }
        None
    }

    /// Marks the specified key as pressed
    pub fn press_key(&mut self, key: Key) {
        let index: usize = key.into();
        self.key_pressed[index] = true;
    }

    /// Marks the specified key as not pressed
    pub fn release_key(&mut self, key: Key) {
        let index: usize = key.into();
        self.key_pressed[index] = false;
    }
}

impl Debug for KeyMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KeyMap: [ ").unwrap();
        for (i, state) in self.key_pressed.iter().enumerate() {
            write!(f, "{i}: {}, ", if *state { "X" } else { "-" }).unwrap();
        }
        write!(f, " ]")
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        Self {
            key_pressed: [false; Key::SIZE],
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
    use crate::key::Key;

    use super::KeyMap;

    #[test]
    fn get_key_pressed() {
        let mut map = KeyMap::default();
        // No key pressed
        assert!(map.get_key_pressed().is_none());
        // Key pressed, get lowest
        map.key_pressed[5] = true;
        map.key_pressed[2] = true;
        assert_eq!(2, map.get_key_pressed().unwrap());
    }

    #[test]
    fn press_key() {
        let mut map = KeyMap::default();

        map.press_key(Key::Three);
        assert!(map.key_pressed[3]);
    }

    #[test]
    fn release_key() {
        let mut map = KeyMap::default();
        map.press_key(Key::Three);
        map.press_key(Key::Five);
        assert!(map.key_pressed[3]);
        assert!(map.key_pressed[5]);

        map.release_key(Key::Three);
        assert!(!map.key_pressed[3]);
        assert!(map.key_pressed[5]);
    }

    #[test]
    fn index() {
        let mut map = KeyMap::default();
        map.key_pressed[0] = true;

        assert!(map[0]);
    }
}
