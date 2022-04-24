mod cpu;
mod ram;
mod vram;

pub(crate) mod exception;

use cpu::Cpu;
use ram::Ram;
use vram::VRam;

/// Specification of the address type to correctly indicate when it's being used
pub type Address = u16;
/// Specification of the opcode type to correctly indicate when it's being used
pub type OpCode = u16;		// each Chip8 instruction is made of two bytes
/// Specification of the bit type to correctly indicate what it's built upon bits
pub type Byte = u8;

/// Core of the emmulator
pub struct OitoCore {
	/// Emmulated CPU
	cpu: Cpu,
	/// Current frame to draw
	vram: VRam,
	/// Emmulated RAM
	ram: Ram,
}

impl OitoCore {

	/// Returns a new instance of the emulator core
	pub fn new() -> Self {
		Self::default()
	}

}

impl Default for OitoCore {
    fn default() -> Self {
        Self { 
			cpu: Default::default(), 
			vram: Default::default(), 
			ram: Default::default() 
		}
    }
}