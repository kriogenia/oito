use crate::exception::Exception;
use crate::OpCode;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    NOP,
}

impl TryFrom<OpCode> for Instruction {
    type Error = Exception;

    fn try_from(value: OpCode) -> Result<Self, Self::Error> {
        match split(value) {
            (0, 0, 0, 0) => Ok(Instruction::NOP),
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

#[cfg(test)]
mod test {
    use crate::exception::Exception;

    use super::Instruction;

    #[test]
    fn try_from() {
        assert_eq!(Instruction::NOP, Instruction::try_from(0x0000).unwrap());
        assert_eq!(
            Exception::WrongOpCode(0xFFFF),
            Instruction::try_from(0xFFFF).unwrap_err()
        );
    }

    #[test]
    fn split() {
        assert_eq!((0x2, 0xA, 0x9, 0x0), super::split(0x2A90))
    }
}
