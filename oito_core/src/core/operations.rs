use crate::RegIndex;

pub enum ArithOp<T> {
    Add(RegIndex, T),
    CheckedAdd(RegIndex, RegIndex),
    Sub(RegIndex, RegIndex),
    SubN(RegIndex, RegIndex),
}

pub enum BitOp {
    And(RegIndex, RegIndex),
    Or(RegIndex, RegIndex),
    Xor(RegIndex, RegIndex),
    ShiftRight(RegIndex),
    ShiftLeft(RegIndex),
}
