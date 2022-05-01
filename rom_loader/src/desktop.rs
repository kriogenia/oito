use std::{fs::File, io::Read};

use crate::RomLoader;

/// Loads binary files using the file path
pub struct FilePathLoader {
    buffer: Vec<u8>,
}

impl FilePathLoader {
    pub fn new(path: &str) -> Self {
        let mut rom = File::open(path).expect("unable to open file");
        let mut buffer = Vec::new();
        rom.read_to_end(&mut buffer).expect("error reading file");
        Self { buffer }
    }
}

impl RomLoader for FilePathLoader {
    fn rom(&self) -> &[u8] {
        &self.buffer
    }
}

#[test]
fn test() {
    let mut loader = FilePathLoader::new("test/test_opcode.ch8");

    let rom = loader.rom();
    assert_eq!(rom[0], 0x12);
    assert_eq!(rom[1], 0x4E);
    assert_eq!(rom[2], 0xEA);
    assert_eq!(rom[rom.len() - 1], 0xDC);
}
