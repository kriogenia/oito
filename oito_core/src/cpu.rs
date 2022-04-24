mod ireg;
mod stack;
mod timer;
mod vreg;

use ireg::IRegister;
use stack::Stack;
use timer::Timer;
use vreg::VRegister;

const NUMBER_OF_REGISTERS: usize = 16;

/// Making a whole entity of this would be going way overboard, but this allows me to see the CPU as a collection of named components
type ProgramCounter = u16;

/// Simmulated CPU
pub struct Cpu {
	/// Program Counter
	pc: ProgramCounter,
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