use crate::{Address, Byte};

const IREG_INIT: Address = 0;
const VREG_INIT: Byte = 0;

pub type IRegister = Register<Address>;
pub type VRegister = Register<Byte>;

/// Representation of one of the CPU's Registers
pub struct Register<T>(T);

impl<T> Register<T>
where
    T: Copy,
{
    pub fn get(&self) -> T {
        self.0
    }
}

impl Default for IRegister {
    fn default() -> Self {
        Self(IREG_INIT)
    }
}

impl Default for VRegister {
    fn default() -> Self {
        Self(VREG_INIT)
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::register::IRegister;

    use super::VRegister;


	#[test]
	fn get() {
		let mut vreg = VRegister::default();
		vreg.0 = 12;
		assert_eq!(12, vreg.get());

		let mut vreg = IRegister::default();
		vreg.0 = 2;
		assert_eq!(2, vreg.get());
	}

}