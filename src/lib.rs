#![allow(unused_macros)]
use regex::Regex;

mod string_utils;
use crate::string_utils::StringUtils;

/// Internal parser type
pub type Parser = Box<dyn Fn(Context) -> Result<Success, Failure>>;

/// Parser context
/// * `txt` - input string
/// * `pos` - current position in input string
#[derive(Debug, Clone)]
pub struct Context {
    /// Current input string
    pub txt: String,
    /// Current position in input string
    pub pos: usize,
}

impl Context {
    /// Creates a new Context
    /// * `txt` - The text of the context
    /// * `pos` - The position in the context
    pub fn new<S: AsRef<str>>(txt: S, pos: usize) -> Self {
        Context {
            txt: txt.as_ref().to_string(),
            pos,
        }
    }

    /// Creates a new Context from a text string
    /// * `txt` - The text of the context
    pub fn from<S: AsRef<str>>(txt: S) -> Self {
        Context {
            txt: txt.as_ref().to_string(),
            pos: 0,
        }
    }
}

/// `Success` is a successful parse result
/// * `val` holds the value of the parse
/// * `ctx` holds the context of the parse
#[derive(Debug, Clone)]
pub struct Success {
    /// Value of the parse
    pub val: Vec<String>,
    /// Context of the parse
    pub ctx: Context,
}

/// `Failure` is a failed parse result
/// * `exp` holds the error message
/// * `ctx` holds the context of the parse
#[derive(Debug, Clone)]
pub struct Failure {
    /// Error message
    pub exp: String,
    /// Context of the parse
    pub ctx: Context,
}

impl Failure {
    pub fn get_error_message(&self) -> String {
        format!(
            "[Parser error] Expected {} at position: '{}'",
            self.exp, self.ctx.pos,
        )
    }
}

/// Creates a new `Success` object with the given value and context
/// * `ctx` - the parse context
/// * `val` - the parsed value
pub fn success(val: Vec<String>, ctx: Context) -> Success {
    Success { val, ctx }
}

/// Creates a new `Failure` object with a short error message and context
/// * `ctx` - the parse context
/// * `exp` - a string of what was expected
pub fn failure<S: AsRef<str>>(exp: S, ctx: Context) -> Failure {
    let exp = exp.as_ref().to_string();
    Failure { exp, ctx }
}

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
/// assert_eq!(res.unwrap().val[0], "Hello World");
/// ```
pub fn string<S: AsRef<str>>(target: S) -> Parser {
    let target = target.as_ref().to_string();

    Box::new(move |mut ctx: Context| {
        if ctx.txt.slice(ctx.pos..).starts_with(&target) {
            ctx.pos += target.len();
            return Ok(success(vec![target.clone()], ctx));
        }

        return Err(failure(format!("'{}'", target.clone()), ctx));
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
/// assert_eq!(res.unwrap().val[0], "+12 345 67890");
///
/// let res = parse("+12 45 6890", regex(r"\+\d{2}\s\d{3}\s\d{5}", "Phone number"));
/// assert_eq!(
///     res.unwrap_err().get_error_message(),
///     "[Parser error] Expected 'Phone number' at position: '0'"
/// );
/// ```
pub fn regex<A: AsRef<str>, B: AsRef<str>>(target: A, expected: B) -> Parser {
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
                return Ok(success(vec![mat.as_str().to_string()], ctx));
            }
        }

        return Err(failure(format!("'{}'", expected.clone()), ctx));
    })
}

/// # Optional parser
/// Tries to parse the given parser, but if it fails, it returns a successful result with an empty value
/// ### Arguments
/// * `parser` - The parser to try to parse
/// ### Returns
/// * A parser that can be used in other parsers or directly ran in the `parse(...)` function
/// ## Example
/// ```
/// use parse_me::{optional, string, parse};
///
/// let res = parse("Hello World", optional(string("Hello World")));
/// assert_eq!(res.unwrap().val[0], "Hello World".to_string());
///
/// let res = parse("Hello World", optional(string("Hallo World")));
/// assert_eq!(res.unwrap().val.is_empty(), true);
/// ```
pub fn optional(parser: Parser) -> Parser {
    Box::new(move |ctx: Context| match parser(ctx.clone()) {
        Ok(res) => Ok(res),
        Err(_) => Ok(success(vec![], ctx)),
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
pub fn sequence(parsers: Vec<Parser>) -> Parser {
    Box::new(move |mut ctx: Context| {
        let mut result = Vec::new();
        for parser in parsers.iter() {
            match parser(ctx.clone()) {
                Ok(mut res) => {
                    ctx = res.ctx;
                    result.append(&mut res.val);
                }
                Err(err) => return Err(err),
            };
        }

        return Ok(success(result, ctx));
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
/// assert_eq!(res.unwrap().val, vec!["Hello".to_string()]);
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
/// assert_eq!(res.unwrap().val, vec!["Hello".to_string()]);
/// ```
pub fn any(parsers: Vec<Parser>) -> Parser {
    Box::new(move |ctx: Context| {
        let mut errs = Vec::new();

        for parser in parsers.iter() {
            match parser(ctx.clone()) {
                Ok(res) => return Ok(res),
                Err(err) => errs.push(err.exp),
            }
        }

        return Err(failure(format!("any of [{}]", errs.join(", ")), ctx));
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
/// assert_eq!(res.unwrap().val, vec!["Hello World".to_string()]);
pub fn either(parser_a: Parser, parser_b: Parser) -> Parser {
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
/// use parse_me::{map, sequence, string, parse};
///
/// let res = parse(
///     "Hello World",
///     map(
///         sequence!(string("Hello"), string(" "), string("World")),
///         |res| Ok(vec![res.val.join("")]),
///     ),
/// );
/// assert_eq!(res.unwrap().val, vec!["Hello World".to_string()]);
/// ```
pub fn map(parser: Parser, mapper: fn(Success) -> Result<Vec<String>, String>) -> Parser {
    Box::new(move |ctx: Context| {
        let res = match parser(ctx.clone()) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        let ctx = res.ctx.clone();
        match mapper(res) {
            Ok(mapped) => Ok(success(mapped, ctx)),
            Err(map_err) => Err(failure(map_err, ctx)),
        }
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
pub fn many(parser: Parser) -> Parser {
    Box::new(move |mut ctx: Context| {
        let mut ret = Vec::new();

        loop {
            match parser(ctx.clone()) {
                Ok(mut res) => {
                    ctx = res.ctx;
                    ret.append(&mut res.val);
                }
                Err(err) if ret.len() == 0 => return Err(err),
                Err(_) => return Ok(success(ret, ctx)),
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
/// assert_eq!(res.unwrap().val, vec!["Hello"]);
/// ```
pub fn between(front: Parser, middle: Parser, back: Parser) -> Parser {
    map(sequence!(front, middle, back), |v| {
        Ok(vec![v.val[1].clone()])
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
pub fn spaces() -> Parser {
    return map(many(string(" ")), |s| Ok(vec![s.val.join("")]));
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
/// assert_eq!(res.unwrap().val, vec!["Hello"]);
/// ```
pub fn letters() -> Parser {
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
/// assert_eq!(res.unwrap().val, vec!["123"]);
/// ```
pub fn integer() -> Parser {
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
/// assert_eq!(res.unwrap().val, vec!["123.456"]);
/// ```
pub fn float() -> Parser {
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
/// assert_eq!(res.unwrap_err().get_error_message(), "[Parser error] Expected '\"Hello World\"' at position: '0'");
/// ```
pub fn expect<S: AsRef<str>>(parser: Parser, expected: S) -> Parser {
    let expected = expected.as_ref().to_string();

    Box::new(move |ctx: Context| match parser(ctx.clone()) {
        Ok(res) => Ok(res),
        Err(err) => Err(failure(format!("'{expected}'"), err.ctx)),
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
pub fn parse_from_context(ctx: Context, parser: Parser) -> Result<Success, Failure> {
    match parser(ctx) {
        Ok(res) => Ok(res),
        Err(err) => Err(failure(err.exp, err.ctx)),
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
pub fn parse<S: AsRef<str>>(txt: S, parser: Parser) -> Result<Success, Failure> {
    parse_from_context(Context::from(txt), parser)
}
