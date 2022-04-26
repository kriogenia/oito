mod register;

use register::{IRegister, VRegister};

use crate::{Address, RegIndex, Byte};

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

	/// Returns the value stored in the specified register
	pub fn register(&self, index: RegIndex) -> Byte {
		self.vreg[index as usize].get()
	}

	/// Loads the value into the specified register
	pub fn load_vx(&mut self, index: RegIndex, value: Byte) {
		self.vreg[index as usize].set(value);
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
}
