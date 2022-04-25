use crate::{cpu::Cpu, instruction::Instruction, vram::VRam};

use super::OitoCore;

#[test]
fn cls() {
    let mut oito = OitoCore::default();
    for i in 0..8 {
        oito.vram.set(i);
        assert_eq!(oito.vram.get(i), VRam::WHITE);
    }

    oito.execute(Instruction::CLS).unwrap();
    for i in 0..8 {
        assert_eq!(oito.vram.get(i), VRam::BLACK);
    }
}

#[test]
fn ret() {
    let mut oito = OitoCore::default();
    oito.stack.push(0x1234).unwrap();
    assert_eq!(Cpu::STARTING_ADDRESS, oito.cpu.pc);

    oito.execute(Instruction::RET).unwrap();
    assert_eq!(0x1234, oito.cpu.pc);
}

#[test]
fn sys() {
    let mut oito = OitoCore::default();
    assert_eq!(Cpu::STARTING_ADDRESS, oito.cpu.pc);

    oito.execute(Instruction::SYS(0xF10F)).unwrap();
    assert_eq!(0xF10F, oito.cpu.pc);
}

#[test]
fn jp() {
    let mut oito = OitoCore::default();
    assert_eq!(Cpu::STARTING_ADDRESS, oito.cpu.pc);

    oito.execute(Instruction::JP(0x01CF)).unwrap();
    assert_eq!(0x01CF, oito.cpu.pc);
}
