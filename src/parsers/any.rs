use std::fmt::Display;

use crate::{Context, Failure, Parser, ParserType};

/// # Any parser
/// Parses for any of the given parsers and returns the first successful result, or an error if no parser matched
///
/// Convenience macro, works identical to `any()` but without having to manually create a vector.
/// ### Arguments
/// * `parsers` - The parsers to parse for
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// #[macro_use] extern crate parse_me;
/// use parse_me::{any, string, parse_str};
///
/// let res = parse_str::<String, String>("Hello World", any!(string("Hallo"), string("Hello")));
/// assert_eq!(res.unwrap().val, "Hello");
/// ```
#[macro_export]
macro_rules! any {
    ($($p:expr),+) => {
        any(vec![$($p),*])
    };
}

/// # Any parser
/// Parses for any of the given parsers and returns the first successful result, or an error if no parser matched
/// ### Arguments
/// * `parsers` - The parsers to parse for
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{any, string, parse_str};
///
/// let res = parse_str::<String, String>("Hello World", any(vec![string("Hallo"), string("Hello")]));
/// assert_eq!(res.unwrap().val, "Hello");
/// ```
pub fn any<T: 'static, F: 'static + Display>(parsers: Vec<Parser<T, F>>) -> Parser<T, F> {
    Box::new(move |ctx: Context| {
        let mut errs = Vec::new();

        for parser in parsers.iter() {
            match parser(ctx.clone()) {
                Ok(res) => return Ok(res),
                Err(err) => errs.push(err.exp),
            }
        }

        return Err(Failure::new(
            format!("{{ `{}` }}", errs.join("` | `")),
            ctx,
            Some(ParserType::Any),
        ));
    })
}
