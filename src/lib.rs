#![allow(unused_macros)]
use regex::Regex;

mod string_utils;
use crate::string_utils::StringUtils;

mod types;
pub use types::*;

/// # String parser
/// Parses for a given target string
/// ### Arguments
/// * `target` - The target string to parse for
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{string, parse};
///
/// let res = parse("Hello World", string("Hello World"));
/// assert_eq!(res.unwrap().val, "Hello World");
/// ```
pub fn string<S: AsRef<str>>(target: S) -> Parser<String> {
    let target = target.as_ref().to_string();

    Box::new(move |mut ctx: Context| {
        if ctx.txt.slice(ctx.pos..).starts_with(&target) {
            ctx.pos += target.len();
            return Ok(Success::new(target.clone(), ctx));
        }

        return Err(Failure::new(format!("{}", target.clone()), ctx));
    })
}

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

        return Err(Failure::new(format!("{}", expected.clone()), ctx));
    })
}

/// # Optional parser
/// Tries to parse the given parser, but if it fails, it returns a successful result with a None value
/// ### Arguments
/// * `parser` - The parser to try to parse
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{optional, string, parse};
///
/// let res = parse("Hello World", optional(string("Hello World")));
/// assert_eq!(res.unwrap().val.unwrap(), "Hello World");
///
/// let res = parse("Hello World", optional(string("Hallo World")));
/// assert_eq!(res.unwrap().val.is_none(), true);
/// ```
pub fn optional<T: 'static>(parser: Parser<T>) -> Parser<Option<T>> {
    Box::new(move |ctx: Context| match parser(ctx.clone()) {
        Ok(res) => Ok(Success::new(Some(res.val), res.ctx)),
        Err(_) => Ok(Success::new(None, ctx)),
    })
}

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
                Err(err) => return Err(err),
            };
        }

        return Ok(Success::new(result, ctx));
    })
}

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
        let mut errs = Vec::new();

        for parser in parsers.iter() {
            match parser(ctx.clone()) {
                Ok(res) => return Ok(res),
                Err(err) => errs.push(err.exp),
            }
        }

        return Err(Failure::new(format!("{{ `{}` }}", errs.join("` | `")), ctx));
    })
}

/// # Either parser
/// Parses either of the two given parsers and returns either the first to match or an error if both failed
/// ### Arguments
/// * `parser_a` - The first parser to parse for
/// * `parser_b` - The second parser to parse for
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{either, string, parse};
///
/// let res = parse("Hello World", either(string("Hallo Welt"), string("Hello World")));
/// assert_eq!(res.unwrap().val, "Hello World");
/// ```
pub fn either<T: 'static>(parser_a: Parser<T>, parser_b: Parser<T>) -> Parser<T> {
    any!(parser_a, parser_b)
}

/// # Map parser
/// Maps the result of a parser to a new value
/// ### Arguments
/// * `parser` - The parser to map
/// * `mapper` - The function to map the result of the parser
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// #[macro_use] extern crate parse_me;
/// use parse_me::{map, sequence, string, integer, parse};
///
/// let res = parse(
///     "Hello World",
///     map(
///         sequence!(string("Hello"), string(" "), string("World")),
///         |res| Ok(res.val.join("")),
///     ),
/// );
/// assert_eq!(res.unwrap().val, "Hello World");
///
/// let res = parse("234", map(integer(), |res| Ok(res.val.parse::<usize>().unwrap())));
/// assert_eq!(res.unwrap().val, 234);
/// ```
pub fn map<T: 'static, M: 'static>(
    parser: Parser<T>,
    mapper: fn(Success<T>) -> Result<M, String>,
) -> Parser<M> {
    Box::new(move |ctx: Context| {
        let res = match parser(ctx.clone()) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        let ctx = res.ctx.clone();
        match mapper(res) {
            Ok(mapped) => Ok(Success::new(mapped, ctx)),
            Err(map_err) => Err(Failure::new(map_err, ctx)),
        }
    })
}

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
        Err(err) => Err(err),
    })
}

/// # Many parser
/// Parses as many times as possible, returns an error if no parsing was successful
/// ### Arguments
/// * `parser` - The parser to parse for
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// * Look at the `spaces()` parser implementation for an example
pub fn many<T: 'static>(parser: Parser<T>) -> Parser<Vec<T>> {
    Box::new(move |mut ctx: Context| {
        let mut ret = Vec::new();

        loop {
            match parser(ctx.clone()) {
                Ok(res) => {
                    ctx = res.ctx;
                    ret.push(res.val);
                }
                Err(err) if ret.len() == 0 => return Err(err),
                Err(_) => return Ok(Success::new(ret, ctx)),
            };
        }
    })
}

/// # Between parser
/// Parses between two parsers
/// ### Arguments
/// * `front` - The left parser
/// * `middle` - The parser to parse between the left and right parser
/// * `back` - The right parser
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{between, string, parse};
///
/// let res = parse(
///     "\"Hello\"",
///     between(string("\""), string("Hello"), string("\"")),
/// );
/// assert_eq!(res.unwrap().val, "Hello");
/// ```
pub fn between<N: 'static, T: 'static, M: 'static>(
    front: Parser<N>,
    middle: Parser<T>,
    back: Parser<M>,
) -> Parser<T> {
    Box::new(move |ctx| {
        let ctx = match front(ctx) {
            Ok(res) => res.ctx,
            Err(err) => return Err(err),
        };

        let res = match middle(ctx) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        let ctx = match back(res.ctx) {
            Ok(res) => res.ctx,
            Err(err) => return Err(err),
        };

        Ok(Success::new(res.val, ctx))
    })
}

/// # Exact parser
/// Attemts to parse a specified number of chars or to the EOI and fails otherwise
/// ### Arguments
/// * `parser` - The parser to parse with
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{exact, string, parse, Pos};
///
/// let res = parse("Hello World", exact(string("Hello World"), Pos::EOI));
/// assert_eq!(res.unwrap().val, "Hello World");
/// ```
pub fn exact<T: 'static>(parser: Parser<T>, pos: Pos) -> Parser<T> {
    Box::new(move |ctx: Context| {
        let prev_pos = ctx.pos;
        let mut res = match parser(ctx) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        match pos {
            Pos::Chars(x) => {
                if res.ctx.pos - prev_pos == x {
                    Ok(res)
                } else {
                    res.ctx.pos = prev_pos;
                    Err(Failure::new(format!("parsing {x} characters"), res.ctx))
                }
            }
            Pos::EOI => {
                if res.ctx.pos == res.ctx.txt.len() {
                    Ok(res)
                } else {
                    res.ctx.pos = prev_pos;
                    Err(Failure::new("parsing to EOI", res.ctx))
                }
            }
        }
    })
}

/// # Spaces parser
/// Parses for at least one and as many spaces as possible
/// # Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// #[macro_use] extern crate parse_me;
/// use parse_me::{spaces, string, parse, sequence};
///
/// let res = parse(
///     "Hello World",
///     sequence!(string("Hello"), spaces(), string("World")),
/// );
/// assert_eq!(
///     res.unwrap().val,
///     vec!["Hello".to_string(), " ".to_string(), "World".to_string()]
/// );
/// ```
pub fn spaces() -> Parser<String> {
    return map(many(string(" ")), |s| Ok(s.val.join("")));
}

/// # Letters parser
/// Parses for at least one letter
/// # Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{letters, parse};
///
/// let res = parse("Hello", letters());
/// assert_eq!(res.unwrap().val, "Hello");
/// ```
pub fn letters() -> Parser<String> {
    return regex("[a-zA-Z]+", "letters");
}

/// # Integer parser
/// Parses for an integer
/// # Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{integer, parse};
///
/// let res = parse("123", integer());
/// assert_eq!(res.unwrap().val, "123");
/// ```
pub fn integer() -> Parser<String> {
    return regex(r"\d+", "integer");
}

/// # Float parser
/// Parses for a float
/// # Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{float, parse};
///
/// let res = parse("123.456", float());
/// assert_eq!(res.unwrap().val, "123.456");
/// ```
pub fn float() -> Parser<String> {
    return regex(r"\d+\.\d*", "float");
}

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
        Err(err) => Err(Failure::new(format!("{expected}"), err.ctx)),
    })
}

/// Runs a given parser on a given context.
/// ### Arguments
/// * `ctx` - The context to parse
/// * `parser` - The parser to run
/// ### Returns
/// * `Result<Success, String>` containing the result of the parser or the error message
/// ## Example
/// ```
/// #[macro_use] extern crate parse_me;
/// use parse_me::{map, parse_from_context, string, spaces, sequence, Context};
///
/// let res = parse_from_context(Context::from("Hello World"),
///     map(sequence!(string("Hello"), spaces(), string("World")),
///         |r| Ok(vec![r.val.join("")]),
///     ),
/// );
///
/// assert_eq!(
///     res.unwrap().val,
///     vec!["Hello World".to_string()]
/// );
/// ```
pub fn parse_from_context<T>(ctx: Context, parser: Parser<T>) -> Result<Success<T>, Failure> {
    match parser(ctx) {
        Ok(res) => Ok(res),
        Err(err) => Err(Failure::new(err.exp, err.ctx)),
    }
}

/// Runs a given parser on a given string.
/// ### Arguments
/// * `txt` - The text to parse
/// * `parser` - The parser to run
/// ### Returns
/// * `Result<Success, String>` containing the result of the parser or the error message
/// ## Example
/// ```
/// #[macro_use] extern crate parse_me;
/// use parse_me::{map, parse, string, spaces, sequence};
///
/// let res = parse("Hello World",
///     map(sequence!(string("Hello"), spaces(), string("World")),
///         |r| Ok(vec![r.val.join("")]),
///     ),
/// );
///
/// assert_eq!(
///     res.unwrap().val,
///     vec!["Hello World".to_string()]
/// );
/// ```
pub fn parse<T, S: AsRef<str>>(txt: S, parser: Parser<T>) -> Result<Success<T>, Failure> {
    parse_from_context(Context::from(txt), parser)
}
