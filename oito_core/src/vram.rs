const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

type Pixel = bool;				// only b&w, so bool is enough
/// Value representing the a black pixel
const BLACK: bool = false;
/// Value representing the a white pixel
const _WHITE: bool = true;

/// Representation of the screen to draw
pub struct VRam {
	/// Buffer of the current visual content
	buffer: [Pixel; SCREEN_SIZE ]	
}

impl Default for VRam {
    fn default() -> Self {
        Self { buffer: [ BLACK; SCREEN_SIZE ] }
    }
}