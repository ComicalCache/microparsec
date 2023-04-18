use crate::{Context, Parser, ParserType, Success};

/// # Many parser
/// Parses as many times as possible, returns an error if no parsing was successful
/// ### Arguments
/// * `parser` - The parser to parse for
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// * Look at the `spaces()` parser implementation for an example
pub fn many<T: 'static>(parser: Parser<T>) -> Parser<Vec<T>> {
    Box::new(move |mut ctx: Context| {
        let mut ret = Vec::new();

        loop {
            match parser(ctx.clone()) {
                Ok(res) => {
                    ctx = res.ctx;
                    ret.push(res.val);
                }
                Err(mut err) if ret.is_empty() => {
                    err.p_type_stack.push(ParserType::Many);
                    return Err(err);
                }
                Err(_) => return Ok(Success::new(ret, ctx)),
            };
        }
    })
}
