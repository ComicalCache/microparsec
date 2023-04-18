use std::fmt::Display;

use crate::{any, failure_type_clone, Parser, ParserType};

/// # Either parser
/// Parses either of the two given parsers and returns either the first to match or an error if both failed
/// ### Arguments
/// * `parser_a` - The first parser to parse for
/// * `parser_b` - The second parser to parse for
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{either, string, parse_str};
///
/// let res = parse_str::<String, String>("Hello World", either(string("Hallo Welt"), string("Hello World")));
/// assert_eq!(res.unwrap().val, "Hello World");
/// ```
pub fn either<T: 'static, F: 'static + Display + Clone>(
    parser_a: Parser<T, F>,
    parser_b: Parser<T, F>,
) -> Parser<T, F> {
    failure_type_clone(any!(parser_a, parser_b), Some(ParserType::Either::<F>))
}
