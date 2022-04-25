const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

type Pixel = bool; // only b&w, so bool is enough
/// Value representing the a black pixel
const BLACK: bool = false;
/// Value representing the a white pixel
const WHITE: bool = true;

/// Representation of the screen to draw
pub struct VRam {
    /// Buffer of the current visual content
    buffer: [Pixel; SCREEN_SIZE],
}

impl VRam {

	/// Clears the current buffered content
	pub fn clear(&mut self) {
		self.buffer = [BLACK; SCREEN_SIZE];
	}
}

impl Default for VRam {
    fn default() -> Self {
        Self {
            buffer: [BLACK; SCREEN_SIZE],
        }
    }
}

#[cfg(test)]
mod test {
	use super::{VRam, BLACK, WHITE, SCREEN_SIZE};

	#[test]
	fn clear() {
		let mut vram = VRam {
			buffer: [WHITE; SCREEN_SIZE]
		};

		vram.clear();
		assert_eq!([BLACK; SCREEN_SIZE], vram.buffer);
		
	}

}