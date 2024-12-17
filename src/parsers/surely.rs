use crate::{Context, ContextParserT, Failure, ParserRc, ParserType, StringParserT, Success};

/// If an any parser comes across a surely parser and it fails the any parser immediately fails as well
/// ## Example
/// ```
/// use microparsec::{SequenceParser, SurelyParser, AnyParser, StringParser, ParserRc, ContextParserT, StringParserT, parsers};
///
/// // let res = parse("Hello Welt", any!(sequence!(string("Hello"), spaces(), surely(string("World"))), sequence!(string("Hallo"), spaces(), surely(string("Welt")))));
/// let hello_parser = StringParser::new("Hello");
/// let hallo_parser = StringParser::new("Hallo");
/// let world_parser = SurelyParser::new(ParserRc::new(StringParser::new("World")));
/// let space_parser = StringParser::new(" ");
/// let hello_world_parser = SequenceParser::new(parsers!(hello_parser, space_parser.clone(), world_parser.clone()));
/// let hallo_world_parser = SequenceParser::new(parsers!(hallo_parser, space_parser, world_parser));
///
/// let res = AnyParser::new(parsers!(hello_world_parser, hallo_world_parser)).parse("Hallo Welt");
/// assert_eq!(res.unwrap_err().get_error_message(), "[Parser error] Expected `surely `World`` at position: 6");
/// ```
#[derive(Clone)]
pub struct SurelyParser<T> {
    parser: ParserRc<dyn ContextParserT<T>>,
    generic_error: String,
}

impl<T> SurelyParser<T> {
    pub fn new(parser: ParserRc<dyn ContextParserT<T>>) -> Self {
        let generic_error = format!("surely `{}`", parser.get_generic_error_message());

        SurelyParser {
            parser,
            generic_error,
        }
    }
}

impl<T> ContextParserT<T> for SurelyParser<T> {
    fn get_generic_error_message(&self) -> String {
        self.generic_error.clone()
    }

    fn get_parser_type(&self) -> ParserType {
        ParserType::Surely
    }

    fn parse_from_context(&self, ctx: Context) -> Result<Success<T>, Failure> {
        match self.parser.parse_from_context(ctx) {
            Ok(res) => Ok(res),
            Err(mut err) => {
                err.exp = self.generic_error.clone();
                err.p_type_stack.push(ParserType::Surely);
                Err(err)
            }
        }
    }
}

impl<T> StringParserT<T> for SurelyParser<T> {}
