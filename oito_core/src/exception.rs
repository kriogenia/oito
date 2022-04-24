use thiserror::Error;

#[derive(Error, Debug)]
pub enum Exception {
	#[error("stack overflow")]
	StackOverflow,
}