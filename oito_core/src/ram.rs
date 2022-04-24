use crate::Bit;

/// 4KB of RAM
const RAM_SIZE: usize = 4096; 
const EMPTY_MEM: Bit = 0;

/// Simmulated RAM
pub struct Ram {
	/// Buffer with the memory mantained by the RAM
	memory: [Bit; RAM_SIZE]
}

impl Default for Ram {
    fn default() -> Self {
        Self { memory: [ EMPTY_MEM; RAM_SIZE ] }
    }
}