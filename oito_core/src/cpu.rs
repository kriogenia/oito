mod register;

use register::{IRegister, VRegister};

use crate::Address;

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
}
