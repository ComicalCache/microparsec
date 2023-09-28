use crate::{Context, ParserType, ContextParserT, StringParserT, Success, Failure, RegexParser};

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

    fn parse_from_context(&self, ctx: Context) -> Result<Success<String>, Failure> {
        match RegexParser::new("[ ]+", "spaces").parse_from_context(ctx) {
            Ok(res) => Ok(res),
            Err(mut err) => {
                err.p_type_stack.push(ParserType::Spaces);
                Err(err)
            }
        }
    }
}

impl StringParserT<String> for SpacesParser {}
