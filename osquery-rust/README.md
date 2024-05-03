This crate facilitates the development of Osquery extensions in Rust. Osquery communicates 
with its extensions via an interface based on Apache Thrift. So, to put it simply, 
`osquery-rust` is a wrapper around that interface and turns it into a powerful API.

The crate is still in its early stages. However, it already works and got some traction in the 
beginning of 2024. Don't hesitate to create an issue on GitHub if it does not work as expected, 
you are missing something or have ideas on how to improve the API.

To get you inspired, have a look at the [examples](https://github.com/polarlabs/osquery-rust/tree/main/examples).

# Roadmap

:pencil: An extension might provide multiple plugins. Not sure, how to do this 
with `osquery-rust`.

:pencil: Today, `osquery-rust` is limited to table plugins. Osquery knows other plugins as well, 
such as config and logger.

:pencil: Rethink API and align with Rust API Guidelines.

:pencil: Improve documentation and provide a tutorial.

:heavy_check_mark: Publish a first example.

:heavy_check_mark: Publish a working version of `osquery-rust` on crates.io based PR merged into thrift 0.17.

:heavy_check_mark: MVP, communicates with Osquery via socket and provides a table with data from /proc/meminfo.

:heavy_check_mark: Done: PR to Thrift to contribute the support for unix domain sockets to upstream. This was 
a preliminary to this project.

# Naming of this crate

We do not follow Rust's naming convention for crates as usage of '-' and '_' in package and crate 
names is not fully clear. So we follow the naming convention of Osquery. The Go bindings are 
called `osquery-go`. So, we named this crate `osquery-rust`.

# Tools

Thank you to all who build or contributed to these tools, as they enabled us to build this.

- Osquery
- Thrift
- Rust and Cargo
- Clap (Rust library to define and parse CLI)
- Manjaro Linux
- git
- Intellij Idea and RustRover

# Links

[Osquery GitHub repo](https://github.com/osquery)

[Osquery Go bindings](https://github.com/osquery/osquery-go)

[Clap](https://docs.rs/clap/latest/clap/)
