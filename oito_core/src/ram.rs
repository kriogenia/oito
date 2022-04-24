/// 4KB of RAM
const RAM_SIZE: usize = 4096; 

/// Simmulated RAM
pub struct Ram {
	/// Buffer with the memory mantained by the RAM
	memory: [u8; RAM_SIZE]
}