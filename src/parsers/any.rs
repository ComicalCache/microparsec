use crate::{Context, ContextParserT, Failure, ParserRc, ParserType, StringParserT, Success};

/// Parses for any of the supplied parsers and returns the first successful result,
/// or an error if no parser matched.
/// ### Example
/// ```
/// use parse_me::{ParserRc, AnyParser, StringParser, ContextParserT, StringParserT, parsers};
///
/// let hello_parser = StringParser::new("Hello");
/// let hallo_parser = StringParser::new("Hallo");
/// let res = AnyParser::new(parsers!(hallo_parser, hello_parser)).parse("Hello World");
/// assert_eq!(res.unwrap().val, "Hello");
/// ```
#[derive(Clone)]
pub struct AnyParser<T> {
    parsers: Vec<ParserRc<dyn ContextParserT<T>>>,
    generic_error: String,
}

impl<T> AnyParser<T> {
    pub fn new(parsers: Vec<ParserRc<dyn ContextParserT<T>>>) -> Self {
        let generic_error = format!(
            "{{ `{}` }}",
            parsers
                .iter()
                .map(|p| p.get_generic_error_message().to_string())
                .collect::<Vec<String>>()
                .join("` | `")
        );

        AnyParser {
            parsers,
            generic_error,
        }
    }
}

impl<T> ContextParserT<T> for AnyParser<T> {
    fn get_generic_error_message(&self) -> String {
        self.generic_error.clone()
    }

    fn get_parser_type(&self) -> ParserType {
        ParserType::Any
    }

    fn parse_from_context(&self, ctx: Context) -> Result<Success<T>, Failure> {
        for parser in self.parsers.iter() {
            match parser.parse_from_context(ctx.clone()) {
                Ok(res) => return Ok(res),
                Err(mut err) => {
                    if err.p_type_stack.contains(&ParserType::Surely) {
                        err.p_type_stack.push(ParserType::Any);
                        return Err(err);
                    }
                }
            }
        }

        return Err(Failure::new(
            self.generic_error.clone(),
            ctx,
            vec![ParserType::Any],
        ));
    }
}

impl<T> StringParserT<T> for AnyParser<T> {}
