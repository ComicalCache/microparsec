use std::fmt::Display;

use crate::{Context, Failure, Parser, ParserType};

/// Runs a given parser on the context, if fails, returns a custom error message
/// ### Arguments
/// * `parser` - The parser to run
/// * `expected` - The error message
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{string, expect, parse_str};
///
/// let res = parse_str::<String, String>("Hallo Welt", expect(string("Hello World"), "\"Hello World\""));
/// assert_eq!(res.unwrap_err().get_error_message(), "[Parser error] Expected `\"Hello World\"` at position: 0");
/// ```
pub fn expect<T: 'static, F: 'static + Display, S: AsRef<str>>(
    parser: Parser<T, F>,
    expected: S,
) -> Parser<T, F> {
    let expected = expected.as_ref().to_string();

    Box::new(move |ctx: Context| match parser(ctx.clone()) {
        Ok(res) => Ok(res),
        Err(err) => Err(Failure::new(
            format!("{expected}"),
            err.ctx,
            Some(ParserType::Expect),
        )),
    })
}
