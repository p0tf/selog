//! Simple, Easy LOGger logs to StdErr.
extern crate log;

use log::{set_boxed_logger, set_max_level, LevelFilter, Log, Metadata, Record, SetLoggerError};

/// The SELog struct.
#[derive(Clone)]
pub struct SELog;

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
        SELog
    }

    /// Set logger as active.
    pub fn init(&self) -> Result<(), SetLoggerError> {
        set_max_level(LevelFilter::max());
        set_boxed_logger(Box::new(self.clone()))
    }
}
