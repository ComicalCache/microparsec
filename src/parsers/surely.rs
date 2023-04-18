use crate::{Context, Parser, ParserType};

/// # Surely parser
/// If an any parser comes across a surely parser and it fails the any parser immediately fails as well
/// ### Arguments
/// * `parser` - The parser that's surely expected
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{string, parse, surely, any, sequence, spaces};
///
/// let res = parse("Hello Welt", any!(sequence!(string("Hello"), spaces(), surely(string("World"))), sequence!(string("Hallo"), spaces(), surely(string("Welt")))));
/// assert_eq!(res.unwrap_err().get_error_message(), "[Parser error] Expected `surely `World`` at position: 6");
/// ```
pub fn surely<T: 'static>(parser: Parser<T>) -> Parser<T> {
    Box::new(move |ctx: Context| match parser(ctx) {
        Ok(res) => Ok(res),
        Err(mut err) => {
            err.exp = format!("surely `{}`", err.exp);
            err.p_type_stack.push(ParserType::Surely);
            Err(err)
        }
    })
}
