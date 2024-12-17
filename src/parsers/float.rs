use crate::{Context, ContextParserT, Failure, ParserType, RegexParser, StringParserT, Success};

/// Parses for a float
/// ## Example
/// ```
/// use microparsec::{FloatParser, ContextParserT, StringParserT};
///
/// let res = FloatParser::new().parse("123.456");
/// assert_eq!(res.unwrap().val, "123.456");
/// ```
#[derive(Clone)]
pub struct FloatParser {}

impl FloatParser {
    #[allow(clippy::new_without_default)]
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
        match RegexParser::new(r"\d+\.\d*", "float").parse_from_context(ctx.clone()) {
            Ok(res) => Ok(res),
            Err(err) => Err(Failure::new("float", err.ctx, vec![ParserType::Float])),
        }
    }
}

impl StringParserT<String> for FloatParser {}
