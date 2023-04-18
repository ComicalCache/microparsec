use crate::{Context, Failure, Parser, ParserType, Pos};

/// # Exact parser
/// Attemts to parse a specified number of chars or to the EOI and fails otherwise
/// ### Arguments
/// * `parser` - The parser to parse with
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{exact, string, parse, Pos};
///
/// let res = parse("Hello World", exact(string("Hello World"), Pos::EOI));
/// assert_eq!(res.unwrap().val, "Hello World");
/// ```
pub fn exact<T: 'static>(parser: Parser<T>, pos: Pos) -> Parser<T> {
    Box::new(move |ctx: Context| {
        let prev_pos = ctx.pos;
        let mut res = match parser(ctx) {
            Ok(res) => res,
            Err(mut err) => {
                err.p_type_stack.push(ParserType::Exact);
                return Err(err);
            }
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
                        vec![ParserType::Exact],
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
                        vec![ParserType::Exact],
                    ))
                }
            }
        }
    })
}
