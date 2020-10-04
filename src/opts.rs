//! The Integration with Clap(v3)
extern crate clap;

/// The macro to define struct which implements `clap::Clap` trait.
#[macro_export]
macro_rules! opts {
    (
        $(#[$attr:meta])*
        $pub:vis struct $name:ident {
        $($(#[$subattr:meta])* $fpub:vis $field:ident: $type:ty,)*
        }
    ) => {
        use clap::Clap;

        $(#[$attr])*
        #[derive(Clap)]
        #[clap(version = clap::crate_version!(),
               author = clap::crate_authors!(),
               about = clap::crate_description!())]
        $pub struct $name {
            $($(#[$subattr])* $fpub $field: $type,)*
        }
    };
}
