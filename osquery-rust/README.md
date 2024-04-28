This project provides the bindings to osquery for Rust. It's still in its early stages. However, its continuously 
improved. Feel free to create an issue on Github if you are missing something or experience bugs.

Osquery and its plugins communicate via an interface based on Apache Thrift. So,
`osquery-rust` is a wrapper around that interface and turns it into a powerful API.

# Tools

Thank you to all who contributed to these tools, as they enabled me to build this.

- Rust and Cargo
- Manjaro Linux
- Intellij Idea
- Thrift
- osquery
- Clap (Rust library to define and parse CLI)
- git

# Roadmap

- MVP: communicates with osquery via socket and provides a table with data from /proc/meminfo (Done)
- Publish MVP on crates.io
- Improve documentation and provide a tutorial
- Rethink API and align with Rust API Guidelines
- PR to Thrift to contribute the support for unix domain sockets to upstream

# Naming Convention

Do not follow rust naming as usage of - / _ in package / crate names is unclear. So wie follow the naming convention 
of osquery, e.g. osquery-go => Go bindings.

# Links

https://github.com/osquery osquery GitHub repo

https://github.com/osquery/osquery-go osquery Go bindings GitHub repo

https://github.com/clap-rs/clap/blob/v3.0.13/examples/tutorial_derive/README.md Clap is so cool!
