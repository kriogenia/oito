pub(crate) use keymap::KeyMap;

mod keymap;

/// Keys in the Chip8 system
pub enum Key {
	Zero,
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	A,
	B,
	C,
	D,
	E,
	F,
}

impl Key {
	pub const SIZE: usize = 16;
}

impl Into<usize> for Key {
    fn into(self) -> usize {
        use Key::*;
		match self {
			Zero => 0x0,
			One => 0x1,
			Two => 0x2,
			Three => 0x3,
			Four => 0x4,
			Five => 0x5,
			Six => 0x6,
			Seven => 0x7,
			Eight => 0x8,
			Nine => 0x9,
			A => 0xA,
			B => 0xB,
			C => 0xC,
			D => 0xD,
			E => 0xE,
			F => 0xF,
		}
    }
}
