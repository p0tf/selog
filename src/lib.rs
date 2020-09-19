//! Simple, Easy LOGger logs to StdErr.
extern crate log;

use log::{Log, Metadata, Record};

/// The SELog struct.
pub struct SELog;

impl Log for SELog {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &Record<'_>) {
        eprintln!("{}: {}", record.level().to_string().to_lowercase(), record.args())
    }

    fn flush(&self) {}
}
