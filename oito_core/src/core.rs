use crate::cpu::Cpu;
use crate::exception::Exception;
use crate::instruction::Instruction;
use crate::ram::Ram;
use crate::stack::Stack;
use crate::timer::Timer;
use crate::vram::VRam;
use crate::{Address, OpCode};

/// Core of the emmulator
pub struct OitoCore {
    /// Emmulated CPU
    cpu: Cpu,
    /// Emmulated RAM
    ram: Ram,
    /// Emmulated Stack
    stack: Stack,
    /// Display memory with the information of the current frame to draw
    vram: VRam,
    /// Delay timer
    dt: Timer,
    /// Sound timer
    st: Timer,
}

impl OitoCore {
    /// Returns a new instance of the emulator core
    pub fn new() -> Self {
        Self::default()
    }

    /// Performs a cycle of the emulator
    pub fn tick(&mut self) -> Result<(), Exception> {
        let opcode = self.fetch(self.cpu.pc)?; // fetch
        let instruction = Instruction::try_from(opcode)?; // decode
        self.execute(instruction)?; // execute
        self.cpu.increase(); // advance
        Ok(())
    }

    /// Perfoms a frame-tied tick
    pub fn frame_tick(&mut self) {
        self.dt.decrease();
        self.st.decrease();
    }

    /// Reads from memory the next instruction and points to the next one
    fn fetch(&mut self, address: Address) -> Result<OpCode, Exception> {
        let big_byte = self.ram.read(address)? as u16;
        let small_byte = self.ram.read(address + 1)? as u16;
        Ok((big_byte << 8) | small_byte)
    }

    /// Executes the provided instruction
    fn execute(&mut self, instruction: Instruction) -> Result<(), Exception> {
        use Instruction::*;
        match instruction {
            NOP => {}
            CLS => self.vram.clear(),
            RET => {
                let address = self.stack.pop()?;
                self.cpu.point_at(address);
            },
			SYS(address) => {
				self.cpu.point_at(address);
			}
            _ => unimplemented!("this instruction is yet to be implemented"),
        }
        Ok(())
    }
}

impl Default for OitoCore {
    fn default() -> Self {
        Self {
            cpu: Default::default(),
            ram: Default::default(),
            stack: Default::default(),
            vram: Default::default(),
            dt: Default::default(),
            st: Default::default(),
        }
    }
}

#[cfg(test)]
mod instructions_test;

#[cfg(test)]
mod api_test {
    use super::OitoCore;
    use crate::cpu::Cpu;

    #[test]
    fn tick() {
        let mut oito = OitoCore::default();

        oito.tick().unwrap();
        assert_eq!(Cpu::STARTING_ADDRESS + 2, oito.cpu.pc);
    }

    #[test]
    fn frame_tick() {
        let mut oito = OitoCore::default();
        oito.dt.set(5);
        oito.st.set(4);

        oito.frame_tick();
        assert_eq!(4, oito.dt.count());
        assert_eq!(3, oito.st.count());
    }

    #[test]
    fn fetch() {
        let mut oito = OitoCore::default();
        oito.ram.set(0, 0x5);
        oito.ram.set(1, 0x1);
        oito.ram.set(2, 0xC);

        let opcode = oito.fetch(1);
        assert_eq!(0x010C, opcode.unwrap());
    }
}
