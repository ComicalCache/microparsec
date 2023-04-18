#![allow(unused_macros)]

mod string_utils;

mod types;

pub use types::*;

mod parsers;
pub use parsers::*;

/// Runs a given parser on a given context
/// ### Arguments
/// * `ctx` - The context to parse
/// * `parser` - The parser to run
/// ### Returns
/// * `Result<Success, String>` containing the result of the parser or the error message
/// ## Example
/// ```
/// #[macro_use] extern crate parse_me;
/// use parse_me::{map, parse_from_context, string, spaces, sequence, Context};
///
/// let res = parse_from_context(Context::from("Hello World"),
///     map(sequence!(string("Hello"), spaces(), string("World")),
///         |r| Ok(r.val.join("")),
///     ),
/// );
///
/// assert_eq!(res.unwrap().val, "Hello World");
/// ```
pub fn parse_from_context<T>(ctx: Context, parser: Parser<T>) -> Result<Success<T>, Failure> {
    match parser(ctx) {
        Ok(res) => Ok(res),
        Err(err) => Err(err),
    }
}

/// Runs a given parser on a given string
/// ### Arguments
/// * `txt` - The text to parse
/// * `parser` - The parser to run
/// ### Returns
/// * `Result<Success, String>` containing the result of the parser or the error message
/// ## Example
/// ```
/// #[macro_use] extern crate parse_me;
/// use parse_me::{map, parse, string, spaces, sequence};
///
/// let res = parse::<String, String>(String::from("Hello World"),
///     map(sequence!(string("Hello"), spaces(), string("World")),
///         |r| Ok(r.val.join("")),
///     ),
/// );
///
/// assert_eq!(res.unwrap().val, "Hello World");
/// ```
pub fn parse<S: AsRef<str>, T>(txt: S, parser: Parser<T>) -> Result<Success<T>, Failure> {
    parse_from_context(Context::from(txt), parser)
}