use crate::{Context, Parser, ParserType, Success};

/// # Forget parser
/// "Forgets" the success value type and changes it to `()`
/// ### Arguments
/// * `parser` - The parser to parse which value to forget
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{forget, string, parse};
///
/// let res = parse("Hello World", forget(string("Hello World")));
/// assert_eq!(res.unwrap().val, ());
/// ```
pub fn forget<T: 'static>(parser: Parser<T>) -> Parser<()> {
    Box::new(move |ctx: Context| match parser(ctx) {
        Ok(res) => Ok(Success::new((), res.ctx)),
        Err(mut err) => {
            err.p_type_stack.push(ParserType::Forget);
            Err(err)
        }
    })
}
