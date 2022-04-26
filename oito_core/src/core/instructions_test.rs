use crate::{cpu::Cpu, instruction::Instruction, vram::VRam};

use super::OitoCore;

#[test]
fn cls() {
    let mut oito = OitoCore::default();
    for i in 0..8 {
        oito.vram.set(i);
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

    oito.execute(Instruction::RET).unwrap();
    assert_eq!(0x1234, oito.cpu.pc);
    assert!(oito.stack.peek().is_none());
}

#[test]
fn sys() {
    let mut oito = OitoCore::default();

    oito.execute(Instruction::SYS(0xF10F)).unwrap();
    assert_eq!(0xF10F, oito.cpu.pc);
}

#[test]
fn jp() {
    let mut oito = OitoCore::default();

    oito.execute(Instruction::JP(0x01CF)).unwrap();
    assert_eq!(0x01CF, oito.cpu.pc);
}

#[test]
fn call() {
    let mut oito = OitoCore::default();

    oito.execute(Instruction::CALL(0x1CC0)).unwrap();
    assert_eq!(0x1CC0, oito.cpu.pc);
    assert_eq!(Cpu::STARTING_ADDRESS, oito.stack.peek().unwrap());
}

#[test]
fn call_and_ret() {
    let mut oito = OitoCore::default();

    oito.execute(Instruction::CALL(0x2371)).unwrap();
    oito.execute(Instruction::RET).unwrap();
    assert_eq!(Cpu::STARTING_ADDRESS, oito.cpu.pc);
    assert!(oito.stack.peek().is_none());
}

#[test]
fn se_byte() {
    let mut oito = OitoCore::default();
    // Skip
    oito.execute(Instruction::SErb { x: 0, byte: 0 }).unwrap();
    assert_eq!(Cpu::STARTING_ADDRESS + 2, oito.cpu.pc);
    // No Skip
    oito.execute(Instruction::SErb { x: 0, byte: 1 }).unwrap();
    assert_eq!(Cpu::STARTING_ADDRESS + 2, oito.cpu.pc);
}

#[test]
fn sne() {
    let mut oito = OitoCore::default();
    // No Skip
    oito.execute(Instruction::SNErb { x: 0, byte: 0 }).unwrap();
    assert_eq!(Cpu::STARTING_ADDRESS, oito.cpu.pc);
    // skip
    oito.execute(Instruction::SNErb { x: 0, byte: 1 }).unwrap();
    assert_eq!(Cpu::STARTING_ADDRESS + 2, oito.cpu.pc);
}

#[test]
fn se_register() {
    let mut oito = OitoCore::default();
    // Skip
    oito.execute(Instruction::SErr { x: 0, y: 0 }).unwrap();
    assert_eq!(Cpu::STARTING_ADDRESS + 2, oito.cpu.pc);
    // No Skip
    oito.cpu.load_to_v(1, 0x1);
    oito.execute(Instruction::SErr { x: 0, y: 1 }).unwrap();
    assert_eq!(Cpu::STARTING_ADDRESS + 2, oito.cpu.pc);
}

#[test]
fn ld_byte_to_register() {
    let x = 1;
    let mut oito = OitoCore::default();

    oito.execute(Instruction::LDbr { x, byte: 0x1 }).unwrap();
    assert!(*oito.cpu.v(x) == 0x1);
}

#[test]
fn add_byte_to_register() {
    let mut oito = OitoCore::default();

    oito.execute(Instruction::ADDbr { x: 0, byte: 0x2 })
        .unwrap();
    assert_eq!(*oito.cpu.v(0), 0x2);

    oito.execute(Instruction::ADDbr { x: 0, byte: 0x8 })
        .unwrap();
    assert_eq!(*oito.cpu.v(0), 0xA);
}

#[test]
fn or() {
    let mut oito = OitoCore::default();
	oito.cpu.load_to_v(1, 0x1);

    oito.execute(Instruction::OR { x: 0, y: 1 })
        .unwrap();
    assert_eq!(*oito.cpu.v(0), 0x1);
}

#[test]
fn and() {
    let mut oito = OitoCore::default();
	oito.cpu.load_to_v(0, 0x7);
	oito.cpu.load_to_v(1, 0xD);

    oito.execute(Instruction::AND { x: 0, y: 1 })
        .unwrap();
    assert_eq!(*oito.cpu.v(0), 0x5);
}

