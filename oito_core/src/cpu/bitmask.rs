use crate::Byte;

pub trait BitMask {
    const LEAST_SIGNIFICANT_BIT: Self;
    const MOST_SIGNIFICANT_BIT: Self;
}

impl BitMask for Byte {
    const LEAST_SIGNIFICANT_BIT: Self = 0b00000001;
    const MOST_SIGNIFICANT_BIT: Self = 0b10000000;
}
