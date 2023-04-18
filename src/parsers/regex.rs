use regex::Regex;

use crate::{string_utils::StringUtils, Context, Failure, Parser, ParserType, Success};

/// # Regex parser
/// Parses for a given regex pattern
/// ### Arguments
/// * `target` - The target regex pattern
/// * `expected` - A custom error message
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{regex, parse};
///
/// let res = parse("+12 345 67890", regex(r"\+\d{2}\s\d{3}\s\d{5}", "Phone number"));
/// assert_eq!(res.unwrap().val, "+12 345 67890");
///
/// let res = parse("+12 45 6890", regex(r"\+\d{2}\s\d{3}\s\d{5}", "Phone number"));
/// assert_eq!(
///     res.unwrap_err().get_error_message(),
///     "[Parser error] Expected `Phone number` at position: 0"
/// );
/// ```
pub fn regex<A: AsRef<str>, B: AsRef<str>>(target: A, expected: B) -> Parser<String> {
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
            vec![ParserType::Regex],
        ));
    })
}
