use crate::{Context, ContextParserT, Failure, ParserType, StringParserT, Success};

/// Parses for a specific target string
/// ### Example
/// ```
/// use parse_me::{StringParser, ContextParserT, StringParserT};
///
/// let res = StringParser::new("Hello World").parse("Hello World");
/// assert_eq!(res.unwrap().val, "Hello World");
/// ```
#[derive(Clone)]
pub struct StringParser {
    target: String,
}

impl StringParser {
    /// Creates a new `StringParser` with the specified target string
    pub fn new<S: AsRef<str>>(target: S) -> Self {
        StringParser {
            target: target.as_ref().to_string(),
        }
    }
}

impl ContextParserT<String> for StringParser {
    fn get_generic_error_message(&self) -> String {
        self.target.to_string()
    }

    fn get_parser_type(&self) -> ParserType {
        ParserType::String
    }

    fn parse_from_context(&self, mut ctx: Context) -> Result<Success<String>, Failure> {
        if ctx.txt[ctx.pos..].starts_with(&self.target) {
            ctx.pos += self.target.len();
            return Ok(Success::new(self.target.clone(), ctx));
        }

        return Err(Failure::new(
            self.target.clone(),
            ctx,
            vec![ParserType::String],
        ));
    }
}

impl StringParserT<String> for StringParser {}
