use crate::{Context, ContextParserT, Failure, ParserRc, ParserType, StringParserT, Success};

/// Parses as many times as possible, returns an error if no parsing was successful
/// ## Example
/// ```
/// use microparsec::{ManyParser, ContextParserT, StringParserT, ParserRc, RegexParser};
///
/// let digit_parser = RegexParser::new(r"\d", "digit");
/// let digits_parser = ManyParser::new(ParserRc::new(digit_parser));
///
/// let res = digits_parser.parse("123abc");
/// assert_eq!(
///     res.unwrap().val,
///     vec!["1".to_string(), "2".to_string(), "3".to_string()]
/// );
/// ```
#[derive(Clone)]
pub struct ManyParser<T> {
    parser: ParserRc<dyn ContextParserT<T>>,
    generic_error: String,
}

impl<T> ManyParser<T> {
    pub fn new(parser: ParserRc<dyn ContextParserT<T>>) -> Self {
        let generic_error = format!("many `{}`", parser.get_generic_error_message());

        ManyParser {
            parser,
            generic_error,
        }
    }
}

impl<T> ContextParserT<Vec<T>> for ManyParser<T> {
    fn get_generic_error_message(&self) -> String {
        self.generic_error.clone()
    }

    fn get_parser_type(&self) -> ParserType {
        ParserType::Many
    }

    fn parse_from_context(&self, mut ctx: Context) -> Result<Success<Vec<T>>, Failure> {
        let mut ret = Vec::new();

        loop {
            match self.parser.parse_from_context(ctx.clone()) {
                Ok(res) => {
                    ctx = res.ctx;
                    ret.push(res.val);
                }
                Err(mut err) if ret.is_empty() => {
                    err.p_type_stack.push(ParserType::Many);
                    return Err(err);
                }
                Err(_) => return Ok(Success::new(ret, ctx)),
            };
        }
    }
}

impl<T> StringParserT<Vec<T>> for ManyParser<T> {}
