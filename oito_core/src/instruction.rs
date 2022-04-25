use crate::exception::Exception;
use crate::{Address, OpCode};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    NOP,         	// 0000
    CLS,         	// 00E0
    RET,         	// 00EE
	SYS(Address),	// 0nnn
    JP(Address), 	// 1nnn
	CALL(Address),	// 2nnn
}

impl TryFrom<OpCode> for Instruction {
    type Error = Exception;

    fn try_from(value: OpCode) -> Result<Self, Self::Error> {
        use Instruction::*;
        match split(value) {
            (0x0, 0x0, 0x0, 0x0) => Ok(NOP),
            (0x0, 0x0, 0xE, 0x0) => Ok(CLS),
            (0x0, 0x0, 0xE, 0xE) => Ok(RET),
            (0x0, n1, n2, n3) => Ok(SYS(to_address(n1, n2, n3))),
            (0x1, n1, n2, n3) => Ok(JP(to_address(n1, n2, n3))),
            (0x2, n1, n2, n3) => Ok(CALL(to_address(n1, n2, n3))),
            (_, _, _, _) => Err(Exception::WrongOpCode(value)),
        }
    }
}

fn split(opcode: OpCode) -> (u16, u16, u16, u16) {
    (
        (opcode & 0xF000) >> 12,
        (opcode & 0x0F00) >> 8,
        (opcode & 0x00F0) >> 4,
        opcode & 0x000F,
    )
}

fn to_address(first: u16, second: u16, third: u16) -> Address {
    first << 8 | second << 4 | third
}

#[cfg(test)]
mod test {
    use crate::exception::Exception;

    use super::Instruction;

    #[test]
    fn try_from() {
        assert_eq!(Instruction::NOP, Instruction::try_from(0x0000).unwrap());
        assert_eq!(Instruction::CLS, Instruction::try_from(0x00E0).unwrap());
        assert_eq!(Instruction::RET, Instruction::try_from(0x00EE).unwrap());
        assert_eq!(
            Exception::WrongOpCode(0xFFFF),
            Instruction::try_from(0xFFFF).unwrap_err()
        );
    }

    #[test]
    fn split() {
        assert_eq!((0x2, 0xA, 0x9, 0x0), super::split(0x2A90))
    }

	#[test]
	fn to_address() {
		assert_eq!(0xB32, super::to_address(0xB, 0x3, 0x2))
	}
}
