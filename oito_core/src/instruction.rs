use crate::exception::Exception;
use crate::{Address, Byte, OpCode, RegIndex};

/// Mask to convert a word into 12-byte address
const ADDRESS_MASK: u16 = 0x0FFF;
/// Mask to convert a word into a single byte
const BYTE_MASK: u16 = 0x00FF;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    /// 0000 - Do nothing
    NOP,
    /// 00E0 - Clear screen
    CLS,
    /// 00EE - Return
    RET,
    /// 0nnn - SYS jump
    SYS(Address),
    /// 1nnn - Jump to address
    JP(Address),
    /// 2nnn - Call subroutine at address
    CALL(Address),
    /// 3xkk - Skip next instruction if *Vx == kk
    SEb { vx: RegIndex, byte: Byte },
    /// 4xkk - Skip next instruction if *Vx != kk
    SNEb { vx: RegIndex, byte: Byte },
    /// 5xy0 - Skip next instruction if *Vx = *Vy
    SEr { vx: RegIndex, vy: RegIndex },
    /// 6xkk - Load doubleword value into Vx
    LDb { vx: RegIndex, byte: Byte },
    /// 7xkk - Add kk value into Vx
    ADDb { vx: RegIndex, byte: Byte },
    /// 8xy0 - Load Vy content into Vx
    LDr { vx: RegIndex, vy: RegIndex },
}

impl TryFrom<OpCode> for Instruction {
    type Error = Exception;

    fn try_from(value: OpCode) -> Result<Self, Self::Error> {
        use Instruction::*;
        match split(value) {
            (0x0, 0x0, 0x0, 0x0) => Ok(NOP),
            (0x0, 0x0, 0xE, 0x0) => Ok(CLS),
            (0x0, 0x0, 0xE, 0xE) => Ok(RET),
            (0x0, ..) => Ok(SYS(value & ADDRESS_MASK)),
            (0x1, ..) => Ok(JP(value & ADDRESS_MASK)),
            (0x2, ..) => Ok(CALL(value & ADDRESS_MASK)),
            (0x3, vx, ..) => Ok(SEb {
                vx: vx as RegIndex,
                byte: (value & BYTE_MASK) as Byte,
            }),
            (0x4, vx, ..) => Ok(SNEb {
                vx: vx as RegIndex,
                byte: (value & BYTE_MASK) as Byte,
            }),
            (0x5, vx, vy, 0) => Ok(SEr {
                vx: vx as RegIndex,
                vy: vy as RegIndex,
            }),
            (0x6, vx, ..) => Ok(LDb {
                vx: vx as RegIndex,
                byte: (value & BYTE_MASK) as Byte,
            }),
            (0x7, vx, ..) => Ok(ADDb {
                vx: vx as RegIndex,
                byte: (value & BYTE_MASK) as Byte,
            }),
            (0x8, vx, vy, 0) => Ok(LDr {
                vx: vx as RegIndex,
                vy: vy as RegIndex,
            }),
            (..) => Err(Exception::WrongOpCode(value)),
        }
    }
}

/// Divides a word into four nibbles
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
        assert_eq!(Instruction::CLS, Instruction::try_from(0x00E0).unwrap());
        assert_eq!(Instruction::RET, Instruction::try_from(0x00EE).unwrap());
        assert_eq!(
            Instruction::SYS(0xC0A),
            Instruction::try_from(0x0C0A).unwrap()
        );
        assert_eq!(
            Instruction::JP(0xF23),
            Instruction::try_from(0x1F23).unwrap()
        );
        assert_eq!(
            Instruction::CALL(0x232),
            Instruction::try_from(0x2232).unwrap()
        );
        assert_eq!(
            Instruction::SEb { vx: 0, byte: 0x12 },
            Instruction::try_from(0x3012).unwrap()
        );
        assert_eq!(
            Instruction::SNEb { vx: 1, byte: 0x34 },
            Instruction::try_from(0x4134).unwrap()
        );
        assert_eq!(
            Instruction::SEr { vx: 2, vy: 3 },
            Instruction::try_from(0x5230).unwrap()
        );
        assert_eq!(
            Instruction::LDb { vx: 4, byte: 0x56 },
            Instruction::try_from(0x6456).unwrap()
        );
        assert_eq!(
            Instruction::ADDb { vx: 5, byte: 0x67 },
            Instruction::try_from(0x7567).unwrap()
        );
        assert_eq!(
            Instruction::LDr { vx: 6, vy: 7 },
            Instruction::try_from(0x8670).unwrap()
        );
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
