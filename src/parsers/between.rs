use std::fmt::Display;

use crate::{Failure, Parser, ParserType, Success};

/// # Between parser
/// Parses between two parsers
/// ### Arguments
/// * `front` - The left parser
/// * `middle` - The parser to parse between the left and right parser
/// * `back` - The right parser
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{between, string, parse_str};
///
/// let res = parse_str::<String, String>(
///     "\"Hello\"",
///     between(string("\""), string("Hello"), string("\"")),
/// );
/// assert_eq!(res.unwrap().val, "Hello");
/// ```
pub fn between<N: 'static, T: 'static, M: 'static, F: 'static + Display>(
    front: Parser<N, F>,
    middle: Parser<T, F>,
    back: Parser<M, F>,
) -> Parser<T, F> {
    Box::new(move |ctx| {
        let ctx = match front(ctx) {
            Ok(res) => res.ctx,
            Err(err) => return Err(Failure::new(err.exp, err.ctx, Some(ParserType::Between))),
        };

        let res = match middle(ctx) {
            Ok(res) => res,
            Err(err) => return Err(Failure::new(err.exp, err.ctx, Some(ParserType::Between))),
        };

        let ctx = match back(res.ctx) {
            Ok(res) => res.ctx,
            Err(err) => return Err(Failure::new(err.exp, err.ctx, Some(ParserType::Between))),
        };

        Ok(Success::new(res.val, ctx))
    })
}
