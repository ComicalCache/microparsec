use crate::{Context, Parser, ParserType, Success};

/// # Sequence parser
/// Parses for a sequence of parsers.
///
/// Convenience macro, works identical to `sequence()` but without having to manually create a vector.
/// ### Arguments
/// * `parsers` - The parsers to parse for
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// #[macro_use] extern crate parse_me;
/// use parse_me::{sequence, string, spaces, parse};
///
/// let res = parse("Hello World", sequence!(string("Hello"), spaces(), string("World")));
/// assert_eq!(
///     res.unwrap().val,
///     vec!["Hello".to_string(), " ".to_string(), "World".to_string()]
/// );
/// ```
#[macro_export]
macro_rules! sequence {
    ($($p:expr),+) => {
        sequence(vec![$($p),*])
    };
}

/// # Sequence parser
/// Parses for a sequence of parsers
/// ### Arguments
/// * `parsers` - The parsers to parse for
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{sequence, string, spaces, parse};
///
/// let res = parse("Hello World", sequence(vec![string("Hello"), spaces(), string("World")]));
/// assert_eq!(
///     res.unwrap().val,
///     vec!["Hello".to_string(), " ".to_string(), "World".to_string()]
/// );
/// ```
pub fn sequence<T: 'static>(parsers: Vec<Parser<T>>) -> Parser<Vec<T>> {
    Box::new(move |mut ctx: Context| {
        let mut result = Vec::new();
        for parser in parsers.iter() {
            match parser(ctx.clone()) {
                Ok(res) => {
                    ctx = res.ctx;
                    result.push(res.val);
                }
                Err(mut err) => {
                    err.p_type_stack.push(ParserType::Sequence);
                    return Err(err);
                }
            };
        }

        return Ok(Success::new(result, ctx));
    })
}
