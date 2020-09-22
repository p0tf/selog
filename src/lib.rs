//! Simple, Easy LOGger logs to StdErr.
extern crate log;

#[cfg(test)]
mod tests;

pub mod level;

// Re-exports
pub use level::SELevel;

use log::{set_boxed_logger, set_max_level, LevelFilter, Log, Metadata, Record, SetLoggerError};

/// The SELog struct.
#[derive(Clone)]
pub struct SELog {
    level: LevelFilter,
}

impl Log for SELog {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &Record<'_>) {
        eprintln!(
            "{}: {}",
            record.level().to_string().to_lowercase(),
            record.args()
        )
    }

    fn flush(&self) {}
}

impl SELog {
    /// Create new SELog.
    pub fn new() -> Self {
        SELog {
            level: LevelFilter::Warn,
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

    /// Set logger as active.
    pub fn init(&self) -> Result<(), SetLoggerError> {
        set_max_level(self.level);
        set_boxed_logger(Box::new(self.clone()))
    }
}
