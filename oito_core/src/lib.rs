mod cpu;
mod ram;
mod vram;

use cpu::Cpu;
use ram::Ram;
use vram::VRam;

/// Core of the emmulator
pub struct OitoCore {
	/// Emmulated CPU
	cpu: Cpu,
	/// Current frame to draw
	vram: VRam,
	/// Emmulated RAM
	ram: Ram,
}