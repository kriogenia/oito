pub mod desktop;

/// Used to load Chip-8 ROMs to run them in **OitoCore**
trait RomLoader {
    /// Returns the ROM ready to be injected in the OitoCore.
    fn rom(&mut self) -> &[u8];
}
