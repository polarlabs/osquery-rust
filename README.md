# osquery-rust
This rust crate provides osquery bindings facilitating the implementation of osquery extensions. An extension consists of one or more plugins.
osquery-rust contributes to polarlabs mission by enabling any developer to extend osquery easily without sacrificing performance or security.

# Known issues / limitations
🏗️ Version 0.1.1 does not work out of the box because the thrift crate we rely on does not support Unix Domain Sockets. We brought forward a [PR to the thrift-rust](https://github.com/apache/thrift/pull/2545) bindings but as of 12th of Mar '22 it's still in progress.  
🚧 Version 0.1.x only table plugins are supported by osquery-rust.  
🐧 Version 0.1.x has been testet on Linux only.

# Roadmap and future considerations
## Version 0.4.0
- Support config plugins

## Version 0.3.0
- Support logging plugins

## Version 0.2.0
- Add support for Windows
- Automate building and testing

## Version 0.1.2
- Update to thrift-rust bindings to communicate with osquery via Unix Domain Socket

#  Project structure
Besides the library itself, osquery-rust offers additional value projects:

🐋 docker images for osquery covering different platforms, various Linux distributions and up to date as well as outdated osquery versions.  
🪺 examples to showcase how to use osquery-rust.  
🚀 osquery-rust bindings: the crate published at [crates.io](https://crates.io/crates/osquery-rust) used by osquery developers to implement their osquery extension.  
🧞 osquery-rust-codegent: a helper crate to generate code via macros, this is not meant to be used by osquery extension developers.  
🦘 tutorial: get familiar with osquery and jump start into implementing an osquery extension with osquery-rust.  

# Related projects
polarlabs plans to implement valuable osquery extensions as separate projects. So, stay tuned 🎸.

# Additional resources

- Homepage polarlabs: [polarlabs.io](https://www.polarlabs.io)
- Lib documentation: [docs.rs/osquery-rust](https://docs.rs/osquery-rust/)
- Tutorial: [osquery-rust tutorial](https://github.com/polarlabs/osquery-rust/tree/main/tutorial)
- Examples: [osquery-rust by example](https://github.com/polarlabs/osquery-rust/tree/main/examples)
- Docker: [images @ docker hub](https://hub.docker.com/r/polarlabs/osquery)

# Links

- [osquery's github repo](https://github.com/osquery/osquery)