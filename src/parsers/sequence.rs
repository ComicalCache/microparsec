use crate::{Context, ContextParserT, Failure, ParserRc, ParserType, StringParserT, Success};

/// Parses for a sequence of parsers
/// ## Example
/// ```
/// use microparsec::{ParserRc, SpacesParser, StringParser, SequenceParser, StringParserT, ContextParserT, parsers};
///
/// let hello_parser = StringParser::new("Hello");
/// let spaces_parser = SpacesParser::new();
/// let world_parser = StringParser::new("World");
/// let res = SequenceParser::new(parsers!(hello_parser, spaces_parser, world_parser)).parse("Hello  World");
///
/// assert_eq!(
///     res.unwrap().val,
///     vec!["Hello".to_string(), "  ".to_string(), "World".to_string()]
/// );
/// ```
#[derive(Clone)]
pub struct SequenceParser<T> {
    parsers: Vec<ParserRc<dyn ContextParserT<T>>>,
    generic_error: String,
}

impl<T> SequenceParser<T> {
    pub fn new(parsers: Vec<ParserRc<dyn ContextParserT<T>>>) -> Self {
        let generic_error = parsers
            .iter()
            .map(|p| p.get_generic_error_message())
            .collect::<Vec<String>>()
            .join(" -> ");

        SequenceParser {
            parsers,
            generic_error,
        }
    }
}

impl<T> ContextParserT<Vec<T>> for SequenceParser<T> {
    fn get_generic_error_message(&self) -> String {
        self.generic_error.clone()
    }

    fn get_parser_type(&self) -> ParserType {
        ParserType::Sequence
    }

    fn parse_from_context(&self, mut ctx: Context) -> Result<Success<Vec<T>>, Failure> {
        let mut result = Vec::new();
        for parser in self.parsers.iter() {
            match parser.parse_from_context(ctx.clone()) {
                Ok(res) => {
                    ctx = res.ctx;
                    result.push(res.val);
                }
                Err(mut err) => {
                    err.p_type_stack.push(ParserType::Sequence);
                    return Err(err);
                }
            };
        }

        Ok(Success::new(result, ctx))
    }
}

impl<T> StringParserT<Vec<T>> for SequenceParser<T> {}
