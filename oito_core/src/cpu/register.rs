use std::fmt::{Debug, LowerHex};

use crate::{Address, Byte};

const IREG_INIT: Address = 0;
const VREG_INIT: Byte = 0;

pub type IRegister = Register<Address>;
pub type VRegister = Register<Byte>;

/// Representation of one of the CPU's Registers
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Register<T>(T);

impl<T> Register<T> {
    /// Loads the value in the register
    pub fn load(&mut self, value: T) {
        self.0 = value
    }
}

impl<T: Copy> Register<T> {
    /// Returns the current value of the register
    pub fn get(&self) -> T {
        self.0
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

impl<T: LowerHex> Debug for Register<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Register: [{:#x}]", &self.0))
    }
}

impl<T: PartialEq> PartialEq<T> for Register<T> {
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::register::{IRegister, Register};

    use super::VRegister;

    #[test]
    fn load() {
        let mut reg = VRegister::default();

        assert_eq!(0x0, reg.0);

        reg.load(0x1);
        assert_eq!(0x1, reg.0);
    }

    #[test]
    fn get() {
        let reg = Register(2.3);

        assert_eq!(2.3, reg.get())
    }

    #[test]
    fn debug() {
        let reg = Register(0x12);

        assert_eq!("Register: [0x12]", format!("{reg:?}"));
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
