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

impl<T> PartialEq<T> for Register<T>
where
	T: PartialEq
{
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
