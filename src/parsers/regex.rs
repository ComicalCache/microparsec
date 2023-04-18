use std::fmt::Display;

use regex::Regex;

use crate::{Parser, Context, string_utils::StringUtils, Success, Failure, ParserType};

/// # Regex parser
/// Parses for a given regex pattern
/// ### Arguments
/// * `target` - The target regex pattern
/// * `expected` - A custom error message
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{regex, parse_str};
///
/// let res = parse_str::<String, String>("+12 345 67890", regex(r"\+\d{2}\s\d{3}\s\d{5}", "Phone number"));
/// assert_eq!(res.unwrap().val, "+12 345 67890");
///
/// let res = parse_str::<String, String>("+12 45 6890", regex(r"\+\d{2}\s\d{3}\s\d{5}", "Phone number"));
/// assert_eq!(
///     res.unwrap_err().get_error_message(),
///     "[Parser error] Expected `Phone number` at position: 0"
/// );
/// ```
pub fn regex<F: 'static + Display, A: AsRef<str>, B: AsRef<str>>(
    target: A,
    expected: B,
) -> Parser<String, F> {
    let target = target.as_ref().to_string();
    let expected = expected.as_ref().to_string();

    Box::new(move |mut ctx: Context| {
        let regex = match Regex::new(&target) {
            Ok(regex) => regex,
            Err(_) => panic!("Invalid regex: {}", target),
        };

        let sliced_ctx = ctx.txt.slice(ctx.pos..);
        let mat = regex.find(&sliced_ctx);
        if let Some(mat) = mat {
            if mat.start() == 0 {
                ctx.pos += mat.end();
                return Ok(Success::new(mat.as_str().to_string(), ctx));
            }
        }

        return Err(Failure::new(
            format!("{}", expected.clone()),
            ctx,
            Some(ParserType::Regex),
        ));
    })
}
