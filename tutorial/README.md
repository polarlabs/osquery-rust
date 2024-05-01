In this tutorial you will create an osquery extension which implements a table plugin. In osquery, tables are used 
to provide data to its users.

For the time being this tutorial is limited to Linux and Mac OS.

# Prior knowledge

To follow this tutorial you should have 

1. basic programming skills, preferably in Rust.
2. familiarity with osquery

# Prerequisites

To follow this tutorial successfully, you don't need elevated privileges. However, the following prerequisites 
are required:

1. osquery installed.
2. Rust toolchain installed.
3. access to Internet to download dependencies.

## Osquery

To check your osquery installation, execute the following command.

```
$ osqueryi --version
osqueryi version 5.0.1
```

## Rust toolchain

The easiest way to install the Rust toolchain is via [[LINK_TO_RUSTUP|rustup]].

You can check your Rust installation by executing the following commands.

```
$ cargo --version
cargo 1.59.0 (49d8809dc 2022-02-10)
$ rustc --version
rustc 1.59.0 (9d1b2106e 2022-02-23)
```

# Create a Rust project

An osquery extension is a binary. So we need to create a binary crate via cargo.

```
$ cargo new osquery-rust-tutorial
     Created binary (application) `osquery-rust-tutorial` package
```

The project folder ''osquery-rust-tutorial'' consists of two files:

* ./Cargo.toml: describes the crate and its dependencies
* ./src/main.rs: contains the source code, especially the main function, which is started when the binary is started

# Define dependencies

The `osquery-rust` crate provides all you need to implement an osquery extension. However, the more powerfull your 
extension, the more likely you need to refer to other libraries / dependencies.


# Conclusion

I hope you enjoyed this tutorial. You find more examples in the ''examples'' folder.






//! ## Getting started
//!
//! Getting started with `osquery-rust` is straight forward. However, as an osquery extension does not stand alone,
//! but requires osquery, some setup steps are required.
//!
//! ### Install osquery
//!
//! ### Include `osquery-rust` in your rust project
//!
//! Make sure to include `osquery-rust` as a dependency in your Cargo.toml. As `osquery-rust` is evolving fast, please
//! check for the latest version often.
//!
//! ```
//! [dependencies]
//! osquery-rust = "0.1"
//! ```
//!
//! ### Code and compile
//!
//! See example.
//!
//! ### Run osquery with your extension
//!
//!
//!
