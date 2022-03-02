//! # osquery bindings for rust
//!
//! ## Getting started
//! ```
//! use osquery_rust::prelude::*;
//! ```
//!

// Allow access to osquery API throughout the osquery-rust bindings
// but not to users of the library.
pub(crate) mod _osquery;

mod client;
pub mod plugin;
mod server;

// Re-exports
pub type ExtensionResponse = _osquery::osquery::ExtensionResponse;
pub type ExtensionPluginRequest =_osquery::osquery::ExtensionPluginRequest;
pub type ExtensionPluginResponse =_osquery::osquery::ExtensionPluginResponse;
pub type ExtensionStatus =_osquery::osquery::ExtensionStatus;

pub use crate::server::server::Server;

///
/// Expose all structures required in virtually any osquery plugin
///
/// ```
/// use osquery_rust::{prelude::*, *};
/// ```
pub mod prelude {
    pub use crate::{ExtensionResponse, ExtensionPluginRequest, ExtensionPluginResponse, ExtensionStatus};
    pub use crate::{Server};
    pub use clap::{Parser};
}


macro_rules! codegen_reexport {
    ($name:ident) => {
        #[cfg(feature = "macros")]
        #[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
        pub use osquery_rust_codegen::$name;
    };
}

codegen_reexport!(args);
//codegen_reexport!(parse);
