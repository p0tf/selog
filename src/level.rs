//! Define `SELevel` and implement methods.
use log::LevelFilter;

/// Simple wrapper of `log::LevelFilter`.
///
/// | `off` | `quiet` | `verbose` | `debug` | `LevelFilter` |
/// |:-----:|:-------:|:---------:|:-------:|:-------------:|
/// |  true |       - |         - |       - |         `Off` |
/// | false |    true |         - |       - |       `Error` |
/// | false |   false |     false |   false |        `Warn` |
/// | false |   false |      true |   false |        `Info` |
/// | false |   false |     false |    true |       `Debug` |
/// | false |   false |      true |    true |       `Trace` |
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SELevel {
    off: bool,
    quiet: bool,
    verbose: bool,
    debug: bool,
}

impl Default for SELevel {
    fn default() -> Self {
        SELevel {
            off: false,
            quiet: false,
            verbose: false,
            debug: false,
        }
    }
}

impl From<SELevel> for LevelFilter {
    fn from(level: SELevel) -> Self {
        if level.off {
            LevelFilter::Off
        } else if level.quiet {
            LevelFilter::Error
        } else if level.verbose && level.debug {
            LevelFilter::Trace
        } else if level.verbose {
            LevelFilter::Info
        } else if level.debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Warn
        }
    }
}

impl SELevel {
    /// Create new instance. (default level is `Warn`.)
    pub fn new() -> Self {
        Self::default()
    }

    /// Turn off all outputs.
    pub fn off(mut self, off: bool) -> Self {
        self.off = off;
        self
    }

    /// Silence warnings.
    pub fn quiet(mut self, quiet: bool) -> Self {
        self.quiet = quiet;
        self
    }

    /// More outputs.
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Enable debug outputs.
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
}
