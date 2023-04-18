use std::fmt::Display;

use crate::{failure_type_clone, Parser, ParserType};

use super::regex;

/// # Integer parser
/// Parses for an integer
/// # Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{integer, parse_str};
///
/// let res = parse_str::<String, String>("123", integer());
/// assert_eq!(res.unwrap().val, "123");
/// ```
pub fn integer<F: 'static + Display + Clone>() -> Parser<String, F> {
    failure_type_clone(regex(r"\d+", "integer"), Some(ParserType::Integer::<F>))
}
