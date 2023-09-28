use crate::{Context, ContextParserT, Failure, ParserRc, ParserType, StringParserT, Success};

/// Tries to parse the given parser, but if it fails, it returns a successful result with a None value
/// ## Example
/// ```
/// use parse_me::{StringParser, OptionalParser, ContextParserT, StringParserT, ParserRc};
///
/// let hello_world_parser = StringParser::new("Hello World");
/// let res = OptionalParser::new(ParserRc::new(hello_world_parser.clone())).parse("Hello World");
/// assert_eq!(res.unwrap().val.unwrap(), "Hello World");
///
/// let res = OptionalParser::new(ParserRc::new(hello_world_parser)).parse("Hallo World");
/// assert_eq!(res.unwrap().val.is_none(), true);
/// ```
#[derive(Clone)]
pub struct OptionalParser<T> {
    parser: ParserRc<dyn ContextParserT<T>>,
    generic_error: String,
}

impl<T> OptionalParser<T> {
    pub fn new(parser: ParserRc<dyn ContextParserT<T>>) -> Self {
        let generic_error = format!("optional `{}`", parser.get_generic_error_message());

        OptionalParser {
            parser,
            generic_error,
        }
    }
}

impl<T> ContextParserT<Option<T>> for OptionalParser<T> {
    fn get_generic_error_message(&self) -> String {
        self.generic_error.clone()
    }

    fn get_parser_type(&self) -> ParserType {
        ParserType::Optional
    }

    fn parse_from_context(&self, ctx: Context) -> Result<Success<Option<T>>, Failure> {
        match self.parser.parse_from_context(ctx.clone()) {
            Ok(res) => Ok(Success::new(Some(res.val), res.ctx)),
            Err(_) => Ok(Success::new(None, ctx)),
        }
    }
}

impl<T> StringParserT<Option<T>> for OptionalParser<T> {}
