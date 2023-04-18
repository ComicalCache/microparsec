use std::fmt::Display;

use crate::{Context, Failure, Parser, ParserType, Success};

/// # Forget parser
/// "Forgets" the success value type and changes it to `()`
/// ### Arguments
/// * `parser` - The parser to parse which value to forget
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{forget, string, parse_str};
///
/// let res = parse_str::<(), String>("Hello World", forget(string("Hello World")));
/// assert_eq!(res.unwrap().val, ());
/// ```
pub fn forget<T: 'static, F: 'static + Display>(parser: Parser<T, F>) -> Parser<(), F> {
    Box::new(move |ctx: Context| match parser(ctx) {
        Ok(res) => Ok(Success::new((), res.ctx)),
        Err(err) => Err(Failure::new(
            err.exp,
            err.ctx,
            Some(ParserType::Forget::<F>),
        )),
    })
}
