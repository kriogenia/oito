use crate::Address;

const STACK_SIZE: usize = 16;
const STACK_POINTER_INIT: Address = 0;
const EMPTY: Address = 0;

/// Abstraction of the CPU's stack
pub struct Stack {
	/// Pointer to the top of the stack
	pointer: Address,
	/// Content of the stack
	content: [Address; STACK_SIZE],
}

impl Default for Stack {
    fn default() -> Self {
        Self { pointer: STACK_POINTER_INIT, content: [ EMPTY; STACK_SIZE ] }
    }
}