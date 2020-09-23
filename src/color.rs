//! Utilities to colorize outputs.
extern crate ansi_term;
extern crate atty;
extern crate log;

use crate::Color;
use ansi_term::Style;
use atty::Stream::Stderr;
use log::Level;

use std::str::FromStr;

/// The color pallet used by the logger.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEPallet {
    pub error: Color,
    pub warn: Color,
    pub info: Color,
    pub debug: Color,
    pub trace: Color,
}

/// The enum to express whether outputs are colorized.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Colorize {
    Off,
    Auto,
    On,
}

impl FromStr for Colorize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" => Ok(Colorize::Off),
            "auto" => Ok(Colorize::Auto),
            "on" => Ok(Colorize::On),
            s => Err(format!("{} is invalid.", s)),
        }
    }
}

impl Default for SEPallet {
    fn default() -> Self {
        SEPallet {
            error: Color::Red,
            warn: Color::Yellow,
            info: Color::Cyan,
            debug: Color::Purple,
            trace: Color::Blue,
        }
    }
}

impl SEPallet {
    /// Create new SEPallet
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the color if Stderr is not redirected.
    pub fn auto(self) -> Option<Self> {
        if atty::is(Stderr) {
            Some(self)
        } else {
            None
        }
    }

    /// Create `ansi_term::Style` from `SEPallet` and `log::Level`.
    pub fn style(&self, level: Level) -> Style {
        match level {
            Level::Error => self.error,
            Level::Warn => self.warn,
            Level::Info => self.info,
            Level::Debug => self.debug,
            Level::Trace => self.trace,
        }
        .normal()
    }

    /// Change color of `error`
    pub fn error(mut self, color: Color) -> Self {
        self.error = color;
        self
    }

    /// Change color of `warn`
    pub fn warn(mut self, color: Color) -> Self {
        self.warn = color;
        self
    }

    /// Change color of `info`
    pub fn info(mut self, color: Color) -> Self {
        self.info = color;
        self
    }

    /// Change color of `debug`
    pub fn debug(mut self, color: Color) -> Self {
        self.debug = color;
        self
    }

    /// Change color of `trace`
    pub fn trace(mut self, color: Color) -> Self {
        self.trace = color;
        self
    }
}
