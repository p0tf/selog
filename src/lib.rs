//! A Simple, Easy LOGger.
//!
//! > CAUTION: This crate is compatible with `log ^0.4.11`.
//!
//! This crate provides a simple, minimal and easy to use
//! implementation of [`log`](https://docs.rs/log) crate.
extern crate log;

use log::{
    Log, set_boxed_logger, SetLoggerError,
    LevelFilter, set_max_level,
    Metadata, Record
};

/// The logger implementer.
///
/// ```
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SELog;

impl SELog {
    /// Create a new instance.
    pub fn new() -> Self {
        SELog
    }

    /// Set this logger active.
    pub fn set(self) -> Result<(), SetLoggerError> {
        set_max_level(LevelFilter::max());
        set_boxed_logger(Box::new(self))
    }
}

impl Log for SELog {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, _record: &Record) {
        eprintln!("Unimplemented");
    }

    fn flush(&self) {}
}
