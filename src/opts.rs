//! The Integration with Clap(v3)
extern crate clap;

use crate::{Colorize, SELevel, SELog};
use log::SetLoggerError;

/// A macro to define struct which implements `clap::Clap` trait.
#[macro_export]
macro_rules! opts {
    (
        $(#[$attr:meta])*
        $pub:vis struct $name:ident {
        $($(#[$fattr:meta])* $fpub:vis $field:ident: $type:ty,)*
        }
    ) => {
        use clap::Clap;

        $(#[$attr])*
        #[derive(Clap)]
        #[clap(version = clap::crate_version!(),
               author = clap::crate_authors!(),
               about = clap::crate_description!())]
        $pub struct $name {
            $($(#[$fattr])* $fpub $field: $type,)*
        }
    };
}

/// A trait to initialize `SELog` with struct.
pub trait SEOpts {
    /// Get the value of `verbose`.
    fn verbose(&self) -> bool;

    /// Get the value of `quiet`.
    fn quiet(&self) -> bool;

    /// Get the value of `debug`.
    fn debug(&self) -> bool;

    /// Get the value of `no_output`.
    fn color(&self) -> Colorize;

    /// Initialize `SELog`.
    fn init_log(&self) -> Result<(), SetLoggerError> {
        SELog::new()
            .level(
                SELevel::new()
                    .verbose(self.verbose())
                    .quiet(self.quiet())
                    .debug(self.debug()),
            )
            .colorize(self.color())
            .init()
    }
}
