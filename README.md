# osquery-rust

By providing Rust bindings for osquery this crate facilitates the implementation of osquery extensions. The crate, published on [crates.io](https://crates.io/crates/osquery-rust), contributes to polarlabs mission by enabling any developer to extend osquery easily without sacrificing performance and security.

# Known issues / limitations
- 🏗️ Version 0.1.1 does not work out of the box because the thrift crate we rely on does not support Unix Domain Sockets. We brought forward a [PR to the thrift-rust](https://github.com/apache/thrift/pull/2545) bindings but as of 13th of Mar '22 it's still in progress.  
- 🚧 An osquery extension consists of one or multiple plugins. Version 0.1.x is limited to table plugins, other plugin types such as config or logger are not supported (yet).  
- 🐧 Version 0.1.x has been testet on Linux only.

# Roadmap and future considerations
## Version 0.4.0
- Support config plugins

## Version 0.3.0
- Support logging plugins

## Version 0.2.0
- Add support for Windows
- Automate building and testing
- Provide a tutorial to explain osquery and how to extend its functionality with `osquery-rust`

## Version 0.1.2
- Update to thrift-rust bindings to communicate with osquery via Unix Domain Sockets

#  Project structure
Besides the library itself, `osquery-rust` offers additional value:

- 🐋 docker images of osquery covering different platforms, various Linux distributions and up to date as well as outdated osquery versions.  
- 🪺 examples to showcase how to use `osquery-rust`.  
- 🚀 `osquery-rust` bindings: the crate published at [crates.io](https://crates.io/crates/osquery-rust) used by osquery developers to implement their own osquery extension.  
- 🧞 `osquery-rust-codegen`: a helper crate to generate code via macros, this is not meant to be used directly by osquery extension developers.  
- 🦘 tutorial: get familiar with osquery and jump start into implementing an osquery extension with `osquery-rust`.  

# Related projects
polarlabs plans to implement valuable osquery extensions as separate projects. So, stay tuned 🎸.

This project contributed the support for Unix Domain Sockets to Apache Thrift's Rust crate.

todo: link the issue here + the issue on Apache Thrifts Jira.

# Additional resources

- Homepage polarlabs: [polarlabs.io](https://www.polarlabs.io)
- Tutorial: [osquery-rust tutorial](https://github.com/polarlabs/osquery-rust/tree/main/tutorial)
- Examples: [osquery-rust by example](https://github.com/polarlabs/osquery-rust/tree/main/examples)
- Crate: [crates.io/osquery-rust](https://crates.io/crates/osquery-rust)
- Lib documentation: [docs.rs/osquery-rust](https://docs.rs/osquery-rust/)
- Docker: [images @ docker hub](https://hub.docker.com/r/polarlabs/osquery)

# Links

- [osquery's github repo](https://github.com/osquery/osquery)