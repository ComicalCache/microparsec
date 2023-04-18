use crate::{Parser, ParserType, Success};

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
/// use parse_me::{between, string, parse};
///
/// let res = parse(
///     "\"Hello\"",
///     between(string("\""), string("Hello"), string("\"")),
/// );
/// assert_eq!(res.unwrap().val, "Hello");
/// ```
pub fn between<N: 'static, T: 'static, M: 'static>(
    front: Parser<N>,
    middle: Parser<T>,
    back: Parser<M>,
) -> Parser<T> {
    Box::new(move |ctx| {
        let ctx = match front(ctx) {
            Ok(res) => res.ctx,
            Err(mut err) => {
                err.p_type_stack.push(ParserType::Between);
                return Err(err);
            }
        };

        let res = match middle(ctx) {
            Ok(res) => res,
            Err(mut err) => {
                err.p_type_stack.push(ParserType::Between);
                return Err(err);
            }
        };

        let ctx = match back(res.ctx) {
            Ok(res) => res.ctx,
            Err(mut err) => {
                err.p_type_stack.push(ParserType::Between);
                return Err(err);
            }
        };

        Ok(Success::new(res.val, ctx))
    })
}
