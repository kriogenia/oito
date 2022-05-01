pub mod desktop;

/// Used to load Chip-8 ROMs to run them in **OitoCore**
pub trait RomLoader {
    /// Returns the ROM ready to be injected in the OitoCore.
    fn rom(&self) -> &[u8];
}
