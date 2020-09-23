//! Simple, Easy LOGger logs to StdErr.
//!
//! > CAUTION: This crate is compatible with `log ^0.4.11`.
//!
//! This crate provides a simple, minimal and easy to use
//! implementation of [`log`](https://docs.rs/log) crate.
//!
//!
//! # Example
//! ## Simple Use Case
//!
//! ```rust
//! #[macro_use]
//! extern crate log;
//! extern crate selog;
//!
//! use selog::SELog;
//!
//! fn main() {
//!     SELog::new().init().unwrap();
//!
//!     error!("Failed something");
//!
//!     // ..
//! }
//! ```
//!
//! ## Using `clap`
//! Tested on `clap 2.33.3`
//!
//! ```rust
//! #[macro_use]
//! extern crate log;
//! #[macro_use]
//! extern crate clap;
//! extern crate selog;
//!
//! use clap::Arg;
//! use selog::{Colorize, SELevel, SELog};
//! use std::str::FromStr;
//!
//! fn main() {
//!     let m = app_from_crate!()
//!         .args(&[
//!             Arg::from_usage("-v --verbose 'More verbose output.'"),
//!             Arg::from_usage("-d --debug 'Output debug log.'"),
//!             Arg::from_usage("-q --quiet 'Less output.'"),
//!             Arg::from_usage("--no-output 'Silence all output.'"),
//!             Arg::from_usage("--color=[mode] 'Control color of output.'")
//!                 .possible_values(&["off", "auto", "on"])
//!                 .default_value("auto"),
//!         ])
//!         .get_matches();
//!
//!     SELog::new()
//!         .level(
//!             SELevel::new()
//!                 .verbose(m.is_present("verbose"))
//!                 .debug(m.is_present("debug"))
//!                 .quiet(m.is_present("quiet"))
//!                 .off(m.is_present("no-output")),
//!         )
//!         // As the string is validated in the definition, these `unwrap` are safe.
//!         .colorize(Colorize::from_str(m.value_of("color").unwrap()).unwrap())
//!         .init()
//!         .unwrap();
//!
//!
//!     error!("Failed something.");
//!
//!     // ...
//! }
//! ```
extern crate ansi_term;
extern crate log;

#[cfg(test)]
mod tests;

pub mod color;
pub mod level;

// Re-exports
pub use ansi_term::Colour as Color;
pub use color::{Colorize, SEPallet};
pub use level::SELevel;

use ansi_term::Style;
use log::{
    set_boxed_logger, set_max_level, Level, LevelFilter, Log, Metadata, Record, SetLoggerError,
};

use std::fmt;

/// The SELog struct.
#[derive(Clone, Copy, PartialEq)]
pub struct SELog {
    level: LevelFilter,
    pallet: Option<SEPallet>,
}

impl Default for SELog {
    fn default() -> Self {
        SELog {
            level: LevelFilter::Warn,
            pallet: SEPallet::new().auto(),
        }
    }
}

impl fmt::Debug for SELog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SELog")
            .field("level", &self.level)
            .field("pallet", &self.pallet)
            .finish()
    }
}

impl Log for SELog {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        match self.level.to_level() {
            Some(level) => metadata.level() <= level,
            None => false,
        }
    }

    fn log(&self, record: &Record<'_>) {
        let level = record.level();

        eprintln!(
            "{}: {}",
            self.style(level).paint(level.to_string().to_lowercase()),
            record.args()
        )
    }

    fn flush(&self) {
        use std::io::{stderr, Write};
        stderr().flush().ok();
    }
}

impl SELog {
    /// Create new SELog.
    pub fn new() -> Self {
        Self::default()
    }

    // Set loglevel.
    pub fn level<T>(mut self, level: T) -> Self
    where
        T: Into<LevelFilter>,
    {
        self.level = level.into();
        self
    }

    /// Set `Colorize`.
    pub fn colorize(mut self, colorize: Colorize) -> Self {
        self.pallet = match colorize {
            Colorize::Off => None,
            Colorize::Auto => self.pallet.unwrap_or(SEPallet::new()).auto(),
            Colorize::On => Some(self.pallet.unwrap_or(SEPallet::new())),
        };
        self
    }

    /// Set color pallet.
    pub fn color(mut self, pallet: SEPallet) -> Self {
        self.pallet = match self.pallet {
            Some(_) => Some(pallet),
            None => None,
        };
        self
    }

    /// Create `Style` from `SELog` and `log::Level`.
    pub fn style(&self, level: Level) -> Style {
        match self.pallet {
            Some(p) => p.style(level).bold(),
            None => Style::new(),
        }
    }

    /// Set logger as active.
    pub fn init(&self) -> Result<(), SetLoggerError> {
        set_max_level(self.level);
        set_boxed_logger(Box::new(self.clone()))
    }
}
