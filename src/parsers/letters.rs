use crate::{Context, Parser, ParserType};

use super::regex;

/// # Letters parser
/// Parses for at least one letter
/// # Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{letters, parse};
///
/// let res = parse("Hello", letters());
/// assert_eq!(res.unwrap().val, "Hello");
/// ```
pub fn letters() -> Parser<String> {
    Box::new(
        move |ctx: Context| match regex("[a-zA-Z]+", "letters")(ctx) {
            Ok(res) => Ok(res),
            Err(mut err) => {
                err.p_type_stack.push(ParserType::Letters);
                Err(err)
            }
        },
    )
}
