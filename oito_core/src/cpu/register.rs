use crate::{Address, Byte};

const IREG_INIT: Address = 0;
const VREG_INIT: Byte = 0;

pub type IRegister = Register<Address>;
pub type VRegister = Register<Byte>;

/// Representation of one of the CPU's Registers
pub struct Register<T>(T);

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