const STACK_SIZE: usize = 16;

/// Abstraction of the CPU's stack
pub struct Stack {
	/// Pointer to the top of the stack
	pointer: u16,
	/// Content of the stack
	content: [u16; STACK_SIZE],
}