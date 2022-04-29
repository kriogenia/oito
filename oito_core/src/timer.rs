use std::fmt::Debug;

/// Abstraction of both timers of the Chip8 CPU
#[derive(Default)]
pub struct Timer {
    /// Current count of the timer
    count: u8,
}

impl Timer {
    /// Decreases the timer value
    pub fn decrease(&mut self) {
        if self.count > 0 {
            self.count -= 1;
        }
    }

    #[cfg(test)]
    pub(crate) fn set(&mut self, value: u8) {
        self.count = value;
    }

    pub fn get(&self) -> u8 {
        self.count
    }
}

impl Debug for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Timer: [{}]", &self.count)
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
