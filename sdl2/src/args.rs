use structopt::StructOpt;

/// Customize the emmulator
#[derive(Debug, StructOpt)]
pub struct Args {
    /// The ROM file to read
    pub file: String,
    /// Background color
    #[structopt(long = "bg", short = "b", default_value = "black")]
    pub bg: String,
    /// Foreground color
    #[structopt(long = "fg", short = "f", default_value = "white")]
    pub fg: String,
    /// Scale to apply to the screen
    #[structopt(long = "scale", short = "s", default_value = "20")]
    pub scale: u32,
}
