use std::fmt::Display;

use crate::{Context, Failure, Parser, ParserType, Success};

/// # Many parser
/// Parses as many times as possible, returns an error if no parsing was successful
/// ### Arguments
/// * `parser` - The parser to parse for
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// * Look at the `spaces()` parser implementation for an example
pub fn many<T: 'static, F: 'static + Display>(parser: Parser<T, F>) -> Parser<Vec<T>, F> {
    Box::new(move |mut ctx: Context| {
        let mut ret = Vec::new();

        loop {
            match parser(ctx.clone()) {
                Ok(res) => {
                    ctx = res.ctx;
                    ret.push(res.val);
                }
                Err(err) if ret.is_empty() => {
                    return Err(Failure::new(err.exp, err.ctx, Some(ParserType::Many::<F>)))
                }
                Err(_) => return Ok(Success::new(ret, ctx)),
            };
        }
    })
}
