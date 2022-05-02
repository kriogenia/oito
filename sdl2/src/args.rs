use std::str::FromStr;

use sdl2::pixels::Color;
use structopt::StructOpt;

/// Customize the emmulator
#[derive(Debug, StructOpt)]
pub struct Args {
    /// The ROM file to read
    pub file: String,
    /// Background color
    #[structopt(long = "bg", short = "b", default_value = "#000000")]
    pub bg: CliColor,
    /// Foreground color
    #[structopt(long = "fg", short = "f", default_value = "#FFFFFF")]
    pub fg: CliColor,
    /// Scale to apply to the screen
    #[structopt(long = "scale", short = "s", default_value = "20")]
    pub scale: u32,
}

#[derive(Debug)]
pub struct CliColor(u8, u8, u8);

impl FromStr for CliColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
		let from = if s.starts_with("#") { 1 } else { 0 };
		let str = &s[from..];

		let r: u8 = u8::from_str_radix(&str[0..=1], 16).unwrap_or_default();
        let g: u8 = u8::from_str_radix(&str[2..=3], 16).unwrap_or_default();
        let b: u8 = u8::from_str_radix(&str[4..=5], 16).unwrap_or_default();

        Ok(CliColor(r, g, b))
    }
}

impl Into<Color> for CliColor {
    fn into(self) -> Color {
        Color::RGB(self.0, self.1, self.2)
    }
}
