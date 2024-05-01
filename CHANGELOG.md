# 0.1.2

## Code

- Update to latest Apache Thrift version (0.17.0).
- Update all dependencies to latest version.
- Set #![forbid(unsafe_code)].

## Project

- Adopt Polarlabs branch model.
- Move updated Thrift related documentation to CONTRIBUTING.md.
- Update licensing: now dual licensed, MIT or Apache 2.0

1. Improve README.md for osquery-rust => this is visible on crates.io.
2. Complete review of osquery-rust.
3. Use osquery-rust in an example.
4. Compare to osquery-rust-outdated to cherry-pick missing pieces.
5. Fix issue reported for codegen.
6. Check all open todos in code and documentation, e.g. counter example.
7. Remove all #[allow(unused_imports)] and #[allow(dead_code)].
8. Quality assurance: no clippy warnings, linter!
9. Publish to crates.io: use the publishing process described in Kodiak factory.
