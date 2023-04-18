use std::fmt::Display;

use crate::{failure_type_clone, Parser, ParserType};

use super::regex;

/// # Letters parser
/// Parses for at least one letter
/// # Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{letters, parse_str};
///
/// let res = parse_str::<String, String>("Hello", letters());
/// assert_eq!(res.unwrap().val, "Hello");
/// ```
pub fn letters<F: 'static + Display + Clone>() -> Parser<String, F> {
    failure_type_clone(
        regex("[a-zA-Z]+", "letters"),
        Some(ParserType::Letters::<F>),
    )
}
