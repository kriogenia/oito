use std::fmt::Debug;

use crate::{exception::Exception, Address, Byte};

/// 4KB of RAM
const RAM_SIZE: usize = 4096;
const EMPTY_MEM: Byte = 0;

/// Simmulated RAM
pub struct Ram {
    /// Buffer with the memory mantained by the RAM
    memory: [Byte; RAM_SIZE],
}

impl Ram {
    /// Loads the slice starting in the specified position
    pub fn load(&mut self, start: Address, content: &[Byte]) {
        let start = start as usize;
        self.memory[start..start + content.len()].copy_from_slice(content);
    }

    /// Returns the content of the specified address
    pub fn read(&self, address: Address) -> Result<Byte, Exception> {
        let i = address as usize;
        if i >= RAM_SIZE {
            Err(Exception::SegmentationFault(address))
        } else {
            Ok(self.memory[address as usize])
        }
    }

    #[cfg(test)]
    pub(crate) fn set(&mut self, address: Address, value: Byte) {
        self.memory[address as usize] = value;
    }
}

impl Debug for Ram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..16 {
            for j in 0..(RAM_SIZE / 16) {
                write!(f, "{:?}", self.memory[i * 16 + j]).unwrap();
            }
            writeln!(f).unwrap();
        }
        write!(f, "")
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self {
            memory: [EMPTY_MEM; RAM_SIZE],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::exception::Exception;

    use super::Ram;

    #[test]
    fn load() {
        let mut ram = Ram::default();

        ram.load(0x10, &[0xFF, 0xAA]);
        assert_eq!(0xFF, ram.read(0x10).unwrap());
        assert_eq!(0xAA, ram.read(0x11).unwrap());
        assert_eq!(0x00, ram.read(0x12).unwrap());
    }

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
        assert_eq!(
            Ram::default().read(5000).unwrap_err(),
            Exception::SegmentationFault(5000)
        );
    }
}
