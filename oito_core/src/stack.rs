use crate::{exception::Exception, Address};

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

    /// Takes out the top value of the stack and returns it
    pub fn pop(&mut self) -> Result<Address, Exception> {
        let i = self.pointer as usize;
        if i == 0 {
            Err(Exception::StackUnderflow)
        } else {
            self.pointer -= 1;
            Ok(self.content[i - 1])
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            pointer: STACK_POINTER_INIT,
            content: [EMPTY; STACK_SIZE],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::exception::Exception;

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
        assert_eq!(stack.push(0).unwrap_err(), Exception::StackOverflow);
    }

    #[test]
    fn pop() {
        let mut stack = Stack::default();
        stack.push(0).unwrap();
        stack.push(1).unwrap();
        stack.push(2).unwrap();
        assert_eq!(stack.pop().unwrap(), 2);
        assert_eq!(stack.pop().unwrap(), 1);
        assert_eq!(stack.pop().unwrap(), 0);
    }

    #[test]
    fn underflow_exception() {
        let mut stack = Stack::default();
        assert_eq!(stack.pop().unwrap_err(), Exception::StackUnderflow);
    }
}
