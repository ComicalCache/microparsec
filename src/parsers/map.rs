use crate::{Context, Failure, Parser, ParserType, Success};

/// # Map parser
/// Maps the result of a parser to a new value
/// ### Arguments
/// * `parser` - The parser to map
/// * `mapper` - The function to map the result of the parser
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// #[macro_use] extern crate parse_me;
/// use parse_me::{map, sequence, string, integer, parse};
///
/// let res = parse(
///     "Hello World",
///     map(
///         sequence!(string("Hello"), string(" "), string("World")),
///         |res| Ok(res.val.join("")),
///     ),
/// );
/// assert_eq!(res.unwrap().val, "Hello World");
///
/// let res = parse("234", map(integer(), |res| Ok(res.val.parse::<usize>().unwrap())));
/// assert_eq!(res.unwrap().val, 234);
/// ```
pub fn map<T: 'static, M: 'static>(
    parser: Parser<T>,
    mapper: fn(Success<T>) -> Result<M, String>,
) -> Parser<M> {
    Box::new(move |ctx: Context| {
        let res = match parser(ctx.clone()) {
            Ok(res) => res,
            Err(mut err) => {
                err.p_type_stack.push(ParserType::Map);
                return Err(err);
            }
        };

        let ctx = res.ctx.clone();
        match mapper(res) {
            Ok(mapped) => Ok(Success::new(mapped, ctx)),
            Err(map_err) => Err(Failure::new(map_err, ctx, vec![ParserType::Map])),
        }
    })
}
