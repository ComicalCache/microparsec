use crate::{Context, Parser, ParserType};

use super::regex;

/// # Float parser
/// Parses for a float
/// # Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{float, parse};
///
/// let res = parse("123.456", float());
/// assert_eq!(res.unwrap().val, "123.456");
/// ```
pub fn float() -> Parser<String> {
    Box::new(move |ctx: Context| {
        match regex(r"\d+\.\d*", "float")(ctx) {
            Ok(res) => Ok(res),
            Err(mut err) => {
                err.p_type_stack.push(ParserType::Float);
                Err(err)
            }
        }
    })
}
