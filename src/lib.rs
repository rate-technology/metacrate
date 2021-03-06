#![warn(missing_docs)]

//! This crate provides macros to attach metadata of the crate.

meta!();

/// Attaches metadata to the crate that called this macros.
#[macro_export]
macro_rules! meta {
    () => {
        #[doc(hidden)]
        /// The metadata of the crate.
        pub mod meta {
            /// Version.
            pub static VERSION: &str = env!("CARGO_PKG_VERSION");
            /// List of authors.
            pub static AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
            /// Name.
            pub static NAME: &str = env!("CARGO_PKG_NAME");
            /// Description.
            pub static DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
        }
    };
}
