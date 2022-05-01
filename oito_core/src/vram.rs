use std::fmt::Debug;

use crate::{Pixel, SCREEN_WIDTH, SCREEN_HEIGHT};

const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

/// Representation of the screen to draw
pub struct VRam {
    /// Buffer of the current visual content
    buffer: [Pixel; SCREEN_SIZE],
}

impl VRam {
    /// Value representing the a black pixel
    pub const BLACK: bool = false;
    /// Value representing the a white pixel
    pub const WHITE: bool = true;

	pub fn buffer(&self) -> &[Pixel] {
		&self.buffer
	} 

    /// Clears the current buffered content
    pub fn clear(&mut self) {
        self.buffer = [Self::BLACK; SCREEN_SIZE];
    }

    /// Paints over the pixel.
	/// If the coordinates overflow the screen space, it will be drawn counting the overflow from the start.
	/// If this already painted, it sets the pixel to not painted.
    pub fn paint(&mut self, x: usize, y: usize) {
		self.buffer[Self::to_index(x, y)] ^= VRam::WHITE;
    }

	/// Returns the content of the pixel at the specified location
    pub fn get(&mut self, x: usize, y: usize) -> Pixel {
        self.buffer[Self::to_index(x, y)]
    }

	/// Converts the coordinates into the index position in the buffer
	fn to_index(x: usize, y: usize) -> usize {
		x % SCREEN_WIDTH + (y % SCREEN_HEIGHT) * SCREEN_WIDTH
	}

}

impl Debug for VRam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "VRAM: ").unwrap();
		for i in 0..SCREEN_HEIGHT {
			for j in 0..SCREEN_WIDTH {
				write!(f, "{}", if self.buffer[i * SCREEN_WIDTH + j] { "X" } else { "_" }).unwrap();
			}
			writeln!(f).unwrap();
		}
		write!(f, "")
    }
}

impl Default for VRam {
    fn default() -> Self {
        Self {
            buffer: [Self::BLACK; SCREEN_SIZE],
        }
    }
}

#[cfg(test)]
mod test {
    use super::{VRam, SCREEN_SIZE};

    #[test]
    fn clear() {
        let mut vram = VRam {
            buffer: [VRam::WHITE; SCREEN_SIZE],
        };

        vram.clear();
        assert_eq!([VRam::BLACK; SCREEN_SIZE], vram.buffer);
    }
}
