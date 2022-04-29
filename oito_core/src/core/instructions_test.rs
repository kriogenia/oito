use crate::{cpu::Cpu, instruction::Instruction, vram::VRam, Byte};

use super::OitoCore;

#[test]
fn cls() {
    let mut oito = OitoCore::default();
    for i in 0..8 {
        oito.vram.set(i);
    }

    oito.execute(Instruction::CLS).unwrap();
    for i in 0..8 {
        assert_eq!(oito.vram[i], VRam::BLACK);
    }
}

#[test]
fn ret() {
    let mut oito = OitoCore::default();
    oito.stack.push(0x1234).unwrap();

    oito.execute(Instruction::RET).unwrap();
    assert_eq!(0x1234, oito.cpu.pc());
    assert!(oito.stack.peek().is_none());
}

#[test]
fn sys() {
    let mut oito = OitoCore::default();

    oito.execute(Instruction::SYS(0xF10F)).unwrap();
    assert_eq!(0xF10F, oito.cpu.pc());
}

#[test]
fn jp() {
    let mut oito = OitoCore::default();

    oito.execute(Instruction::JP(0x01CF)).unwrap();
    assert_eq!(0x01CF, oito.cpu.pc());
}

#[test]
fn call() {
    let mut oito = OitoCore::default();

    oito.execute(Instruction::CALL(0x1CC0)).unwrap();
    assert_eq!(0x1CC0, oito.cpu.pc());
    assert_eq!(Cpu::STARTING_ADDRESS, oito.stack.peek().unwrap());
}

#[test]
fn call_and_ret() {
    let mut oito = OitoCore::default();

    oito.execute(Instruction::CALL(0x2371)).unwrap();
    oito.execute(Instruction::RET).unwrap();
    assert_eq!(Cpu::STARTING_ADDRESS, oito.cpu.pc());
    assert!(oito.stack.peek().is_none());
}

#[test]
fn se_byte() {
    let mut oito = OitoCore::default();
    // Skip
    oito.execute(Instruction::SErb { x: 0, byte: 0 }).unwrap();
    assert_eq!(Cpu::STARTING_ADDRESS + 2, oito.cpu.pc());
    // No Skip
    oito.execute(Instruction::SErb { x: 0, byte: 1 }).unwrap();
    assert_eq!(Cpu::STARTING_ADDRESS + 2, oito.cpu.pc());
}

#[test]
fn sne_byte() {
    let mut oito = OitoCore::default();
    // No Skip
    oito.execute(Instruction::SNErb { x: 0, byte: 0 }).unwrap();
    assert_eq!(Cpu::STARTING_ADDRESS, oito.cpu.pc());
    // skip
    oito.execute(Instruction::SNErb { x: 0, byte: 1 }).unwrap();
    assert_eq!(Cpu::STARTING_ADDRESS + 2, oito.cpu.pc());
}

#[test]
fn se_register() {
    let mut oito = OitoCore::default();
    // Skip
    oito.execute(Instruction::SErr { x: 0, y: 0 }).unwrap();
    assert_eq!(Cpu::STARTING_ADDRESS + 2, oito.cpu.pc());
    // No Skip
    oito.cpu.load_to_v(1, 0x1);
    oito.execute(Instruction::SErr { x: 0, y: 1 }).unwrap();
    assert_eq!(Cpu::STARTING_ADDRESS + 2, oito.cpu.pc());
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

    oito.execute(Instruction::OR { x: 0, y: 1 }).unwrap();
    assert_eq!(*oito.cpu.v(0), 0x1);
}

#[test]
fn and() {
    let mut oito = OitoCore::default();
    oito.cpu.load_to_v(0, 0x7);
    oito.cpu.load_to_v(1, 0xD);

    oito.execute(Instruction::AND { x: 0, y: 1 }).unwrap();
    assert_eq!(*oito.cpu.v(0), 0x5);
}

#[test]
fn xor() {
    let mut oito = OitoCore::default();
    oito.cpu.load_to_v(0, 0b1010);
    oito.cpu.load_to_v(1, 0b0111);

    oito.execute(Instruction::XOR { x: 0, y: 1 }).unwrap();
    assert_eq!(*oito.cpu.v(0), 0b1101);
}

#[test]
fn add_register_to_register() {
    let mut oito = OitoCore::default();
    oito.cpu.load_to_v(0, Byte::MAX);
    oito.cpu.load_to_v(1, 1);

    oito.execute(Instruction::ADDrr { x: 0, y: 1 }).unwrap();
    assert_eq!(*oito.cpu.v(0), 0);
    assert_eq!(oito.cpu.vf(), 1);
}

#[test]
fn sub() {
    let mut oito = OitoCore::default();
    oito.cpu.load_to_v(0, 1);
    oito.cpu.load_to_v(1, 2);

    oito.execute(Instruction::SUB { x: 0, y: 1 }).unwrap();
    assert_eq!(*oito.cpu.v(0), Byte::MAX);
    assert_eq!(oito.cpu.vf(), 0);
}

#[test]
fn shr() {
    let mut oito = OitoCore::default();
    oito.cpu.load_to_v(0, 0b0101);

    oito.execute(Instruction::SHR(0)).unwrap();
    assert_eq!(*oito.cpu.v(0), 0b0010);
    assert_eq!(oito.cpu.vf(), 1);
}

#[test]
fn subn() {
    let mut oito = OitoCore::default();
    oito.cpu.load_to_v(0, 2);
    oito.cpu.load_to_v(1, 1);

    oito.execute(Instruction::SUBN { x: 0, y: 1 }).unwrap();
    assert_eq!(*oito.cpu.v(0), Byte::MAX);
    assert_eq!(oito.cpu.vf(), 1);
}

#[test]
fn shl() {
    let mut oito = OitoCore::default();
    oito.cpu.load_to_v(0, 0b10100101);

    oito.execute(Instruction::SHL(0)).unwrap();
    assert_eq!(*oito.cpu.v(0), 0b01001010);
    assert_eq!(oito.cpu.vf(), 1);
}

#[test]
fn sne_register() {
    let mut oito = OitoCore::default();
	oito.cpu.load_to_v(1, 1);
    // Skip
    oito.execute(Instruction::SNErr { x: 0, y: 1 }).unwrap();
    assert_eq!(Cpu::STARTING_ADDRESS + 2, oito.cpu.pc());
    // skip
    oito.execute(Instruction::SNErr { x: 0, y: 2 }).unwrap();
    assert_eq!(Cpu::STARTING_ADDRESS + 2, oito.cpu.pc());
}

#[test]
fn ld_i() {
	let mut oito = OitoCore::default();

	oito.execute(Instruction::LDi(0xA2C9)).unwrap();
	assert_eq!(oito.cpu.i(), 0xA2C9);
}

#[test]
fn jp_address() {
	let mut oito = OitoCore::default();
	oito.cpu.load_to_v(0, 0x20);

	oito.execute(Instruction::JPr(0x10)).unwrap();
	assert_eq!(oito.cpu.pc(), 0x30);
}

#[test]
fn draw() {
	let mut _oito = OitoCore::default();

	// TODO after implementing prerendered sprites
}

#[test]
fn skp() {
	let mut oito = OitoCore::default();
	oito.cpu.load_to_v(0, 1);
	oito.cpu.load_to_v(1, 2);
	oito.keys.press_key(1);

	oito.execute(Instruction::SKP(1)).unwrap();
	assert_eq!(Cpu::STARTING_ADDRESS, oito.cpu.pc());

	oito.execute(Instruction::SKP(0)).unwrap();
	assert_eq!(Cpu::STARTING_ADDRESS + 2, oito.cpu.pc());
}

#[test]
fn sknp() {
	let mut oito = OitoCore::default();
	oito.cpu.load_to_v(0, 1);
	oito.cpu.load_to_v(1, 2);
	oito.keys.press_key(1);

	oito.execute(Instruction::SKNP(1)).unwrap();
	assert_eq!(Cpu::STARTING_ADDRESS + 2, oito.cpu.pc());

	oito.execute(Instruction::SKNP(0)).unwrap();
	assert_eq!(Cpu::STARTING_ADDRESS + 2, oito.cpu.pc());
}

#[test]
fn ld_register_to_delay() {
	let mut oito = OitoCore::default();
	oito.dt.set(0x12);

	oito.execute(Instruction::LDdr(0)).unwrap();
	assert_eq!(*oito.cpu.v(0), 0x12);
}