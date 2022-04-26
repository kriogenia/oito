use crate::RegIndex;

pub enum BitOp {
	And(RegIndex, RegIndex),
	Or(RegIndex, RegIndex),
	Xor(RegIndex, RegIndex),
	ShiftRight(RegIndex),
	ShiftLeft(RegIndex),
}