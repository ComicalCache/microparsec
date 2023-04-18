use std::fmt::Display;

use crate::{Context, Failure, Parser, ParserType};

/// # Failure type parser
/// Sets the failure type of the supplied parser to a supplied type
/// ### Arguments
/// * `parser` - The parser which's failure type to change
/// * `p_type` - The failure type to set it to
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{failure_type, string, parse_str, ParserType};
///
/// let res = parse_str::<String, &str>(
///     "Hello World",
///     failure_type(
///         string("Hallo Welt"),
///         Some(ParserType::Custom("Hallo Welt parser")),
///     ));
/// assert_eq!(res.unwrap_err().p_type, Some(ParserType::Custom("Hallo Welt parser")));
/// ```
pub fn failure_type<T: 'static, F: 'static>(
    parser: Parser<T, F>,
    p_type: Option<ParserType<F>>,
) -> Parser<T, F>
where
    F: Copy + Display,
{
    Box::new(move |ctx: Context| match parser(ctx) {
        Ok(res) => Ok(res),
        Err(err) => Err(Failure::new(err.exp, err.ctx, p_type)),
    })
}

/// # Failure type parser
/// Sets the failure type of the supplied parser to a supplied type that only implements clone
/// ### Arguments
/// * `parser` - The parser which's failure type to change
/// * `p_type` - The failure type to set it to
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{failure_type_clone, string, parse_str, ParserType};
///
/// let res = parse_str::<String, String>(
///     "Hello World",
///     failure_type_clone(
///         string("Hallo Welt"),
///         Some(ParserType::Custom("Hallo Welt parser".to_string())),
///     ));
/// assert_eq!(res.unwrap_err().p_type, Some(ParserType::Custom("Hallo Welt parser".to_string())));
/// ```
pub fn failure_type_clone<T: 'static, F: 'static>(
    parser: Parser<T, F>,
    p_type: Option<ParserType<F>>,
) -> Parser<T, F>
where
    F: Clone + Display,
{
    Box::new(move |ctx: Context| match parser(ctx) {
        Ok(res) => Ok(res),
        Err(err) => Err(Failure::new(err.exp, err.ctx, p_type.clone())),
    })
}
