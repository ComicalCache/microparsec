use std::fmt::Display;

use crate::{failure_type_clone, Parser, ParserType};

use super::regex;

/// # Float parser
/// Parses for a float
/// # Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{float, parse_str};
///
/// let res = parse_str::<String, String>("123.456", float());
/// assert_eq!(res.unwrap().val, "123.456");
/// ```
pub fn float<F: 'static + Display + Clone>() -> Parser<String, F> {
    failure_type_clone(regex(r"\d+\.\d*", "float"), Some(ParserType::Float::<F>))
}
