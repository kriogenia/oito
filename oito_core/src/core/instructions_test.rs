use crate::{cpu::Cpu, instruction::Instruction, key::Key, vram::VRam, Address, Byte};

use super::OitoCore;

#[test]
fn execution_test() {
    let mut oito = OitoCore::new();
    let program = [
        0x61, 0x02, // load 0x22 to V1		V1 = 0x02, PC = 514
        0x82, 0x10, // load V1 to V2		V2 = 0x02, PC = 516
        0x71, 0x01, // add 1 to V1			V1 = 0x03, PC = 518
        0xF1, 0x29, // load sprite[V1] in I	I  = 0x0F, PC = 520
        0xD1, 0x25, // draw *I at V1, V2	VRAM[0x23, 0x22] = true, PC = 522
        0xF0, 0x0A, // load key into V0		V0 = key (will be six), PC = 524
        0xF0, 0x1E, // add V0 to I			I  = 0x15, PC = 526
        0x30, 0x06, // skip if V0 == 6 (T)  PC = 530
        0x00, 0xE0, // clear (skipped)
        0x83, 0x11, // V3 = V3 OR V1		V3 = 0x03, PC = 532
    ];
    oito.load(&program);
    oito.key_press(Key::Six);

    for _ in 0..9 {
        oito.tick().unwrap(); // Run the nine instructions (one will be skipped)
    }

    assert_eq!(*oito.cpu.v(0), 0x06);
    assert_eq!(*oito.cpu.v(1), 0x03);
    assert_eq!(*oito.cpu.v(2), 0x02);
    assert_eq!(*oito.cpu.v(3), 0x03);
    assert_eq!(oito.cpu.i(), 0x15);
    assert!(oito.vram.get(0x03, 0x02));
    assert!(oito.vram.get(0x04, 0x02));
    assert!(oito.vram.get(0x05, 0x02));
    assert!(oito.vram.get(0x06, 0x02));
    assert!(oito.vram.get(0x06, 0x03));
}

#[test]
fn cls() {
    let mut oito = OitoCore::default();
    for i in 0..8 {
        oito.vram.paint(0, i);
    }

    oito.execute(Instruction::CLS).unwrap();
    for i in 0..8 {
        assert_eq!(oito.vram.get(0, i), VRam::BLACK);
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
    let mut oito = OitoCore::new();
    oito.cpu.load_to_v(1, 0x2);
    oito.cpu.load_to_v(2, 0x3);
    oito.cpu.set_i(15); // point to the '3' sprite
                        // No flag
    oito.execute(Instruction::DRW { x: 1, y: 2, n: 5 }).unwrap();
    assert!(oito.vram.get(0x2, 0x3));
    assert_eq!(oito.cpu.vf(), 0);
    // Flag
    oito.execute(Instruction::DRW { x: 1, y: 2, n: 5 }).unwrap();
    assert!(!oito.vram.get(0x2, 0x3));
    assert_eq!(oito.cpu.vf(), 1);
}

#[test]
fn skp() {
    let mut oito = OitoCore::default();
    oito.cpu.load_to_v(0, 1);
    oito.cpu.load_to_v(1, 2);
    oito.keys.press_key(Key::One);

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
    oito.keys.press_key(Key::One);

    oito.execute(Instruction::SKNP(1)).unwrap();
    assert_eq!(Cpu::STARTING_ADDRESS + 2, oito.cpu.pc());

    oito.execute(Instruction::SKNP(0)).unwrap();
    assert_eq!(Cpu::STARTING_ADDRESS + 2, oito.cpu.pc());
}

#[test]
fn ld_delay_to_register() {
    let mut oito = OitoCore::default();
    oito.dt.set(0x12);

    oito.execute(Instruction::LDdr(0)).unwrap();
    assert_eq!(*oito.cpu.v(0), 0x12);
}

#[test]
fn ld_key_to_register() {
    let mut oito = OitoCore::default();
    // Non pressed key
    oito.execute(Instruction::LDkr(0)).unwrap();
    assert_eq!(*oito.cpu.v(0), 0);
    assert_eq!(oito.cpu.pc(), Cpu::STARTING_ADDRESS - 2);
    // Pressed key
    oito.keys.press_key(Key::Five);
    oito.execute(Instruction::LDkr(0)).unwrap();
    assert_eq!(*oito.cpu.v(0), 5);
    assert_eq!(oito.cpu.pc(), Cpu::STARTING_ADDRESS - 2);
}

#[test]
fn ld_register_to_delay() {
    let mut oito = OitoCore::default();
    oito.cpu.load_to_v(0, 0xA1);

    oito.execute(Instruction::LDrd(0)).unwrap();
    assert_eq!(0xA1, oito.dt.get());
}

#[test]
fn ld_register_to_sound() {
    let mut oito = OitoCore::default();
    oito.cpu.load_to_v(0, 0xA1);

    oito.execute(Instruction::LDrs(0)).unwrap();
    assert_eq!(0xA1, oito.st.get());
}

#[test]
fn add_register_to_i() {
    let mut oito = OitoCore::default();
    oito.cpu.load_to_v(0, 6);
    oito.cpu.set_i(8);
    // No overflow
    oito.execute(Instruction::ADDri(0)).unwrap();
    assert_eq!(14, oito.cpu.i());
    // Overflow
    oito.cpu.set_i(Address::MAX);
    oito.cpu.load_to_v(1, 1);
    oito.execute(Instruction::ADDri(1)).unwrap();
    assert_eq!(0, oito.cpu.i());
}

#[test]
fn ld_sprite_to_i() {
    let mut oito = OitoCore::new();
    oito.cpu.load_to_v(0, 1);

    oito.execute(Instruction::LDmi(0)).unwrap();
    assert_eq!(5, oito.cpu.i());
}

#[test]
fn ld_bcd() {
    let mut oito = OitoCore::new();
    oito.cpu.load_to_v(1, 234);

    oito.execute(Instruction::LDrm(1)).unwrap();
    assert_eq!(2, oito.ram.read(0).unwrap());
    assert_eq!(3, oito.ram.read(1).unwrap());
    assert_eq!(4, oito.ram.read(2).unwrap());
}

#[test]
fn ld_registers_to_memory() {
    let mut oito = OitoCore::default();
    let start = 0xA;
    oito.cpu.set_i(start);
    oito.cpu.load_to_v(0, 0x1);
    oito.cpu.load_to_v(1, 0x2);
    oito.cpu.load_to_v(2, 0x3);
    oito.cpu.load_to_v(3, 0x4);

    oito.execute(Instruction::LDvm(2)).unwrap();
    for i in 0..=2 {
        assert_eq!(oito.ram.read(i + start).unwrap() as Address, i + 1);
    }
    assert_eq!(oito.ram.read(3 + start).unwrap(), 0);
}

#[test]
fn ld_memory_to_registers() {
    let mut oito = OitoCore::default();
    let start = 100;
    oito.cpu.set_i(start);
    for i in 0..0xF {
        oito.ram.load(start + i, &[i as Byte]);
    }

    oito.execute(Instruction::LDmv(10)).unwrap();
    for i in 0..=10 {
        assert_eq!(*oito.cpu.v(i), i);
    }
    assert_eq!(*oito.cpu.v(11), 0);
}
