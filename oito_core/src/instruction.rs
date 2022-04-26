use crate::exception::Exception;
use crate::{Address, Byte, OpCode, RegIndex, Sprite};

/// Mask to convert a word into 12-byte address
const ADDRESS_MASK: u16 = 0x0FFF;
/// Mask to convert a word into a single byte
const BYTE_MASK: u16 = 0x00FF;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub enum Instruction {
    /// 00E0 - Clear screen: `cls`
    CLS,
    /// 00EE - Return from subroutine: `return;`
    RET,
    /// 0nnn - SYS jump to address. Legacy call.
    SYS(Address),
    /// 1nnn - Jump to address: `goto nnn`
    JP(Address),
    /// 2nnn - Call subroutine at address: `*(0xNNN)()`
    CALL(Address),
    /// 3xkk - Skip next instruction when register equals byte: `if Vx == kk`
    SErb { x: RegIndex, byte: Byte },
    /// 4xkk - Skip next instruction when register don't equals byte `if Vx != kk`
    SNErb { x: RegIndex, byte: Byte },
    /// 5xy0 - Skip next instruction when registers are equal. `if Vx == Vy`
    SErr { x: RegIndex, y: RegIndex },
    /// 6xkk - Load byte into register `Vx = nn`
    LDbr { x: RegIndex, byte: Byte },
    /// 7xkk - Add byte to register `Vx += kk`
    ADDbr { x: RegIndex, byte: Byte },
    /// 8xy0 - Load register Vy into register Vx `Vx = Vy`
    LDrr { x: RegIndex, y: RegIndex },
    /// 8xy1 - Load into register Vx the result of Vx OR Vy `Vx |= Vy`
    ORrr { x: RegIndex, y: RegIndex },
    /// 8xy2 - Load into register Vx the result of Vx AND Vy `Vx &= Vy`
    ANDrr { x: RegIndex, y: RegIndex },
    /// 8xy3 - Load into register Vx the result of Vx XOR Vy `Vx ^= Vy`
    XORrr { x: RegIndex, y: RegIndex },
    /// 8xy4 - Add register Vy to register Vx `Vx += Vy`
    ADDrr { x: RegIndex, y: RegIndex },
    /// 8xy5 - Substract register Vy from register Vx `Vx -= Vy`
    SUBrr { x: RegIndex, y: RegIndex },
    /// 8xy6 - Shift right register `Vx >>= 1`
    SHRr(RegIndex),
    /// 8xy7 - Substract register Vx from register Vy and stores in Vx: `Vx = Vy - Vx`
    SUBNrr { x: RegIndex, y: RegIndex },
    /// 8xyE - Shift left register `Vx <<= 1
    SHLr(RegIndex),
    /// 9xy0 - Skip next instruction when registers are not equals: `Vx != Vy`
    SNEr { x: RegIndex, y: RegIndex },
    /// Annn - Load address into I `I = nnn`
    LDi(Address),
    /// Bnnn - Jump to the address + V0: `PC = V0 + nnn`
    JPr(Address),
    /// Cxnn - Load random byte AND nn into Vx: `Vx = rand() & nn`
    RND { x: RegIndex, byte: Byte },
    /// Dxyn - Display n-byte sprite starting at memory location I at (Vx, Vy) = `draw(Vx, Vy, N)`
    DRW {
        x: RegIndex,
        y: RegIndex,
        sprite: Sprite,
    },
    /// Ex9E - Skip if the key matching Vx is pressed: `if key() == Vx`
    SKP(RegIndex),
    /// ExA1 - Skip if the key pressed don't match Vx: `if key != Vx`
    SKNP(RegIndex),
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
                x: vx as RegIndex,
                byte: (value & BYTE_MASK) as Byte,
            }),
            (0x4, vx, ..) => Ok(SNErb {
                x: vx as RegIndex,
                byte: (value & BYTE_MASK) as Byte,
            }),
            (0x5, vx, vy, 0x0) => Ok(SErr {
                x: vx as RegIndex,
                y: vy as RegIndex,
            }),
            (0x6, vx, ..) => Ok(LDbr {
                x: vx as RegIndex,
                byte: (value & BYTE_MASK) as Byte,
            }),
            (0x7, vx, ..) => Ok(ADDbr {
                x: vx as RegIndex,
                byte: (value & BYTE_MASK) as Byte,
            }),
            (0x8, vx, vy, 0x0) => Ok(LDrr {
                x: vx as RegIndex,
                y: vy as RegIndex,
            }),
            (0x8, vx, vy, 0x1) => Ok(ORrr {
                x: vx as RegIndex,
                y: vy as RegIndex,
            }),
            (0x8, vx, vy, 0x2) => Ok(ANDrr {
                x: vx as RegIndex,
                y: vy as RegIndex,
            }),
            (0x8, vx, vy, 0x3) => Ok(XORrr {
                x: vx as RegIndex,
                y: vy as RegIndex,
            }),
            (0x8, vx, vy, 0x4) => Ok(ADDrr {
                x: vx as RegIndex,
                y: vy as RegIndex,
            }),
            (0x8, vx, vy, 0x5) => Ok(SUBrr {
                x: vx as RegIndex,
                y: vy as RegIndex,
            }),
            (0x8, vx, _, 0x6) => Ok(SHRr(vx as RegIndex)),
            (0x8, vx, vy, 0x7) => Ok(SUBNrr {
                x: vx as RegIndex,
                y: vy as RegIndex,
            }),
            (0x8, vx, _, 0xE) => Ok(SHLr(vx as RegIndex)),
            (0x9, vx, vy, 0x0) => Ok(SNEr {
                x: vx as RegIndex,
                y: vy as RegIndex,
            }),
            (0xA, ..) => Ok(LDi(value & ADDRESS_MASK)),
            (0xB, ..) => Ok(JPr(value & ADDRESS_MASK)),
            (0xC, vx, ..) => Ok(RND {
                x: vx as RegIndex,
                byte: (value & BYTE_MASK) as Byte,
            }),
            (0xD, vx, vy, sprite) => Ok(DRW {
                x: vx as RegIndex,
                y: vy as RegIndex,
                sprite: sprite as Sprite,
            }),
            (0xE, vx, 0x9, 0xE) => Ok(SKP(vx as RegIndex)),
            (0xE, vx, 0xA, 0x1) => Ok(SKNP(vx as RegIndex)),
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
            Instruction::SErb { x: 0, byte: 0x12 },
            Instruction::try_from(0x3012).unwrap()
        );
        assert_eq!(
            Instruction::SNErb { x: 1, byte: 0x34 },
            Instruction::try_from(0x4134).unwrap()
        );
        assert_eq!(
            Instruction::SErr { x: 2, y: 3 },
            Instruction::try_from(0x5230).unwrap()
        );
        assert_eq!(
            Instruction::LDbr { x: 4, byte: 0x56 },
            Instruction::try_from(0x6456).unwrap()
        );
        assert_eq!(
            Instruction::ADDbr { x: 5, byte: 0x67 },
            Instruction::try_from(0x7567).unwrap()
        );
        assert_eq!(
            Instruction::LDrr { x: 6, y: 7 },
            Instruction::try_from(0x8670).unwrap()
        );
        assert_eq!(
            Instruction::ORrr { x: 8, y: 9 },
            Instruction::try_from(0x8891).unwrap()
        );
        assert_eq!(
            Instruction::ANDrr { x: 10, y: 11 },
            Instruction::try_from(0x8AB2).unwrap()
        );
        assert_eq!(
            Instruction::XORrr { x: 12, y: 13 },
            Instruction::try_from(0x8CD3).unwrap()
        );
        assert_eq!(
            Instruction::ADDrr { x: 14, y: 15 },
            Instruction::try_from(0x8EF4).unwrap()
        );
        assert_eq!(
            Instruction::SUBrr { x: 0, y: 2 },
            Instruction::try_from(0x8025).unwrap()
        );
        assert_eq!(Instruction::SHRr(1), Instruction::try_from(0x8126).unwrap());
        assert_eq!(
            Instruction::SUBNrr { x: 4, y: 6 },
            Instruction::try_from(0x8467).unwrap()
        );
        assert_eq!(Instruction::SHLr(3), Instruction::try_from(0x835E).unwrap());
        assert_eq!(
            Instruction::SNEr { x: 8, y: 10 },
            Instruction::try_from(0x98A0).unwrap()
        );
        assert_eq!(
            Instruction::LDi(0x579),
            Instruction::try_from(0xA579).unwrap()
        );
        assert_eq!(
            Instruction::JPr(0xCE0),
            Instruction::try_from(0xBCE0).unwrap()
        );
        assert_eq!(
            Instruction::RND { x: 11, byte: 0xDF },
            Instruction::try_from(0xCBDF).unwrap()
        );
        assert_eq!(
            Instruction::DRW {
                x: 0,
                y: 3,
                sprite: 0x6
            },
            Instruction::try_from(0xD036).unwrap()
        );
        assert_eq!(Instruction::SKP(1), Instruction::try_from(0xE19E).unwrap());
        assert_eq!(Instruction::SKNP(2), Instruction::try_from(0xE2A1).unwrap());
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
