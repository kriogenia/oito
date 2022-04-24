use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Exception {
	#[error("segmentation fault: invalid RAM address")]
	SegmentationFault,
	#[error("stack overflow")]
	StackOverflow,
	#[error("stack underflow")]
	StackUnderflow,
}