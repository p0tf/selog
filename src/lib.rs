//! Simple, Easy LOGger logs to StdErr.
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

/// The SELog struct.
#[derive(Clone)]
pub struct SELog {
    level: LevelFilter,
    pallet: Option<SEPallet>,
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
        SELog {
            level: LevelFilter::Warn,
            pallet: SEPallet::new().auto(),
        }
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
            Colorize::On => Some(self.pallet.unwrap_or(SEPallet::new())),
            Colorize::Off => None,
            Colorize::Auto => self.pallet.unwrap_or(SEPallet::new()).auto(),
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
