pub mod core;
pub mod key;

pub(crate) mod exception;
pub(crate) mod fontset;

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
/// Specification of the type for register indices
pub type RegIndex = u8;
/// Specification of the type to represent the pixels that will be drawn in the buffer
pub type Pixel = bool;
/// Specification of the type representing the ROMs to run
pub type Rom<'a> = &'a [u8];

/// Types that can be masked
pub trait BitMask {
    const LEAST_SIGNIFICANT_BIT: Self;
    const MOST_SIGNIFICANT_BIT: Self;
}

impl BitMask for Byte {
    const LEAST_SIGNIFICANT_BIT: Self = 0b00000001;
    const MOST_SIGNIFICANT_BIT: Self = 0b10000000;
}

/// Width of the emmulated screen
pub const SCREEN_WIDTH: usize = 64;

/// Height of the emmulated screen
pub const SCREEN_HEIGHT: usize = 32;