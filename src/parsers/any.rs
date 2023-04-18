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
/// use parse_me::{any, string, parse};
///
/// let res = parse("Hello World", any!(string("Hallo"), string("Hello")));
/// assert_eq!(res.unwrap().val, "Hello");
/// ```
#[macro_export]
macro_rules! any {
    ($p:ident) => {
        any($p)
    };
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
/// use parse_me::{any, string, parse};
///
/// let res = parse("Hello World", any(vec![string("Hallo"), string("Hello")]));
/// assert_eq!(res.unwrap().val, "Hello");
/// ```
pub fn any<T: 'static>(parsers: Vec<Parser<T>>) -> Parser<T> {
    Box::new(move |ctx: Context| {
        let mut err_exps = Vec::new();
        let mut err_p_types = Vec::new();

        for parser in parsers.iter() {
            match parser(ctx.clone()) {
                Ok(res) => return Ok(res),
                Err(mut err) => {
                    if err.p_type_stack.contains(&ParserType::Surely) {
                        err.p_type_stack.push(ParserType::Any);
                        return Err(err);
                    } else {
                        err_exps.push(err.exp);
                        err_p_types.push(err.p_type_stack.last().unwrap().clone());
                    }
                }
            }
        }

        err_p_types.reverse();
        err_p_types.push(ParserType::Any);
        return Err(Failure::new(
            format!("{{ `{}` }}", err_exps.join("` | `")),
            ctx,
            err_p_types,
        ));
    })
}
