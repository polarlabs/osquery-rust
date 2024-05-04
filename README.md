# osquery-rust

By providing Rust bindings for Osquery this crate facilitates the implementation of Osquery extensions. 
The crate, published on [crates.io](https://crates.io/crates/osquery-rust), contributes to polarlabs mission by enabling any developer 
to extend Osquery easily without sacrificing performance and security.

# Known issues / limitations 
- 🚧 An Osquery extension consists of one or multiple plugins. Version 0.1.x is limited to table plugins, 
  other plugin types such as config or logger are not supported (yet).  
- 🐧 Version 0.1.x has been tested on Linux only.

# Roadmap and future considerations
## Version 0.5.0
- Add support for Windows.

## Version 0.4.0
- Support config plugins.

## Version 0.3.0
- Support logging plugins.
- Provide a tutorial to explain Osquery and how to extend its functionality with `osquery-rust`.
- Automate building and testing.

## Version 0.2.0
- Refine API: think about a trait to avoid the requirement 
  to define both `ColumnDef` and `ExtensionPluginResponse` consistently.
- Minimize dependencies: users of `osquery-rust` should only be required to use `osquery-rust`.
- Automate building of Docker images for Osquery.

## Version 0.1.2
- Update to thrift-rust bindings to communicate with Osquery via Unix Domain Sockets.

#  Project structure
Besides the library itself, `osquery-rust` offers additional value:

- 🐋 Docker images of Osquery covering different platforms, various Linux distributions and 
  up to date as well as outdated Osquery versions.  
- 🪺 examples to showcase how to use `osquery-rust`.  
- 🚀 `osquery-rust` bindings: the crate published at [crates.io](https://crates.io/crates/osquery-rust) used by Osquery developers 
  to implement their own Osquery extension.  
- 🧞 `osquery-rust-codegen`: a helper crate to generate code via macros, this is not meant to be used 
  directly by Osquery extension developers.  
- 🦘 tutorial: get familiar with Osquery and jump start into implementing an Osquery extension with `osquery-rust`.  

# Related projects
polarlabs plans to implement valuable Osquery extensions as separate projects. So, stay tuned 🎸.

This project contributed the support for Unix Domain Sockets to [Apache Thrift's Rust crate](https://issues.apache.org/jira/browse/THRIFT-5283).

# Additional resources

- Homepage polarlabs: [polarlabs.io](https://www.polarlabs.io)
- Tutorial: [osquery-rust tutorial](https://github.com/polarlabs/osquery-rust/tree/main/tutorial)
- Examples: [osquery-rust by example](https://github.com/polarlabs/osquery-rust/tree/main/examples)
- Crate: [crates.io/osquery-rust](https://crates.io/crates/osquery-rust)
- Lib documentation: [docs.rs/osquery-rust](https://docs.rs/osquery-rust/)
- Docker: [images @ docker hub](https://hub.docker.com/r/polarlabs/osquery)

# Links

- [Osquery's GitHub repo](https://github.com/osquery/osquery)