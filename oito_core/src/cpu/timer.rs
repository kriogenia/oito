/// Abstraction of both timers of the Chip8 CPU
pub struct Timer {
	/// Current count of the timer
	count: u8
}

impl Timer {

	/// Decreases the timer value
	pub fn decrease(&mut self) {
		if self.count > 0 {
			self.count -= 1;
		}
	}

}

impl Default for Timer {
    fn default() -> Self {
        Self { count: Default::default() }
    }
}

#[cfg(test)]
mod test {
    use super::Timer;

	#[test]
	fn decrease() {
		let mut timer = Timer { count: 2 };
		timer.decrease();
		assert_eq!(1, timer.count);
		timer.decrease();
		assert_eq!(0, timer.count);
		timer.decrease();
		assert_eq!(0, timer.count);
	}

}