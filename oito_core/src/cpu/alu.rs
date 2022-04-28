use std::ops::{BitAnd, BitOr, BitXor, Shr, Shl};

use num_traits::{
    ops::overflowing::{OverflowingAdd, OverflowingSub},
    Num,
};

use super::{bitmask::BitMask};

#[inline]
pub fn add<T>(left: T, right: &T) -> (T, T) 
where T: OverflowingAdd + Num {
    match left.overflowing_add(right) {
        (result, true) => (result, T::one()),
        (result, false) => (result, T::zero()),
    }
}

#[inline]
pub fn sub<T>(left: T, right: &T) -> (T, T) 
where T: OverflowingSub + Num {
    match left.overflowing_sub(right) {
        (result, true) => (result, T::zero()),
        (result, false) => (result, T::one()),
    }
}

#[inline]
pub fn and<T: BitAnd<Output = T>>(left: T, right: T) -> T {
    left & right
}

#[inline]
pub fn or<T: BitOr<Output = T>>(left: T, right: T) -> T {
    left | right
}

#[inline]
pub fn xor<T: BitXor<Output = T>>(left: T, right: T) -> T {
    left ^ right
}

#[inline]
pub fn shr<T>(value: T) -> (T, T)
where
    T: Num + Shr<Output = T> + BitMask + BitAnd<Output = T> + Copy,
{
    let lsb = and(value, T::LEAST_SIGNIFICANT_BIT);
    (value >> T::one(), lsb)
}

#[inline]
pub fn shl<T>(value: T) -> (T, T)
where
    T: Num + Shl<Output = T> + BitMask + BitAnd<Output = T> + Copy,
{
    let msb = and(value, T::MOST_SIGNIFICANT_BIT);
    (value << T::one(), if msb.is_zero() { T::zero() } else { T::one() })
}

#[cfg(test)]
mod test {

    #[test]
    fn add() {
        assert_eq!((5, 0), super::add(2, &3));
        assert_eq!((0, 1), super::add(u8::MAX, &1));
    }

    #[test]
    fn sub() {
        assert_eq!((9, 1), super::sub(12, &3));
        assert_eq!((u8::MAX, 0), super::sub(0, &1));
    }

    #[test]
    fn and() {
        assert_eq!(0b0010, super::and(0b1010, 0b0011));
    }

    #[test]
    fn or() {
        assert_eq!(0b1011, super::or(0b1010, 0b0011));
    }

    #[test]
    fn xor() {
        assert_eq!(0b1001, super::xor(0b1010, 0b0011));
    }

    #[test]
    fn shr() {
        assert_eq!((0b0101, 0), super::shr(0b1010));
        assert_eq!((0b0101, 1), super::shr(0b1011));
    }

    #[test]
    fn shl() {
        assert_eq!((0b01010100, 1), super::shl(0b10101010));
        assert_eq!((0b01100110, 0), super::shl(0b00110011));
    }
}
