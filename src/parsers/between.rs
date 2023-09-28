use crate::{ParserRc, Context, ContextParserT, Failure, ParserType, StringParserT, Success};

/// # Between parser
/// Parses between two parsers, both the front and the back parser must succeed as well as the
/// middle parser, only then it is considered a successful parse.
/// ## Example
/// ```
/// use parse_me::{ParserRc, BetweenParser, StringParser, ContextParserT, StringParserT};
///
/// let quote_parser = StringParser::new("\"");
/// let hello_parser = ParserRc::new(StringParser::new("Hello"));
/// let res = BetweenParser::new(ParserRc::new(quote_parser.clone()), hello_parser, ParserRc::new(quote_parser))
///             .parse("\"Hello\"");
/// assert_eq!(res.unwrap().val, "Hello");
/// ```
#[derive(Clone)]
pub struct BetweenParser<N, T, M> {
    front_parser: ParserRc<dyn ContextParserT<N>>,
    middle_parser: ParserRc<dyn ContextParserT<T>>,
    back_parser: ParserRc<dyn ContextParserT<M>>,
    generic_error: String,
}

impl<N, T, M> BetweenParser<N, T, M> {
    pub fn new(front_parser: ParserRc<dyn ContextParserT<N>>,
               middle_parser: ParserRc<dyn ContextParserT<T>>,
               back_parser: ParserRc<dyn ContextParserT<M>>) -> Self {
        let generic_error = format!("{} + {} + {}",
                                    front_parser.get_generic_error_message(),
                                    middle_parser.get_generic_error_message(),
                                    back_parser.get_generic_error_message()
        );

        BetweenParser {
            front_parser,
            middle_parser,
            back_parser,
            generic_error,
        }
    }
}

impl<N, T, M> ContextParserT<T> for BetweenParser<N, T, M> {
    fn get_generic_error_message(&self) -> String {
        self.generic_error.clone()
    }

    fn get_parser_type(&self) -> ParserType {
        ParserType::Between
    }

    fn parse_from_context(&self, ctx: Context) -> Result<Success<T>, Failure> {
        let ctx = match self.front_parser.parse_from_context(ctx) {
            Ok(res) => res.ctx,
            Err(mut err) => {
                err.p_type_stack.push(ParserType::Between);
                return Err(err);
            }
        };

        let res = match self.middle_parser.parse_from_context(ctx) {
            Ok(res) => res,
            Err(mut err) => {
                err.p_type_stack.push(ParserType::Between);
                return Err(err);
            }
        };

        let ctx = match self.back_parser.parse_from_context(res.ctx) {
            Ok(res) => res.ctx,
            Err(mut err) => {
                err.p_type_stack.push(ParserType::Between);
                return Err(err);
            }
        };

        Ok(Success::new(res.val, ctx))
    }
}

impl<N, T, M> StringParserT<T> for BetweenParser<N, T, M> {}
