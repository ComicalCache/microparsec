use std::fmt::Display;

#[cfg(not(feature = "thread-safe"))]
use std::rc::Rc;
#[cfg(feature = "thread-safe")]
use std::sync::Arc;

#[cfg(not(feature = "thread-safe"))]
pub type ParserRc<T> = Rc<T>;

#[cfg(feature = "thread-safe")]
pub type ParserRc<T> = Arc<T>;

/// Trait for parsers that can take in a `Context` and act on it. <br>
/// Parsers are understood to be *pure with static state after initialization*. This is important because
/// `AnyParser`, `SequenceParser` and likewise parsers store parsers internally as
/// `ParserRc<dyn ContextParserT<T>>` and thus only make a shallow copy. Parsers that are *not* pure
/// might and likely will cause unexpected behaviour.
pub trait ContextParserT<T> {
    /// Returns a generic error message of the parser that is configured at initialization and
    /// independent of the runtime result of the attempted parse
    fn get_generic_error_message(&self) -> String;

    /// Returns the `ParserType` of the parser (used for stack traces and error messages)
    fn get_parser_type(&self) -> ParserType;

    /// Consumes a `Context` and attempts to parse it
    fn parse_from_context(&self, ctx: Context) -> Result<Success<T>, Failure>;
}

/// This is a sub-trait of `ContextParserT<T>`. It's only function is to abstract away the creation
/// of an initial `Context` for parsing and acts as "syntax sugar".
pub trait StringParserT<T>: ContextParserT<T> {
    /// Consumes a string type and attempts to parse it
    fn parse<S: AsRef<str>>(&self, txt: S) -> Result<Success<T>, Failure> {
        self.parse_from_context(Context::from(txt))
    }
}

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
/// * `p_type_stack` holds a call stack of parsers that lead up to the failure
#[derive(Debug, Clone)]
pub struct Failure {
    /// Error message
    pub exp: String,
    /// Context of the parse
    pub ctx: Context,

    /// Stack of parsers
    pub p_type_stack: Vec<ParserType>,
}

impl Failure {
    /// Creates a new `Failure` object with a short error message and context
    /// * `ctx` - the parse context
    /// * `exp` - a string of what was expected
    /// * `p_type_stack` - the parser stack that caused the failure
    pub fn new<S: AsRef<str>>(exp: S, ctx: Context, p_type_stack: Vec<ParserType>) -> Failure {
        let exp = exp.as_ref().to_string();
        Failure {
            exp,
            ctx,
            p_type_stack,
        }
    }

    /// Returns a human readable error message of the failure
    pub fn get_error_message(&self) -> String {
        format!(
            "[Parser error] Expected `{}` at position: {}",
            self.exp, self.ctx.pos,
        )
    }

    /// Returns a human readable error message of the failure with stack trace
    pub fn get_error_message_stack_trace(&self) -> String {
        let offset = self.p_type_stack.len();
        let call_stack = self
            .p_type_stack
            .iter()
            .enumerate()
            .map(|(i, e)| format!("{}. `{e}` parser", offset - i))
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "[Parser error] Expected `{}` at position: {}\n\nCall Stack:\n{call_stack}",
            self.exp, self.ctx.pos,
        )
    }
}

/// Enum used to determine the *relative* position to parse to in the `exact` parser
#[derive(Clone, Copy)]
pub enum Pos {
    /// Parse `x` characters
    Chars(usize),
    /// Parse until the end of input
    EOI,
}

/// The types of parsers
#[derive(Debug, Clone, PartialEq)]
pub enum ParserType {
    Any,
    AnythingBetween,
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
    Surely,

    /// Custom parsers type can be denoted with a custom type
    Custom(String),
}

impl Display for ParserType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ParserType::Any => "any",
            ParserType::AnythingBetween => "anything_between",
            ParserType::Between => "between",
            ParserType::Either => "either",
            ParserType::Exact => "exact",
            ParserType::Expect => "expect",
            ParserType::Float => "float",
            ParserType::Forget => "forget",
            ParserType::Integer => "integer",
            ParserType::Letters => "letters",
            ParserType::Many => "many",
            ParserType::Map => "map",
            ParserType::Optional => "optional",
            ParserType::Regex => "regex",
            ParserType::Sequence => "sequence",
            ParserType::Spaces => "spaces",
            ParserType::String => "string",
            ParserType::Surely => "surely",
            ParserType::Custom(parser) => parser.as_ref(),
        };

        write!(f, "{str}")
    }
}
