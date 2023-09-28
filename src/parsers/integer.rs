use crate::{Context, ContextParserT, Failure, ParserType, RegexParser, StringParserT, Success};

/// Parses for an integer
/// ## Example
/// ```
/// use parse_me::{IntegerParser, ContextParserT, StringParserT};
///
/// let res = IntegerParser::new().parse("123");
/// assert_eq!(res.unwrap().val, "123");
/// ```
#[derive(Clone)]
pub struct IntegerParser {}

impl IntegerParser {
    pub fn new() -> Self {
        IntegerParser {}
    }
}

impl ContextParserT<String> for IntegerParser {
    fn get_generic_error_message(&self) -> String {
        "integer".to_string()
    }

    fn get_parser_type(&self) -> ParserType {
        ParserType::Integer
    }

    fn parse_from_context(&self, ctx: Context) -> Result<Success<String>, Failure> {
        match RegexParser::new(r"\d+", "integer").parse_from_context(ctx) {
            Ok(res) => Ok(res),
            Err(mut err) => {
                err.p_type_stack.push(ParserType::Integer);
                Err(err)
            }
        }
    }
}

impl StringParserT<String> for IntegerParser {}
