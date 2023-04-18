use std::fmt::Display;

use crate::{Parser, Context, Success};

/// # Optional parser
/// Tries to parse the given parser, but if it fails, it returns a successful result with a None value
/// ### Arguments
/// * `parser` - The parser to try to parse
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{optional, string, parse_str};
///
/// let res = parse_str::<Option<String>, String>("Hello World", optional(string("Hello World")));
/// assert_eq!(res.unwrap().val.unwrap(), "Hello World");
///
/// let res = parse_str::<Option<String>, String>("Hello World", optional(string("Hallo World")));
/// assert_eq!(res.unwrap().val.is_none(), true);
/// ```
pub fn optional<T: 'static, F: 'static + Display>(parser: Parser<T, F>) -> Parser<Option<T>, F> {
    Box::new(move |ctx: Context| match parser(ctx.clone()) {
        Ok(res) => Ok(Success::new(Some(res.val), res.ctx)),
        Err(_) => Ok(Success::new(None, ctx)),
    })
}