mod alu;
mod register;

use register::{IRegister, VRegister};

use crate::{
    core::operations::{ArithOp, BitOp},
    Address, Byte, RegIndex,
};

const INSTRUCTION_SIZE: u16 = 2;
const NUMBER_OF_REGISTERS: usize = 16;
const FLAG_REG_INDEX: usize = 15;

/// Simmulated CPU
#[derive(Debug)]
pub struct Cpu {
    /// Program Counter
    pc: Address,
    /// V-Registers
    vreg: [VRegister; NUMBER_OF_REGISTERS],
    /// I-Register
    ireg: IRegister,
}

impl Cpu {
    /// Program counter starting address
    pub const STARTING_ADDRESS: Address = 0x200;

    /// Returns the address currently pointed by the Program Counter
    pub fn pc(&self) -> Address {
        self.pc
    }

    /// Returns the address currently pointed by the I Register
    pub fn i(&self) -> Address {
        self.ireg.get()
    }

    /// Returns the state of the Flag Register
    #[cfg(test)]
    pub fn vf(&self) -> Byte {
        self.vreg[FLAG_REG_INDEX].get()
    }

    /// Increases the Program Counter to point to the next instruction
    #[inline]
    pub fn increase(&mut self) {
        self.pc += INSTRUCTION_SIZE;
    }

    /// Decreases the Program Counter to point to the previous instruction
    #[inline]
    pub fn decrease(&mut self) {
        self.pc -= INSTRUCTION_SIZE;
    }

    /// Points the Program Counter to the specified address
    #[inline]
    pub fn point_at(&mut self, position: Address) {
        self.pc = position;
    }

    /// Sets the specified address in the register I
    #[inline]
    pub fn set_i(&mut self, address: Address) {
        self.ireg.load(address);
    }

    /// Raises a flag in the VF register
    #[inline]
    pub fn set_flag(&mut self, flag: Byte) {
        self.vreg[FLAG_REG_INDEX].load(flag);
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

    /// Performs the specified arithmetic operation
    pub fn arith_op(&mut self, operation: ArithOp<u8>) {
        match operation {
            ArithOp::Add(x, value) => {
                let reg = self.v_mut(x);
                reg.load(alu::add(reg.get(), &value).0);
            }
            ArithOp::CheckedAdd(x, y) => {
                let (result, flag) = alu::add(self.v(x).get(), &self.v(y).get());
                self.load_to_v(x, dbg!(result));
                self.set_flag(flag);
            }
            ArithOp::Sub(x, y) => {
                let (result, flag) = alu::sub(self.v(x).get(), &self.v(y).get());
                self.load_to_v(x, result);
                self.set_flag(flag);
            }
            ArithOp::SubN(x, y) => {
                let (result, flag) = alu::sub(self.v(y).get(), &self.v(x).get());
                self.load_to_v(x, result);
                self.set_flag(1 - flag);
            }
        }
    }

    /// Performs the specified bit operation with the registers
    pub fn bit_op(&mut self, operation: BitOp) {
        match operation {
            BitOp::And(x, y) => {
                let result = alu::and(self.v(x).get(), self.v(y).get());
                self.v_mut(x).load(result);
            }
            BitOp::Or(x, y) => {
                let result = alu::or(self.v(x).get(), self.v(y).get());
                self.v_mut(x).load(result);
            }
            BitOp::Xor(x, y) => {
                let result = alu::xor(self.v(x).get(), self.v(y).get());
                self.v_mut(x).load(result);
            }
            BitOp::ShiftRight(x) => {
                let (result, flag) = alu::shr(self.v(x).get());
                self.set_flag(flag);
                self.v_mut(x).load(result);
            }
            BitOp::ShiftLeft(x) => {
                let (result, flag) = alu::shl(self.v(x).get());
                self.set_flag(flag);
                self.v_mut(x).load(result);
            }
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
    use crate::{core::operations::BitOp, Byte};

    const FLAG_CARRY: u8 = 1;
    const NO_FLAG: u8 = 0;

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
    fn decrease() {
        let mut cpu = Cpu::default();

        for i in 1..4 {
            cpu.decrease();
            assert_eq!(Cpu::STARTING_ADDRESS - i * 2, cpu.pc);
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
        assert_eq!(cpu.vf(), FLAG_CARRY);

        cpu.set_flag(NO_FLAG);
        assert_eq!(cpu.vf(), NO_FLAG);
    }

    #[test]
    fn set_i() {
        let mut cpu = Cpu::default();

        cpu.set_i(0xABCD);
        assert_eq!(cpu.ireg, 0xABCD);
    }

    #[test]
    fn load_to_v() {
        let mut cpu = Cpu::default();

        cpu.load_to_v(0, 100);
        assert_eq!(cpu.v(0).get(), 100);
    }

    mod arith_op {
        use crate::core::operations::ArithOp;

        use super::*;

        #[test]
        fn add() {
            let mut cpu = Cpu::default();

            cpu.arith_op(ArithOp::Add(0, Byte::MAX));
            assert_eq!(*cpu.v(0), Byte::MAX);
            assert_eq!(cpu.vf(), NO_FLAG);

            cpu.arith_op(ArithOp::Add(0, 1));
            assert_eq!(*cpu.v(0), 0);
            assert_eq!(cpu.vf(), NO_FLAG);
        }

        #[test]
        fn checked_add() {
            let mut cpu = Cpu::default();
            // No overflow
            cpu.load_to_v(1, 12);
            cpu.arith_op(ArithOp::CheckedAdd(0, 1));
            cpu.load_to_v(2, 13);
            cpu.arith_op(ArithOp::CheckedAdd(0, 2));
            assert_eq!(cpu.vreg[0], 12 + 13);
            assert_eq!(cpu.vf(), NO_FLAG);
            // Overflow
            cpu.load_to_v(0, Byte::MAX);
            cpu.load_to_v(1, 11);
            cpu.arith_op(ArithOp::CheckedAdd(0, 1));
            assert_eq!(cpu.vreg[0], 11 - 1);
            assert_eq!(cpu.vf(), FLAG_CARRY);
        }

        #[test]
        fn sub() {
            let mut cpu = Cpu::default();
            // No underflow
            cpu.load_to_v(0, 12);
            cpu.load_to_v(1, 11);
            cpu.arith_op(ArithOp::Sub(0, 1));
            assert_eq!(cpu.vreg[0], 12 - 11);
            assert_eq!(cpu.vf(), FLAG_CARRY);
            // Underflow
            cpu.load_to_v(0, 1);
            cpu.load_to_v(1, 2);
            cpu.arith_op(ArithOp::Sub(0, 1));
            assert_eq!(cpu.vreg[0], Byte::MAX);
            assert_eq!(cpu.vf(), NO_FLAG);
        }

        #[test]
        fn subn() {
            let mut cpu = Cpu::default();
            // No underflow
            cpu.load_to_v(0, 11);
            cpu.load_to_v(1, 12);
            cpu.arith_op(ArithOp::SubN(0, 1));
            assert_eq!(cpu.vreg[0], 12 - 11);
            assert_eq!(cpu.vf(), NO_FLAG);
            // Underflow
            cpu.load_to_v(0, 2);
            cpu.load_to_v(1, 1);
            cpu.arith_op(ArithOp::SubN(0, 1));
            assert_eq!(cpu.vreg[0], Byte::MAX);
            assert_eq!(cpu.vf(), FLAG_CARRY);
        }
    }

    #[test]
    fn bit_op() {
        let mut cpu = Cpu::default();
        cpu.load_to_v(0, 0b00000001);
        cpu.load_to_v(1, 0b00000110);
        cpu.load_to_v(2, 0b00001110);
        cpu.load_to_v(3, 0b00000101);
        cpu.load_to_v(4, 0b10101010);
        cpu.load_to_v(5, 0b00001011);

        cpu.bit_op(BitOp::Or(0, 5));
        assert_eq!(*cpu.v(0), 0b00001011);

        cpu.bit_op(BitOp::And(1, 5));
        assert_eq!(*cpu.v(1), 0b00000010);

        cpu.bit_op(BitOp::Xor(2, 5));
        assert_eq!(*cpu.v(2), 0b00000101);

        cpu.bit_op(BitOp::ShiftRight(3));
        assert_eq!(*cpu.v(3), 0b00000010);
        assert_eq!(cpu.vf(), 1);

        cpu.bit_op(BitOp::ShiftLeft(4));
        assert_eq!(*cpu.v(4), 0b01010100);
        assert_eq!(cpu.vf(), 1);
    }
}
