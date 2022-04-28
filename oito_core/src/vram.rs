pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

type Pixel = bool; // only b&w, so bool is enough

use std::ops::Index;

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

    /// Clears the current buffered content
    pub fn clear(&mut self) {
        self.buffer = [Self::BLACK; SCREEN_SIZE];
    }

	/// Paints over the pixel. If this already painted, it sets the pixel to not painted.
    pub fn paint(&mut self, index: usize) {
        self.buffer[index] ^= VRam::WHITE;
    }

    #[cfg(test)]
    pub(crate) fn set(&mut self, index: usize) {
        self.buffer[index] = VRam::WHITE;
    }
}

impl Default for VRam {
    fn default() -> Self {
        Self {
            buffer: [Self::BLACK; SCREEN_SIZE],
        }
    }
}

impl Index<usize> for VRam {
    type Output = Pixel;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
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
