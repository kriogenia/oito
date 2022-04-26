use crate::exception::Exception;
use crate::{Address, Byte, OpCode, RegIndex};

/// Mask to convert a word into 12-byte address
const ADDRESS_MASK: u16 = 0x0FFF;
/// Mask to convert a word into a single byte
const BYTE_MASK: u16 = 0x00FF;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    /// 00E0 - Clear screen: `cls`
    CLS,
    /// 00EE - Return from subroutine: `return;`
    RET,
    /// 0nnn - SYS jump to address. Legacy call.
    SYS (Address),
    /// 1nnn - Jump to address: `goto nnn`
    JP (Address),
    /// 2nnn - Call subroutine at address: `*(0xNNN)()`
    CALL (Address),
    /// 3xkk - Skip next instruction when register equals byte: `Vx == kk`
    SErb { vx: RegIndex, byte: Byte },
    /// 4xkk - Skip next instruction when register don't equals byte `Vx != kk`
    SNErb { vx: RegIndex, byte: Byte },
    /// 5xy0 - Skip next instruction when registers are equal. `Vx == Vy`
    SErr { vx: RegIndex, vy: RegIndex },
    /// 6xkk - Load byte into register `Vx = nn`
    LDbr { vx: RegIndex, byte: Byte },
    /// 7xkk - Add byte to register `Vx += kk`
    ADDbr { vx: RegIndex, byte: Byte },
    /// 8xy0 - Load register Vy into register Vx `Vx = Vy`
    LDrr { vx: RegIndex, vy: RegIndex },
    /// 8xy1 - Load into register Vx the result of Vx OR Vy `Vx |= Vy`
    ORrr { vx: RegIndex, vy: RegIndex },
    /// 8xy2 - Load into register Vx the result of Vx AND Vy `Vx &= Vy`
    ANDrr { vx: RegIndex, vy: RegIndex },
    /// 8xy3 - Load into register Vx the result of Vx XOR Vy `Vx ^= Vy`
    XORrr { vx: RegIndex, vy: RegIndex },
    /// 8xy4 - Add register Vy to register Vx `Vx += Vy`
    ADDrr { vx: RegIndex, vy: RegIndex },
    /// 8xy5 - Substract register Vy from register Vx `Vx -= Vy`
    SUBrr { vx: RegIndex, vy: RegIndex },
	/// 8xy6 - Shift right register `Vx >>= 1`
	SHRr (RegIndex),
	/// 8xy7 - Substract register Vx from register Vy and stores in Vx: `Vx = Vy - Vx`
	SUBNrr { vx: RegIndex, vy: RegIndex },
	/// 8xyE - Shift left register `Vx <<= 1
	SHLr (RegIndex),
	/// 9xy0 - Skip next instruction when registers are not equals `Vx != Vy`
	SNEr { vx: RegIndex, vy: RegIndex },
	/// Annn - Load address into I `I = nnn`
	LDi(Address),
}

impl TryFrom<OpCode> for Instruction {
    type Error = Exception;

    fn try_from(value: OpCode) -> Result<Self, Self::Error> {
        use Instruction::*;
        match split(value) {
            (0x0, 0x0, 0xE, 0x0) => Ok(CLS),
            (0x0, 0x0, 0xE, 0xE) => Ok(RET),
            (0x0, ..) => Ok(SYS(value & ADDRESS_MASK)),
            (0x1, ..) => Ok(JP(value & ADDRESS_MASK)),
            (0x2, ..) => Ok(CALL(value & ADDRESS_MASK)),
            (0x3, vx, ..) => Ok(SErb {
                vx: vx as RegIndex,
                byte: (value & BYTE_MASK) as Byte,
            }),
            (0x4, vx, ..) => Ok(SNErb {
                vx: vx as RegIndex,
                byte: (value & BYTE_MASK) as Byte,
            }),
            (0x5, vx, vy, 0x0) => Ok(SErr {
                vx: vx as RegIndex,
                vy: vy as RegIndex,
            }),
            (0x6, vx, ..) => Ok(LDbr {
                vx: vx as RegIndex,
                byte: (value & BYTE_MASK) as Byte,
            }),
            (0x7, vx, ..) => Ok(ADDbr {
                vx: vx as RegIndex,
                byte: (value & BYTE_MASK) as Byte,
            }),
            (0x8, vx, vy, 0x0) => Ok(LDrr {
                vx: vx as RegIndex,
                vy: vy as RegIndex,
            }),
            (0x8, vx, vy, 0x1) => Ok(ORrr {
                vx: vx as RegIndex,
                vy: vy as RegIndex,
            }),
            (0x8, vx, vy, 0x2) => Ok(ANDrr {
                vx: vx as RegIndex,
                vy: vy as RegIndex,
            }),
            (0x8, vx, vy, 0x3) => Ok(XORrr {
                vx: vx as RegIndex,
                vy: vy as RegIndex,
            }),
            (0x8, vx, vy, 0x4) => Ok(ADDrr {
                vx: vx as RegIndex,
                vy: vy as RegIndex,
            }),
            (0x8, vx, vy, 0x5) => Ok(SUBrr {
                vx: vx as RegIndex,
                vy: vy as RegIndex,
            }),
            (0x8, vx, _, 0x6) => Ok(SHRr(vx as RegIndex)),
            (0x8, vx, vy, 0x7) => Ok(SUBNrr {
                vx: vx as RegIndex,
                vy: vy as RegIndex,
            }),
            (0x8, vx, _, 0xE) => Ok(SHLr(vx as RegIndex)),
            (0x9, vx, vy, 0x0) => Ok(SNEr {
                vx: vx as RegIndex,
                vy: vy as RegIndex,
            }),
			(0xA, ..) => Ok(LDi(value & ADDRESS_MASK)),
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
            Instruction::SErb { vx: 0, byte: 0x12 },
            Instruction::try_from(0x3012).unwrap()
        );
        assert_eq!(
            Instruction::SNErb { vx: 1, byte: 0x34 },
            Instruction::try_from(0x4134).unwrap()
        );
        assert_eq!(
            Instruction::SErr { vx: 2, vy: 3 },
            Instruction::try_from(0x5230).unwrap()
        );
        assert_eq!(
            Instruction::LDbr { vx: 4, byte: 0x56 },
            Instruction::try_from(0x6456).unwrap()
        );
        assert_eq!(
            Instruction::ADDbr { vx: 5, byte: 0x67 },
            Instruction::try_from(0x7567).unwrap()
        );
        assert_eq!(
            Instruction::LDrr { vx: 6, vy: 7 },
            Instruction::try_from(0x8670).unwrap()
        );
        assert_eq!(
            Instruction::ORrr { vx: 8, vy: 9 },
            Instruction::try_from(0x8891).unwrap()
        );
        assert_eq!(
            Instruction::ANDrr { vx: 10, vy: 11 },
            Instruction::try_from(0x8AB2).unwrap()
        );
        assert_eq!(
            Instruction::XORrr { vx: 12, vy: 13 },
            Instruction::try_from(0x8CD3).unwrap()
        );
        assert_eq!(
            Instruction::ADDrr { vx: 14, vy: 15 },
            Instruction::try_from(0x8EF4).unwrap()
        );
        assert_eq!(
            Instruction::SUBrr { vx: 0, vy: 2 },
            Instruction::try_from(0x8025).unwrap()
        );
        assert_eq!(
            Instruction::SHRr(1),
            Instruction::try_from(0x8126).unwrap()
        );
        assert_eq!(
            Instruction::SUBNrr { vx: 4, vy: 6 },
            Instruction::try_from(0x8467).unwrap()
        );
        assert_eq!(
            Instruction::SHLr(3),
            Instruction::try_from(0x835E).unwrap()
        );
        assert_eq!(
            Instruction::SNEr { vx: 8, vy: 10 },
            Instruction::try_from(0x98A0).unwrap()
        );
        assert_eq!(
            Instruction::LDi(0x579),
            Instruction::try_from(0xA579).unwrap()
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
