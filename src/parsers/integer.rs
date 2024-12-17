use crate::{Context, ContextParserT, Failure, ParserType, StringParserT, Success};

/// Parses for an integer
/// ## Example
/// ```
/// use microparsec::{IntegerParser, ContextParserT, StringParserT};
///
/// let res = IntegerParser::new().parse("123");
/// assert_eq!(res.unwrap().val, "123");
/// ```
#[derive(Clone)]
pub struct IntegerParser {}

impl IntegerParser {
    #[allow(clippy::new_without_default)]
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

    fn parse_from_context(&self, mut ctx: Context) -> Result<Success<String>, Failure> {
        let integers: String = ctx.txt[ctx.pos..]
            .chars()
            .take_while(|c| c.is_numeric())
            .collect();

        if integers.is_empty() {
            return Err(Failure::new("integer", ctx, vec![ParserType::Integer]));
        }

        ctx.pos += integers.len();
        Ok(Success::new(integers, ctx))
    }
}

impl StringParserT<String> for IntegerParser {}
