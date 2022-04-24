mod stack;
mod timer;
mod register;

use register::{VRegister, IRegister};
use stack::Stack;
use timer::Timer;

use crate::Address;

const NUMBER_OF_REGISTERS: usize = 16;
const STARTING_ADDRESS: Address = 0x200;

/// Simmulated CPU
pub struct Cpu {
	/// Program Counter
	pc: Address,
	/// V-Registers
	vreg: [VRegister; NUMBER_OF_REGISTERS],
	/// Stack
	stack: Stack,
	/// I-Register
	iref: IRegister,
	/// Delay timer
	dt: Timer,
	/// Sound timer
	st: Timer,
}

impl Default for Cpu {
    fn default() -> Self {
        Self { 
			pc: STARTING_ADDRESS, 
			vreg: Default::default(), 
			stack: Default::default(), 
			iref: Default::default(), 
			dt: Default::default(), 
			st: Default::default() 
		}
    }
}