use std::fmt::Display;

use crate::{string_utils::StringUtils, Context, Failure, Parser, ParserType, Success};

/// # String parser
/// Parses for a given target string
/// ### Arguments
/// * `target` - The target string to parse for
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{string, parse_str};
///
/// let res = parse_str::<String, String>("Hello World", string("Hello World"));
/// assert_eq!(res.unwrap().val, "Hello World");
/// ```
pub fn string<F: 'static + Display, S: AsRef<str>>(target: S) -> Parser<String, F> {
    let target = target.as_ref().to_string();

    Box::new(move |mut ctx: Context| {
        if ctx.txt.slice(ctx.pos..).starts_with(&target) {
            ctx.pos += target.len();
            return Ok(Success::new(target.clone(), ctx));
        }

        return Err(Failure::new(
            format!("{}", target.clone()),
            ctx,
            Some(ParserType::String),
        ));
    })
}
