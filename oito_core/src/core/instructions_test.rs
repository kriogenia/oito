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
