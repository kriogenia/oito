mod register;

use register::{IRegister, VRegister};

use crate::{Address, Byte, RegIndex, core::operations::BitOp};

const INSTRUCTION_SIZE: u16 = 2;
const NUMBER_OF_REGISTERS: usize = 16;

/// Simmulated CPU
pub struct Cpu {
    /// Program Counter
    pub pc: Address,
    /// V-Registers
    vreg: [VRegister; NUMBER_OF_REGISTERS],
    /// I-Register
    ireg: IRegister,
}

impl Cpu {
    /// Program counter starting address
    pub const STARTING_ADDRESS: Address = 0x200;

    /// Increases the Program Counter to point to the next instruction
    pub fn increase(&mut self) {
        self.pc += INSTRUCTION_SIZE;
    }

    /// Points the Program Counter to the specified address
    pub fn point_at(&mut self, position: Address) {
        self.pc = position;
    }

    /// Returns the specified register
    pub fn v(&self, index: RegIndex) -> &VRegister {
        &self.vreg[index as usize]
    }

    /// Loads the value into the specified register
    pub fn load_to_v(&mut self, index: RegIndex, value: Byte) {
        self.vreg[index as usize].load(value);
    }

    /// Adds the value to the specified register
    pub fn add_to_v(&mut self, index: RegIndex, value: Byte) {
        self.vreg[index as usize] += value
    }

	/// Performs the specified bit operation with the registers
	pub fn bit_op(&mut self, operation: BitOp) {
		match operation {
			BitOp::Or(x, y) => self.vreg[x as usize] |= self.vreg[y as usize],
			BitOp::And(x, y) => self.vreg[x as usize] &= self.vreg[y as usize],
			_ => unimplemented!("BitOp not yet implemented"),
		}
	}
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            pc: Self::STARTING_ADDRESS,
            vreg: Default::default(),
            ireg: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::core::operations::BitOp;

    use super::Cpu;

    #[test]
    fn increase() {
        let mut cpu = Cpu::default();
        assert_eq!(Cpu::STARTING_ADDRESS, cpu.pc);

        for i in 1..4 {
            cpu.increase();
            assert_eq!(i * 2 + Cpu::STARTING_ADDRESS, cpu.pc);
        }
    }

    #[test]
    fn point_at() {
        let mut cpu = Cpu::default();
        assert_eq!(Cpu::STARTING_ADDRESS, cpu.pc);

        cpu.point_at(0x100);
        assert_eq!(0x100, cpu.pc);
    }

	#[test]
	fn bit_op() {
		let mut cpu = Cpu::default();
		cpu.load_to_v(0, 0x1);
		cpu.load_to_v(1, 0x6);
		cpu.load_to_v(2, 0x3);
		cpu.load_to_v(3, 0x3);
		cpu.load_to_v(4, 0x3);
		cpu.load_to_v(5, 0xB);
		
		cpu.bit_op(BitOp::Or(0, 5));
		assert_eq!(*cpu.v(0), 0xB);

		cpu.bit_op(BitOp::And(1, 5));
		assert_eq!(*cpu.v(1), 0x2);
	}
	
}
