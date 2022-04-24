use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Exception {
	#[error("stack overflow")]
	StackOverflow,
	#[error("stack underflow")]
	StackUnderflow,
}