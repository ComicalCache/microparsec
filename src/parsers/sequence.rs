use std::fmt::Display;

use crate::{Context, Failure, Parser, ParserType, Success};

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
/// use parse_me::{sequence, string, spaces, parse_str};
///
/// let res = parse_str::<Vec<String>, String>("Hello World", sequence!(string("Hello"), spaces(), string("World")));
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
/// use parse_me::{sequence, string, spaces, parse_str};
///
/// let res = parse_str::<Vec<String>, String>("Hello World", sequence(vec![string("Hello"), spaces(), string("World")]));
/// assert_eq!(
///     res.unwrap().val,
///     vec!["Hello".to_string(), " ".to_string(), "World".to_string()]
/// );
/// ```
pub fn sequence<T: 'static, F: 'static + Display>(parsers: Vec<Parser<T, F>>) -> Parser<Vec<T>, F> {
    Box::new(move |mut ctx: Context| {
        let mut result = Vec::new();
        for parser in parsers.iter() {
            match parser(ctx.clone()) {
                Ok(res) => {
                    ctx = res.ctx;
                    result.push(res.val);
                }
                Err(err) => {
                    return Err(Failure::new(
                        err.exp,
                        err.ctx,
                        Some(ParserType::Sequence::<F>),
                    ))
                }
            };
        }

        return Ok(Success::new(result, ctx));
    })
}
