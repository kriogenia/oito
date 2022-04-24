const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

type Pixel = bool;		// only b&w, so bool is enough

/// Representation of the screen to draw
pub struct VRam {
	/// Buffer of the current visual content
	buffer: [Pixel; SCREEN_WIDTH * SCREEN_HEIGHT]	
}