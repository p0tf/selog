//! Convert `Colorize` from/into `termcolor::ColorChoice`
extern crate termcolor;

use crate::Colorize;
use termcolor::ColorChoice;

impl From<ColorChoice> for Colorize {
    fn from(color: ColorChoice) -> Self {
        match color {
            ColorChoice::Always |
            ColorChoice::AlwaysAnsi => Colorize::On,
            ColorChoice::Auto => Colorize::Auto,
            ColorChoice::Never => Colorize::Off
        }
    }
}

impl Into<ColorChoice> for Colorize {
    fn into(self) -> ColorChoice {
        match self {
            Colorize::On => ColorChoice::Always,
            Colorize::Auto => ColorChoice::Auto,
            Colorize::Off => ColorChoice::Never,
        }
    }
}
