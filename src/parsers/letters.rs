use crate::{Context, ContextParserT, Failure, ParserType, StringParserT, Success};

/// Parses for at least one letter
/// ## Example
/// ```
/// use microparsec::{LettersParser, StringParserT, ContextParserT};
///
/// let res = LettersParser::new().parse("Hello");
/// assert_eq!(res.unwrap().val, "Hello");
/// ```
#[derive(Clone)]
pub struct LettersParser {}

impl LettersParser {
    #[allow(clippy::new_without_default)]
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

    fn parse_from_context(&self, mut ctx: Context) -> Result<Success<String>, Failure> {
        let letters: String = ctx.txt[ctx.pos..]
            .chars()
            .take_while(|c| c.is_alphabetic())
            .collect();

        if letters.is_empty() {
            return Err(Failure::new("letters", ctx, vec![ParserType::Letters]));
        }

        ctx.pos += letters.len();
        Ok(Success::new(letters, ctx))
    }
}

impl StringParserT<String> for LettersParser {}
