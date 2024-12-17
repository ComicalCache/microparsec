use crate::{Context, ContextParserT, Failure, ParserRc, ParserType, StringParserT, Success};

/// Runs a supplied parser, if fails, returns a custom error message
/// ## Example
/// ```
/// use microparsec::{ParserRc, ExpectParser, StringParser, ContextParserT, StringParserT};
///
/// let hello_world_parser = StringParser::new("Hello World");
/// let res = ExpectParser::new(ParserRc::new(hello_world_parser), "\"Hello World\"").parse("Hallo Welt");
/// assert_eq!(res.unwrap_err().get_error_message(), "[Parser error] Expected `\"Hello World\"` at position: 0");
/// ```
#[derive(Clone)]
pub struct ExpectParser<T> {
    parser: ParserRc<dyn ContextParserT<T>>,
    generic_error: String,
}

impl<T> ExpectParser<T> {
    pub fn new<S: AsRef<str>>(parser: ParserRc<dyn ContextParserT<T>>, expected: S) -> Self {
        let generic_error = expected.as_ref().to_string();

        ExpectParser {
            parser,
            generic_error,
        }
    }
}

impl<T> ContextParserT<T> for ExpectParser<T> {
    fn get_generic_error_message(&self) -> String {
        self.generic_error.clone()
    }

    fn get_parser_type(&self) -> ParserType {
        ParserType::Expect
    }

    fn parse_from_context(&self, ctx: Context) -> Result<Success<T>, Failure> {
        match self.parser.parse_from_context(ctx.clone()) {
            Ok(res) => Ok(res),
            Err(mut err) => {
                err.exp = self.generic_error.clone();
                err.p_type_stack.push(ParserType::Expect);
                Err(err)
            }
        }
    }
}

impl<T> StringParserT<T> for ExpectParser<T> {}
