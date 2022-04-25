use crate::{instruction::Instruction, vram::VRam};

use super::OitoCore;

#[test]
fn cls() {
    let mut oito = OitoCore::default();
    for i in 0..8 {
        oito.vram.set(i);
        assert_eq!(oito.vram.get(i), VRam::WHITE);
    }

    oito.execute(Instruction::CLS);
    for i in 0..8 {
        assert_eq!(oito.vram.get(i), VRam::BLACK);
    }
}
