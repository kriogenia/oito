use crate::{Address, exception::Exception};

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

impl Stack {

	/// Pushes the specified address at the top of the stack
	pub fn push(&mut self, content: Address) -> Result<(), Exception> {
		let i = self.pointer as usize;
		if i >= STACK_SIZE - 1 {
			Err(Exception::StackOverflow)
		} else {
			self.content[i] = content;
			self.pointer += 1;
			Ok(())
		}
	}

	pub fn pop() {

	}

}

impl Default for Stack {
    fn default() -> Self {
        Self { pointer: STACK_POINTER_INIT, content: [ EMPTY; STACK_SIZE ] }
    }
}

#[cfg(test)]
mod test {
    use super::{Stack, STACK_SIZE};

	#[test]
	fn push() {
		let mut stack = Stack::default();
		stack.push(0x001).unwrap();
		assert_eq!(1, stack.pointer);
		assert_eq!(0x001, stack.content[0]);
		stack.push(0x007).unwrap();
		stack.push(0x123).unwrap();
		assert_eq!(3, stack.pointer);
		assert_eq!(0x123, stack.content[2]);
	}

	#[test]
	fn overflow_exception() {
		let mut stack = Stack::default();
		for i in 0..STACK_SIZE - 1 {
			assert!(stack.push(i as u16).is_ok());
		}
		assert!(stack.push(0).is_err());
	}

}