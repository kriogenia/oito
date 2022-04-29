use crate::core::operations::{ArithOp, BitOp};
use crate::cpu::Cpu;
use crate::exception::Exception;
use crate::instruction::Instruction;
use crate::keymap::KeyMap;
use crate::ram::Ram;
use crate::stack::Stack;
use crate::timer::Timer;
use crate::vram::VRam;
use crate::{fontset, Address, BitMask, Byte, OpCode};

use rand::random;

pub(crate) mod operations;

const BYTE_SIZE: u8 = 8;

/// Core of the emmulator
#[derive(Debug)]
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
    /// Key character map
    keys: KeyMap,
}

impl OitoCore {
    /// Returns a new instance of the emulator core
    pub fn new() -> Self {
        let mut oito = Self::default();
        oito.ram.load(0, &fontset::FONTSET);
        oito
    }

    /// Performs a cycle of the emulator
    pub fn tick(&mut self) -> Result<(), Exception> {
        let opcode = self.fetch(self.cpu.pc())?; // fetch
        self.cpu.increase(); // advance
        let instruction = Instruction::try_from(opcode)?; // decode
        self.execute(instruction)?; // execute
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
            CLS => self.vram.clear(),
            RET => {
                let address = self.stack.pop()?;
                self.cpu.point_at(address);
            }
            SYS(address) => {
                self.cpu.point_at(address);
            }
            JP(address) => {
                self.cpu.point_at(address);
            }
            CALL(address) => {
                self.stack.push(self.cpu.pc())?;
                self.cpu.point_at(address);
            }
            SErb { x, byte } => {
                if *self.cpu.v(x) == byte {
                    self.cpu.increase();
                }
            }
            SNErb { x, byte } => {
                if *self.cpu.v(x) != byte {
                    self.cpu.increase();
                }
            }
            SErr { x, y } => {
                if self.cpu.v(x) == self.cpu.v(y) {
                    self.cpu.increase();
                }
            }
            LDbr { x, byte } => self.cpu.load_to_v(x, byte),
            ADDbr { x, byte } => self.cpu.arith_op(ArithOp::Add(x, byte)),
            LDrr { x, y } => self.cpu.load_to_v(x, self.cpu.v(y).get()),
            OR { x, y } => self.cpu.bit_op(BitOp::Or(x, y)),
            AND { x, y } => self.cpu.bit_op(BitOp::And(x, y)),
            XOR { x, y } => self.cpu.bit_op(BitOp::Xor(x, y)),
            ADDrr { x, y } => self.cpu.arith_op(ArithOp::CheckedAdd(x, y)),
            SUB { x, y } => self.cpu.arith_op(ArithOp::Sub(x, y)),
            SHR(x) => self.cpu.bit_op(BitOp::ShiftRight(x)),
            SUBN { x, y } => self.cpu.arith_op(ArithOp::SubN(x, y)),
            SHL(x) => self.cpu.bit_op(BitOp::ShiftLeft(x)),
            SNErr { x, y } => {
                if self.cpu.v(x) != self.cpu.v(y) {
                    self.cpu.increase();
                }
            }
            LDi(address) => self.cpu.set_i(address),
            JPr(address) => self.cpu.point_at(self.cpu.v(0).get() as Address + address),
            RND { x, byte } => self.cpu.load_to_v(x, byte & random::<Byte>()),
            DRW { x, y, n } => {
				let x = self.cpu.v(x).get();
				let y = self.cpu.v(y).get();

                let mut swapped = false;
                for i in 0..n {
                    let address = self.cpu.i() + i as Address;
                    let pixels = self.ram.read(address)?;
                    for j in 0..BYTE_SIZE {
                        if (pixels & (Byte::MOST_SIGNIFICANT_BIT >> j)) != 0 {
							let x = (x + j) as usize;
							let y = (y + i) as usize;
							dbg!(x, y);
                            swapped |= self.vram.get(x, y);
                            self.vram.paint(x, y);
                        }
                    }
                }
                self.cpu.set_flag(swapped as Byte);
            }
            SKP(x) => {
                if self.keys[self.cpu.v(x).get()] {
                    self.cpu.increase();
                }
            }
            SKNP(x) => {
                if !self.keys[self.cpu.v(x).get()] {
                    self.cpu.increase();
                }
            }
            LDdr(x) => self.cpu.load_to_v(x, self.dt.get()),
            LDkr(x) => {
                match self.keys.get_key_pressed() {
                    Some(k) => self.cpu.load_to_v(x, k as Byte),
                    None => self.cpu.decrease(), // simulate loop pointing to the same instruction
                }
            }
            LDrd(x) => self.dt.set(self.cpu.v(x).get()),
            LDrs(x) => self.st.set(self.cpu.v(x).get()),
            ADDri(x) => {
                let address = self.cpu.i().wrapping_add(self.cpu.v(x).get() as Address);
                self.cpu.set_i(address);
            }
			LDmi(x) => {
				let character = self.cpu.v(x).get();
				let sprite_address = fontset::location(character);
				self.cpu.set_i(sprite_address);
			}
			LDrm(x) => {
				let binary = self.cpu.v(x).get();
				let (h, t, u) = (binary / 100, (binary % 100) / 10, binary % 10);
				let i = self.cpu.i() as usize;
				
				self.ram.load(i, &[h]);
				self.ram.load(i + 1, &[t]);
				self.ram.load(i + 2, &[u]);
			}
			
            _ => unimplemented!("this instruction is yet to be implemented"),
        }
        Ok(())
    }
}

impl Default for OitoCore {
    /// Generates a completely empty emulator. All the contained data will be empty.
    /// To create an emulator with the preloaded data use [OitoCore::new]
    fn default() -> Self {
        Self {
            cpu: Default::default(),
            ram: Default::default(),
            stack: Default::default(),
            vram: Default::default(),
            dt: Default::default(),
            st: Default::default(),
            keys: Default::default(),
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
    fn new() {
        let oito = OitoCore::new();

        assert_eq!(0xF0, oito.ram.read(0x0).unwrap());
    }

    #[test]
    fn tick() {
        let mut oito = OitoCore::default();
        // Next instruction - SE V0 == 1 -> don't skip		// TODO use different and testable instruction
        oito.ram.set(Cpu::STARTING_ADDRESS, 0x30);
        oito.ram.set(Cpu::STARTING_ADDRESS + 1, 0x01);

        oito.tick().unwrap();
        assert_eq!(Cpu::STARTING_ADDRESS + 2, oito.cpu.pc());
    }

    #[test]
    fn frame_tick() {
        let mut oito = OitoCore::default();
        oito.dt.set(5);
        oito.st.set(4);

        oito.frame_tick();
        assert_eq!(4, oito.dt.get());
        assert_eq!(3, oito.st.get());
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
