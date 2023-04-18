#![allow(unused_macros)]

mod string_utils;

mod types;
use std::fmt::Display;

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
/// let res = parse_from_context::<String, String>(Context::from("Hello World"),
///     map(sequence!(string("Hello"), spaces(), string("World")),
///         |r| Ok(r.val.join("")),
///     ),
/// );
///
/// assert_eq!(res.unwrap().val, "Hello World");
/// ```
pub fn parse_from_context<T, F: Display>(
    ctx: Context,
    parser: Parser<T, F>,
) -> Result<Success<T>, Failure<F>> {
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
pub fn parse<T, F: Display>(txt: String, parser: Parser<T, F>) -> Result<Success<T>, Failure<F>> {
    parse_from_context(Context::from(txt), parser)
}

/// Runs a given parser on a given &str
/// ### Arguments
/// * `txt` - The text to parse
/// * `parser` - The parser to run
/// ### Returns
/// * `Result<Success, String>` containing the result of the parser or the error message
/// ## Example
/// ```
/// #[macro_use] extern crate parse_me;
/// use parse_me::{map, parse_str, string, spaces, sequence};
///
/// let res = parse_str::<String, String>("Hello World",
///     map(sequence!(string("Hello"), spaces(), string("World")),
///         |r| Ok(r.val.join("")),
///     ),
/// );
///
/// assert_eq!(res.unwrap().val, "Hello World");
/// ```
pub fn parse_str<T, F: Display>(txt: &str, parser: Parser<T, F>) -> Result<Success<T>, Failure<F>> {
    parse_from_context(Context::from(txt), parser)
}
