/// Abstraction of both timers of the Chip8 CPU
pub struct Timer {
	/// Current count of the timer
	count: u8
}

impl Default for Timer {
    fn default() -> Self {
        Self { count: Default::default() }
    }
}