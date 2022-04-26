use std::{
    fmt::{Debug, LowerHex},
    ops::{AddAssign, BitOrAssign, BitAndAssign, BitXorAssign},
};

use num_traits::WrappingAdd;

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

impl<T: WrappingAdd> AddAssign<T> for Register<T> {
	fn add_assign(&mut self, rhs: T) {
		self.0 = self.0.wrapping_add(&rhs)
    }
}

impl<T: BitAndAssign> BitAndAssign for Register<T> {
	fn bitand_assign(&mut self, rhs: Self) {
		self.0 &= rhs.0
	}
}

impl<T: BitOrAssign> BitOrAssign for Register<T> {
	fn bitor_assign(&mut self, rhs: Self) {
		self.0 |= rhs.0
	}
}

impl<T: BitXorAssign> BitXorAssign for Register<T> {
	fn bitxor_assign(&mut self, rhs: Self) {
		self.0 ^= rhs.0
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
    use crate::{
        cpu::register::{IRegister, Register},
        Address,
    };

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
	fn bitand_assign() {
        let mut vx = Register(3u8);
        let mut vy = Register(1u8);

		vx &= vy;
        assert_eq!(vx, 0x1);
        
		vy.load(0x2);
		vx &= vy;
		assert_eq!(vx, 0x0);
	}

	#[test]
	fn bitor_assign() {
        let mut vx = Register(2u8);
        let mut vy = Register(1u8);

		vx |= vy;
        assert_eq!(vx, 0x3);
        
		vy.load(0x9);
		vx |= vy;
		assert_eq!(vx, 0xB);
	}

	#[test]
	fn bitxor_assign() {
        let mut vx = Register(6u8);
        let vy = Register(5u8);
		
		vx ^= vy;
        assert_eq!(vx, 0x3);
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
