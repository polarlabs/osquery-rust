//! osquery-rust strives to make osquery extension development a breeze. If you have ideas how
//! to improve developer experience, reach out to us on [github](https://github.com/polarlabs).
//!
//! If you encounter any issue with this crate, please raise an [issue](https://github.com/polarlabs/osquery-rust/issues).
//! We are here to support you and your venture.
//!
//! As this is the lib's documentation we will focus on the lib itself. However, osquery-rust is more than
//! just the lib. Please check out the project's [README on github](https://github.com/polarlabs/osquery-rust) to
//! see the whole picture.
//!
//! ## Include osquery-rust in your rust project
//!
//! Make sure to include osquery-rust as a dependency in your Cargo.toml. As osquery-rust is in its early
//! stages and might evolve fast, please check for the latest version often. We adhere to semver. So you can
//! rely on caret notation when selecting the version.
//!
//! ```
//! [dependencies]
//! osquery-rust = "^0.1"
//! ```
//!
//! ## Counter example
//! todo: what is the counter example? where does osquery_rust::args come from?
//! ```
//! use osquery_rust::prelude::*;
//!
//! #[osquery_rust::args]
//! fn main() -> std::io::Result<()> {
//!
//! }
//!

#![forbid(unsafe_code)]

// Restrict access to osquery API to osquery-rust
// Users of osquery-rust are not allowed to access osquery API directly
pub(crate) mod _osquery;
pub(crate) mod client;
pub mod plugin;
pub(crate) mod server;

// Re-exports
pub type ExtensionResponse = _osquery::osquery::ExtensionResponse;
pub type ExtensionPluginRequest =_osquery::osquery::ExtensionPluginRequest;
pub type ExtensionPluginResponse =_osquery::osquery::ExtensionPluginResponse;
pub type ExtensionStatus =_osquery::osquery::ExtensionStatus;

pub use crate::server::Server;

///
/// Expose all structures required in virtually any osquery extension
///
/// ```
/// use osquery_rust::{prelude::*, *};
/// ```
pub mod prelude {
    pub use crate::{ExtensionResponse, ExtensionPluginRequest, ExtensionPluginResponse, ExtensionStatus};
    pub use crate::{Server};
    pub use clap::{Parser};
}

// Generates code to allow using macros for code generation
macro_rules! codegen_reexport {
    ($name:ident) => {
        #[cfg(feature = "macros")]
        #[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
        pub use osquery_rust_codegen::$name;
    };
}

// Provide helper code from osquery-rust-codegen to define CLI interface of osquery extension
codegen_reexport!(args);
