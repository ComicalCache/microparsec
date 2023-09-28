use crate::{Context, ContextParserT, Failure, ParserType, RegexParser, StringParserT, Success};

/// Parses for at least one letter
/// ## Example
/// ```
/// use parse_me::{LettersParser, StringParserT, ContextParserT};
///
/// let res = LettersParser::new().parse("Hello");
/// assert_eq!(res.unwrap().val, "Hello");
/// ```
#[derive(Clone)]
pub struct LettersParser {}

impl LettersParser {
    pub fn new() -> Self {
        LettersParser {}
    }
}

impl ContextParserT<String> for LettersParser {
    fn get_generic_error_message(&self) -> String {
        "letters".to_string()
    }

    fn get_parser_type(&self) -> ParserType {
        ParserType::Letters
    }

    fn parse_from_context(&self, ctx: Context) -> Result<Success<String>, Failure> {
        match RegexParser::new("[a-zA-Z]+", "letters").parse_from_context(ctx) {
            Ok(res) => Ok(res),
            Err(mut err) => {
                err.p_type_stack.push(ParserType::Letters);
                Err(err)
            }
        }
    }
}

impl StringParserT<String> for LettersParser {}
