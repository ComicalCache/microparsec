use crate::{Context, ContextParserT, Failure, ParserType, RegexParser, StringParserT, Success};

/// Parses for a float
/// ## Example
/// ```
/// use parse_me::{FloatParser, ContextParserT, StringParserT};
///
/// let res = FloatParser::new().parse("123.456");
/// assert_eq!(res.unwrap().val, "123.456");
/// ```
#[derive(Clone)]
pub struct FloatParser {}

impl FloatParser {
    pub fn new() -> Self {
        FloatParser {}
    }
}

impl ContextParserT<String> for FloatParser {
    fn get_generic_error_message(&self) -> String {
        "float".to_string()
    }

    fn get_parser_type(&self) -> ParserType {
        ParserType::Float
    }

    fn parse_from_context(&self, ctx: Context) -> Result<Success<String>, Failure> {
        match RegexParser::new(r"\d+\.\d*", "float").parse_from_context(ctx) {
            Ok(res) => Ok(res),
            Err(mut err) => {
                err.p_type_stack.push(ParserType::Float);
                Err(err)
            }
        }
    }
}

impl StringParserT<String> for FloatParser {}
