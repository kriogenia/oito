use crate::{Byte, Address, exception::Exception};

/// 4KB of RAM
const RAM_SIZE: usize = 4096; 
const EMPTY_MEM: Byte = 0;

/// Simmulated RAM
pub struct Ram {
	/// Buffer with the memory mantained by the RAM
	memory: [Byte; RAM_SIZE]
}

impl Ram {
	pub fn read(&self, address: Address) -> Result<Byte, Exception> {
		let i = address as usize;
		if i >= RAM_SIZE {
			Err(Exception::SegmentationFault)
		} else {
			Ok(self.memory[address as usize])
		}
	}
}

impl Default for Ram {
    fn default() -> Self {
        Self { memory: [ EMPTY_MEM; RAM_SIZE ] }
    }
}

#[cfg(test)]
mod test {
    use crate::exception::Exception;

    use super::Ram;

	#[test]
	fn read() {
		let mut ram = Ram::default();
		for i in 0..4 {
			ram.memory[i] = i as u8;
		}
		for i in 0..4 {
			assert_eq!(i, ram.read(i as u16).unwrap());
		}
	}

	#[test]
	fn seg_fault() {
		assert_eq!(Ram::default().read(5000).unwrap_err(), Exception::SegmentationFault);
	}

}