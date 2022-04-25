pub mod core;
pub(crate) mod exception;

mod cpu;
mod instruction;
mod ram;
mod stack;
mod timer;
mod vram;

/// Specification of the address type to correctly indicate when it's being used
pub type Address = u16;
/// Specification of the opcode type to correctly indicate when it's being used
pub type OpCode = u16; // each Chip8 instruction is made of two bytes
/// Specification of the bit type to correctly indicate what it's built upon bits
pub type Byte = u8;
