mod alu;
mod register;

use register::{IRegister, VRegister};

use crate::{core::operations::BitOp, Address, Byte, RegIndex};

const INSTRUCTION_SIZE: u16 = 2;
const NUMBER_OF_REGISTERS: usize = 15;

const NO_FLAG: u8 = 0b00000000;
const FLAG_CARRY: u8 = 0b00000001;

/// Simmulated CPU
pub struct Cpu {
    /// Program Counter
    pub pc: Address,
    /// V-Registers
    vreg: [VRegister; NUMBER_OF_REGISTERS],
    /// I-Register
    ireg: IRegister,
    /// Flag Register
    pub(crate) vf: VRegister,
}

impl Cpu {
    /// Program counter starting address
    pub const STARTING_ADDRESS: Address = 0x200;

    /// Increases the Program Counter to point to the next instruction
    #[inline]
    pub fn increase(&mut self) {
        self.pc += INSTRUCTION_SIZE;
    }

    /// Points the Program Counter to the specified address
    #[inline]
    pub fn point_at(&mut self, position: Address) {
        self.pc = position;
    }

    /// Raises a flag in the VF register
    #[inline]
    pub fn set_flag(&mut self, flag: u8) {
        self.vf.load(flag);
    }

    /// Lowers the flag in the VF register
    #[inline]
    pub fn low_flag(&mut self) {
        self.vf.load(NO_FLAG);
    }

    /// Returns a reference to the specified register. Will panic if the register doesn't exists.
    #[inline]
    pub fn v(&self, index: RegIndex) -> &VRegister {
        &self.vreg[index as usize]
    }

    /// Returns a mutable reference the specified register. Will panic if the register doesn't exists.
    #[inline]
    fn v_mut(&mut self, index: RegIndex) -> &mut VRegister {
        &mut self.vreg[index as usize]
    }

    /// Loads the value into the specified register
    #[inline]
    pub fn load_to_v(&mut self, index: RegIndex, value: Byte) {
        self.v_mut(index).load(value);
    }

    /// Adds the value to the specified register.
    /// In doesn't check overflows, to make an addition with overflow check refer to [checked_add_to_v]
    #[inline]
    pub fn add_to_v(&mut self, index: RegIndex, value: Byte) {
		let reg = self.v_mut(index);
		reg.load(alu::add(reg.get(), value).0);
    }

    /// Performs a checked addition of the value into the specified register.
    /// In case of overflow the flag register (VF) will be set to 1.
    /// If the addition doesn't overflow the VF will be set to 0.
    ///
    /// To perform an addition without check refer to [add_to_v]
    pub fn checked_add_to_v(&mut self, index: RegIndex, value: Byte) {
		let (result, flag) = alu::add(self.v_mut(index).get(), value);
		self.load_to_v(index, result);
		self.set_flag(flag);
    }

    /// Performs a checked substraction of the value into the specified register.
    /// In case of overflow the flag register (VF) will be set to 1.
    /// If the addition doesn't overflow the VF will be set to 0.
    ///
    /// To perform an addition without check refer to [add_to_v]
    pub fn checked_sub_to_v(&mut self, index: RegIndex, value: Byte) {
        if self.v(index).get() > value {
            //self.raise_flag();
        } else {
            //self.low_flag();
        }
        self.vreg[index as usize] -= value;
    }

    /// Performs the specified bit operation with the registers
    pub fn bit_op(&mut self, operation: BitOp) {
        match operation {
            BitOp::And(x, y) => self.vreg[x as usize] &= self.vreg[y as usize],
            BitOp::Or(x, y) => self.vreg[x as usize] |= self.vreg[y as usize],
            BitOp::Xor(x, y) => self.vreg[x as usize] ^= self.vreg[y as usize],
            BitOp::ShiftRight(x) => {
                let least_significant_bit = self.vreg[x as usize] & 0b1;
                self.vf.load(least_significant_bit);
                self.vreg[x as usize] >>= 1;
            }
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
            vf: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        core::operations::BitOp,
        cpu::{FLAG_CARRY, NO_FLAG},
        Byte,
    };

    use super::Cpu;

    #[test]
    fn increase() {
        let mut cpu = Cpu::default();

        for i in 1..4 {
            cpu.increase();
            assert_eq!(i * 2 + Cpu::STARTING_ADDRESS, cpu.pc);
        }
    }

    #[test]
    fn point_at() {
        let mut cpu = Cpu::default();

        cpu.point_at(0x100);
        assert_eq!(0x100, cpu.pc);
    }

    #[test]
    fn v() {
        let mut cpu = Cpu::default();
        cpu.vreg[0].load(10);

        assert_eq!(cpu.v(0).get(), 10);
    }

    #[test]
    fn set_flag() {
        let mut cpu = Cpu::default();

        cpu.set_flag(FLAG_CARRY);
        assert_eq!(cpu.vf, FLAG_CARRY);

        cpu.set_flag(NO_FLAG);
        assert_eq!(cpu.vf, NO_FLAG);
    }

    #[test]
    fn load_to_v() {
        let mut cpu = Cpu::default();

        cpu.load_to_v(0, 100);
        assert_eq!(cpu.v(0).get(), 100);
    }

    #[test]
    fn add_to_v() {
        let mut cpu = Cpu::default();

        cpu.add_to_v(0, Byte::MAX);
        assert_eq!(*cpu.v(0), Byte::MAX);
        assert_eq!(cpu.vf, NO_FLAG);

        cpu.add_to_v(0, 1);
        assert_eq!(*cpu.v(0), 0);
        assert_eq!(cpu.vf, NO_FLAG);
    }

    #[test]
    fn checked_add_to_v() {
        let mut cpu = Cpu::default();
        // No overflow
        cpu.checked_add_to_v(0, 12);
        cpu.checked_add_to_v(0, 13);
        assert_eq!(cpu.vreg[0], 12 + 13);
        assert_eq!(cpu.vf, NO_FLAG);
        // Overflow
        cpu.load_to_v(0, Byte::MAX);
        cpu.checked_add_to_v(0, 11);
        assert_eq!(cpu.vreg[0], 11 - 1);
        assert_eq!(cpu.vf, FLAG_CARRY);
    }

    #[test]
    fn checked_sub_to_v() {
        let mut cpu = Cpu::default();
        // No underflow
        cpu.load_to_v(0, 12);
        cpu.checked_sub_to_v(0, 11);
        assert_eq!(cpu.vreg[0], 12 - 11);
        assert_eq!(cpu.vf, FLAG_CARRY);
        // Underflow
        cpu.load_to_v(0, 1);
        cpu.checked_sub_to_v(0, 2);
        assert_eq!(cpu.vreg[0], Byte::MAX);
        assert_eq!(cpu.vf, NO_FLAG);
    }

    #[test]
    fn bit_op() {
        let mut cpu = Cpu::default();
        cpu.load_to_v(0, 0b0001);
        cpu.load_to_v(1, 0b0110);
        cpu.load_to_v(2, 0b1110);
        cpu.load_to_v(3, 0b0101);
        cpu.load_to_v(4, 0b0011);
        cpu.load_to_v(5, 0b1011);

        cpu.bit_op(BitOp::Or(0, 5));
        assert_eq!(*cpu.v(0), 0b1011);

        cpu.bit_op(BitOp::And(1, 5));
        assert_eq!(*cpu.v(1), 0b0010);

        cpu.bit_op(BitOp::Xor(2, 5));
        assert_eq!(*cpu.v(2), 0b0101);

        cpu.bit_op(BitOp::ShiftRight(3));
        assert_eq!(*cpu.v(3), 0b0010);
        assert_eq!(cpu.vf, 1);
    }
}
