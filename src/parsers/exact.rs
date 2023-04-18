use std::fmt::Display;

use crate::{Context, Failure, Parser, ParserType, Pos};

/// # Exact parser
/// Attemts to parse a specified number of chars or to the EOI and fails otherwise
/// ### Arguments
/// * `parser` - The parser to parse with
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{exact, string, parse_str, Pos};
///
/// let res = parse_str::<String, String>("Hello World", exact(string("Hello World"), Pos::EOI));
/// assert_eq!(res.unwrap().val, "Hello World");
/// ```
pub fn exact<T: 'static, F: 'static + Display>(parser: Parser<T, F>, pos: Pos) -> Parser<T, F> {
    Box::new(move |ctx: Context| {
        let prev_pos = ctx.pos;
        let mut res = match parser(ctx) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        match pos {
            Pos::Chars(x) => {
                if res.ctx.pos - prev_pos == x {
                    Ok(res)
                } else {
                    res.ctx.pos = prev_pos;
                    Err(Failure::new(
                        format!("parsing {x} characters"),
                        res.ctx,
                        Some(ParserType::Exact),
                    ))
                }
            }
            Pos::EOI => {
                if res.ctx.pos == res.ctx.txt.len() {
                    Ok(res)
                } else {
                    res.ctx.pos = prev_pos;
                    Err(Failure::new(
                        "parsing to EOI",
                        res.ctx,
                        Some(ParserType::Exact),
                    ))
                }
            }
        }
    })
}
