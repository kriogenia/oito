mod stack;
mod timer;
mod register;

use register::{VRegister, IRegister};
use stack::Stack;
use timer::Timer;

use crate::{Address, OpCode, exception::Exception, Byte};

const INSTRUCTION_SIZE: u16 = 2;
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

impl Cpu {

	/// Performs a tick of the CPU
	pub fn tick<M>(&mut self, memory_access: M) 
	where M: Fn(Address) -> Result<Byte, Exception> {
		let _opcode = self.fetch(self.pc, memory_access);
		// Decode instruction
		// Execute instruction
	}

	/// Reads from memory the next instruction and points to the next one
	fn fetch<M>(&mut self, address: Address, read: M) -> Result<OpCode, Exception> 
	where M: Fn(Address) -> Result<Byte, Exception> {
		let big_byte = read(address)? as u16;
		let small_byte = read(address + 1)? as u16;
		let opcode = (big_byte << 8) | small_byte;
		
		self.increase();
		Ok(opcode)
	}

	/// Increases the Program Counter to point to the next instruction
	fn increase(&mut self) {
		self.pc += INSTRUCTION_SIZE;
	}

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

#[cfg(test)]
mod test {
    use super::{ STARTING_ADDRESS, Cpu };


	#[test]
	fn fetch() {
		let mut cpu = Cpu::default();
		let opcode = cpu.fetch(1, |address| { Ok(address as u8) });
		assert_eq!(0x0102, opcode.unwrap());
		assert_eq!(STARTING_ADDRESS + 2, cpu.pc);
	}

	#[test]
	fn increase() {
		let mut cpu = Cpu::default();
		assert_eq!(STARTING_ADDRESS, cpu.pc);
		for i in 1..4 {
			cpu.increase();
			assert_eq!(i * 2 + STARTING_ADDRESS, cpu.pc);
		}
	}


}