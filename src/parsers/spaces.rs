use crate::{Context, ContextParserT, Failure, ParserType, StringParserT, Success};

/// Parses for at least one and as many spaces as possible
/// ## Example
/// ```
/// use parse_me::{ParserRc, SpacesParser, StringParser, SequenceParser, StringParserT, ContextParserT, parsers};
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
pub struct SpacesParser {}

impl SpacesParser {
    pub fn new() -> Self {
        SpacesParser {}
    }
}

impl ContextParserT<String> for SpacesParser {
    fn get_generic_error_message(&self) -> String {
        "spaces".to_string()
    }

    fn get_parser_type(&self) -> ParserType {
        ParserType::Spaces
    }

    fn parse_from_context(&self, mut ctx: Context) -> Result<Success<String>, Failure> {
        let whitespace: String = ctx.txt[ctx.pos..]
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect();

        if whitespace.is_empty() {
            return Err(Failure::new("spaces", ctx, vec![ParserType::Spaces]));
        }

        ctx.pos += whitespace.len();
        Ok(Success::new(whitespace, ctx))
    }
}

impl StringParserT<String> for SpacesParser {}
