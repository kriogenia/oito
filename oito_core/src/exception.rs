use thiserror::Error;

use crate::{Address, OpCode};

#[derive(Error, Debug, PartialEq)]
pub enum Exception {
    #[error("Segmentation fault. Invalid RAM address: {0:04x}")]
    SegmentationFault(Address),
    #[error("Stack overflow")]
    StackOverflow,
    #[error("Stack underflow")]
    StackUnderflow,
    #[error("Wrong OpCode: {0:04x}")]
    WrongOpCode(OpCode),
}
