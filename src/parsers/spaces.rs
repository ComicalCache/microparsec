use std::fmt::Display;

use crate::{failure_type_clone, Parser, ParserType};

use super::{many, map, string};

/// # Spaces parser
/// Parses for at least one and as many spaces as possible
/// # Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// #[macro_use] extern crate parse_me;
/// use parse_me::{spaces, string, parse_str, sequence};
///
/// let res = parse_str::<Vec<String>, String>(
///     "Hello World",
///     sequence!(string("Hello"), spaces(), string("World")),
/// );
///
/// assert_eq!(
///     res.unwrap().val,
///     vec!["Hello".to_string(), " ".to_string(), "World".to_string()]
/// );
/// ```
pub fn spaces<F: 'static + Display + Clone>() -> Parser<String, F> {
    failure_type_clone(
        map(many(string(" ")), |s| Ok(s.val.join(""))),
        Some(ParserType::Spaces::<F>),
    )
}
