use proc_macro::TokenStream;

use std::fs;
use std::str::FromStr;

/// Defines a CLI for an osquery-rust based extension which is compliant to osquery interface.
///
/// # Example
/// ```
/// #[osquery_rust::args]
/// fn main() -> std::io::Result<()> {
///    // Args available via procedural macro
///    let args = Args::parse();
///
///    Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn args(_attr: TokenStream, input: TokenStream) -> TokenStream {
    //todo: error handling!
    let s = fs::read_to_string("osquery-rust-codegen/src/cli.rs").unwrap();
    let mut output: TokenStream = TokenStream::from_str(s.as_ref()).unwrap();

    output.extend(input);
    output
}
