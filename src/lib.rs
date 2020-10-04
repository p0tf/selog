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
//! Tested on `clap 3.0.0-beta.2`
//!
//! ```rust
//! #[macro_use]
//! extern crate log;
//! extern crate clap;
//! extern crate selog;
//!
//! use clap::Clap;
//! use selog::{Colorize, SELevel, SELog};
//!
//! #[derive(Clap)]
//! struct Opts {
//!     #[clap(short, long, about = "More verbose output.")]
//!     verbose: bool,
//!     #[clap(short, long, about = "Less output.")]
//!     quiet: bool,
//!     #[clap(short, long, about = "Output debug log.")]
//!     debug: bool,
//!     #[clap(long, about = "Control color of output.",
//!            possible_values = &["off", "auto", "on"],
//!            default_value = "auto")]
//!     color: Colorize,
//!     // Your options...
//! }
//!
//! fn main() {
//!     let opts = Opts::parse();
//!
//!     SELog::new()
//!         .level(
//!             SELevel::new()
//!                 .verbose(opts.verbose)
//!                 .quiet(opts.quiet)
//!                 .debug(opts.debug),
//!         )
//!         .colorize(opts.color)
//!         .init()
//!         .unwrap();
//!
//!     error!("Failed something.");
//!
//!     // ...
//! }
//! ```
//!
//! with `opts` feature:
//!
//! ```rust
//! #[macro_use]
//! extern crate log;
//! #[macro_use]
//! extern crate selog;
//!
//! use selog::{Colorize, SELevel, SELog};
//!
//! opts! {
//!     struct Opts {
//!         // Your options...
//!     }
//! }
//!
//! fn main() {
//!     let opts = Opts::parse();
//!
//!     SELog::new()
//!         .level(
//!             SELevel::new()
//!                 .verbose(opts.verbose)
//!                 .quiet(opts.quiet)
//!                 .debug(opts.debug),
//!         )
//!         .colorize(opts.color)
//!         .init()
//!         .unwrap();
//!
//!     error!("Failed something.");
//!
//!     // ...
//! }
//!
//! ```
extern crate ansi_term;
extern crate log;

#[cfg(test)]
mod tests;

pub mod color;
pub mod level;

#[cfg(feature = "opts")]
pub mod opts;

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
