use std::fmt::Display;

/// Internal parser type
pub type Parser<T, F> = Box<dyn Fn(Context) -> Result<Success<T>, Failure<F>>>;

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
pub struct Success<T> {
    /// Value of the parse
    pub val: T,
    /// Context of the parse
    pub ctx: Context,
}

impl<T> Success<T> {
    /// Creates a new `Success` object with the given value and context
    /// * `ctx` - the parse context
    /// * `val` - the parsed value
    pub fn new(val: T, ctx: Context) -> Success<T> {
        Success { val, ctx }
    }
}

/// `Failure` is a failed parse result
/// * `exp` holds the error message
/// * `ctx` holds the context of the parse
#[derive(Debug, Clone)]
pub struct Failure<T: Display> {
    /// Error message
    pub exp: String,
    /// Context of the parse
    pub ctx: Context,

    /// Parser type that caused the failure
    pub p_type: Option<ParserType<T>>,
}

impl<T: Display> Failure<T> {
    /// Creates a new `Failure` object with a short error message and context
    /// * `ctx` - the parse context
    /// * `exp` - a string of what was expected
    /// * `p_type` - the type of parser that caused the failure
    pub fn new<S: AsRef<str>>(exp: S, ctx: Context, p_type: Option<ParserType<T>>) -> Failure<T> {
        let exp = exp.as_ref().to_string();
        Failure { exp, ctx, p_type }
    }

    /// Returns a human readable error message of the failure
    pub fn get_error_message(&self) -> String {
        format!(
            "[Parser error] Expected `{}` at position: {}",
            self.exp, self.ctx.pos,
        )
    }
}

impl Failure<String> {
    /// Creates a new `Failure` object with a short error message and context
    /// * `ctx` - the parse context
    /// * `exp` - a string of what was expected
    pub fn from<S: AsRef<str>>(exp: S, ctx: Context) -> Failure<String> {
        let exp = exp.as_ref().to_string();
        let p_type: Option<ParserType<String>> = None;
        Failure { exp, ctx, p_type }
    }
}

/// Enum used to determine the *relative* position to parse to in the `exact` parser
pub enum Pos {
    /// Parse `x` characters
    Chars(usize),
    /// Parse until the end of input
    EOI,
}

/// The types of parsers
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParserType<T: Display> {
    Any,
    Between,
    Either,
    Exact,
    Expect,
    Float,
    Forget,
    Integer,
    Letters,
    Many,
    Map,
    Optional,
    Regex,
    Sequence,
    Spaces,
    String,

    /// Custom parsers type can be denoted with a custom type
    Custom(T),
}
