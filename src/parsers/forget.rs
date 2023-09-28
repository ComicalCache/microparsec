use crate::{Context, ContextParserT, Failure, ParserRc, ParserType, StringParserT, Success};

/// "Forgets" the success value type and changes it to `()`
/// ## Example
/// ```
/// use parse_me::{StringParser, ForgetParser, ParserRc, ContextParserT, StringParserT};
///
/// let res = ForgetParser::new(ParserRc::new(StringParser::new("Hello"))).parse("Hello");
/// assert_eq!(res.unwrap().val, ());
/// ```
#[derive(Clone)]
pub struct ForgetParser<T> {
    parser: ParserRc<dyn ContextParserT<T>>,
    generic_error: String,
}

impl<T> ForgetParser<T> {
    pub fn new(parser: ParserRc<dyn ContextParserT<T>>) -> Self {
        let generic_error = format!("forgetting `{}`", parser.get_generic_error_message());

        ForgetParser {
            parser,
            generic_error,
        }
    }
}

impl<T> ContextParserT<()> for ForgetParser<T> {
    fn get_generic_error_message(&self) -> String {
        self.generic_error.clone()
    }

    fn get_parser_type(&self) -> ParserType {
        ParserType::Forget
    }

    fn parse_from_context(&self, ctx: Context) -> Result<Success<()>, Failure> {
        match self.parser.parse_from_context(ctx) {
            Ok(res) => Ok(Success::new((), res.ctx)),
            Err(mut err) => {
                err.p_type_stack.push(ParserType::Forget);
                Err(err)
            }
        }
    }
}

impl<T> StringParserT<()> for ForgetParser<T> {}
