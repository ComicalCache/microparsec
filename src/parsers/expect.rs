use crate::{Context, Parser, ParserType};

/// Runs a given parser on the context, if fails, returns a custom error message
/// ### Arguments
/// * `parser` - The parser to run
/// * `expected` - The error message
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{string, expect, parse};
///
/// let res = parse("Hallo Welt", expect(string("Hello World"), "\"Hello World\""));
/// assert_eq!(res.unwrap_err().get_error_message(), "[Parser error] Expected `\"Hello World\"` at position: 0");
/// ```
pub fn expect<T: 'static, S: AsRef<str>>(parser: Parser<T>, expected: S) -> Parser<T> {
    let expected = expected.as_ref().to_string();

    Box::new(move |ctx: Context| match parser(ctx.clone()) {
        Ok(res) => Ok(res),
        Err(mut err) => {
            err.exp = expected.clone();
            err.p_type_stack.push(ParserType::Expect);
            Err(err)
        }
    })
}
