use num_traits::ops::overflowing::OverflowingAdd;

use super::{NO_FLAG, FLAG_CARRY};

#[inline]
pub fn add<T: OverflowingAdd>(left: T, right: T) -> (T, u8) {
	match left.overflowing_add(&right) {
		(result, true) => (result, FLAG_CARRY),
		(result, false) => (result, NO_FLAG)
	}
}