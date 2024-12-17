use std::ops::Not;

use crate::{Context, ContextParserT, Failure, ParserType, StringParserT, Success};

/// Matches until a character from the target is seen
/// ### Example
/// ```
/// use microparsec::{NotParser, ContextParserT, StringParserT};
///
/// let res = NotParser::new(" ").parse("Hello, World!");
/// assert_eq!(res.unwrap().val, "Hello,");
/// ```
#[derive(Clone)]
pub struct NotParser {
    target: String,
}

impl NotParser {
    /// Creates a new `NotParser` with the specified target string
    pub fn new<S: AsRef<str>>(target: S) -> Self {
        NotParser {
            target: target.as_ref().to_string(),
        }
    }
}

impl ContextParserT<String> for NotParser {
    fn get_generic_error_message(&self) -> String {
        self.target.to_string()
    }

    fn get_parser_type(&self) -> ParserType {
        ParserType::String
    }

    fn parse_from_context(&self, mut ctx: Context) -> Result<Success<String>, Failure> {
        let word: String = ctx.txt[ctx.pos..]
            .chars()
            .take_while(|c| self.target.contains(*c).not())
            .collect();

        ctx.pos += word.len();

        Ok(Success::new(word, ctx))
    }
}

impl StringParserT<String> for NotParser {}
