use std::{
    fmt::{Debug, LowerHex},
    ops::AddAssign,
};

use num_traits::WrappingAdd;

use crate::{Address, Byte};

const IREG_INIT: Address = 0;
const VREG_INIT: Byte = 0;

pub type IRegister = Register<Address>;
pub type VRegister = Register<Byte>;

/// Representation of one of the CPU's Registers
#[derive(PartialEq, Eq)]
pub struct Register<T>(T);

impl<T> Register<T>
where
    T: Copy,
{
    /// Loads the value in the register
    pub fn load(&mut self, value: T) {
        self.0 = value
    }
}

impl Default for IRegister {
    /// Builds the default for the IRegister
    fn default() -> Self {
        Self(IREG_INIT)
    }
}

impl Default for VRegister {
    /// Builds the default for the VRegisters
    fn default() -> Self {
        Self(VREG_INIT)
    }
}

impl<T> AddAssign<T> for Register<T>
where
    T: WrappingAdd,
{
    fn add_assign(&mut self, rhs: T) {
        self.0 = self.0.wrapping_add(&rhs)
    }
}

impl<T> Debug for Register<T>
where
    T: LowerHex,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Register: [{:#x}]", &self.0))
    }
}

impl<T> PartialEq<T> for Register<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

#[cfg(test)]
mod test {
    use crate::{cpu::register::{IRegister, Register}, Address};

    use super::VRegister;

    #[test]
    fn load() {
        let mut reg = VRegister::default();
        assert_eq!(0x0, reg.0);
        reg.load(0x1);
        assert_eq!(0x1, reg.0);
    }

    #[test]
    fn add_assign() {
        let mut reg = IRegister::default();
        reg.load(0x2);
        // No overflow
		reg += 0x3;
        assert_eq!(0x5, reg.0);
		// Overflow
		reg += Address::MAX;
		assert_eq!(0x4, reg.0);
    }

    #[test]
    fn eq_byte() {
        let reg = VRegister::default();
        assert!(reg == 0x0);
        assert!(reg != 0x1);
    }

    #[test]
    fn eq_reg() {
        let vx = IRegister::default();
        let mut vy = Register(1u16);
        assert!(&vx != &vy);
        vy.load(0x0);
        assert!(&vx == &vy);
    }
}
