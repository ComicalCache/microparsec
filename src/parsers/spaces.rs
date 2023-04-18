use crate::{regex, Context, Parser, ParserType};

/// # Spaces parser
/// Parses for at least one and as many spaces as possible
/// # Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// #[macro_use] extern crate parse_me;
/// use parse_me::{spaces, string, parse, sequence};
///
/// let res = parse("Hello World", sequence!(string("Hello"), spaces(), string("World")));
///
/// assert_eq!(
///     res.unwrap().val,
///     vec!["Hello".to_string(), " ".to_string(), "World".to_string()]
/// );
/// ```
pub fn spaces() -> Parser<String> {
    Box::new(move |ctx: Context| match regex("[ ]+", "spaces")(ctx) {
        Ok(res) => Ok(res),
        Err(mut err) => {
            err.p_type_stack.push(ParserType::Spaces);
            Err(err)
        }
    })
}
