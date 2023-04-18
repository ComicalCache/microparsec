use crate::{Context, Parser, ParserType};

use super::regex;

/// # Integer parser
/// Parses for an integer
/// # Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{integer, parse};
///
/// let res = parse("123", integer());
/// assert_eq!(res.unwrap().val, "123");
/// ```
pub fn integer() -> Parser<String> {
    Box::new(move |ctx: Context| match regex(r"\d+", "integer")(ctx) {
        Ok(res) => Ok(res),
        Err(mut err) => {
            err.p_type_stack.push(ParserType::Integer);
            Err(err)
        }
    })
}
