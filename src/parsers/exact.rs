use crate::{ParserRc, Context, ContextParserT, Failure, ParserType, Pos, StringParserT, Success};

/// Attempts to parse a specified number of chars or to the EOI and fails otherwise
/// ## Example
/// ```
/// use parse_me::{ParserRc, ExactParser, StringParser, Pos, ContextParserT, StringParserT};
///
/// let hello_world_parser = StringParser::new("Hello World");
/// let res = ExactParser::new(ParserRc::new(hello_world_parser), Pos::EOI).parse("Hello World");
/// assert_eq!(res.unwrap().val, "Hello World");
/// ```
#[derive(Clone)]
pub struct ExactParser<T> {
    parser: ParserRc<dyn ContextParserT<T>>,
    pos: Pos,
    generic_error: String,
}

impl<T> ExactParser<T> {
    pub fn new(parser: ParserRc<dyn ContextParserT<T>>, pos: Pos) -> Self {
        let generic_error = format!("exactly `{}`", parser.clone().get_generic_error_message());
        ExactParser {
            parser,
            pos,
            generic_error,
        }
    }
}

impl<T> ContextParserT<T> for ExactParser<T> {
    fn get_generic_error_message(&self) -> String {
        self.generic_error.clone()
    }

    fn get_parser_type(&self) -> ParserType {
        ParserType::Exact
    }

    fn parse_from_context(&self, ctx: Context) -> Result<Success<T>, Failure> {
        let prev_pos = ctx.pos;
        let mut res = match self.parser.parse_from_context(ctx) {
            Ok(res) => res,
            Err(mut err) => {
                err.p_type_stack.push(ParserType::Exact);
                return Err(err);
            }
        };

        match self.pos {
            Pos::Chars(x) => {
                if res.ctx.pos - prev_pos == x {
                    Ok(res)
                } else {
                    res.ctx.pos = prev_pos;
                    Err(Failure::new(
                        format!("parsing {x} characters"),
                        res.ctx,
                        vec![ParserType::Exact],
                    ))
                }
            }
            Pos::EOI => {
                if res.ctx.pos == res.ctx.txt.len() {
                    Ok(res)
                } else {
                    res.ctx.pos = prev_pos;
                    Err(Failure::new(
                        "parsing to EOI",
                        res.ctx,
                        vec![ParserType::Exact],
                    ))
                }
            }
        }
    }
}

impl<T> StringParserT<T> for ExactParser<T> {}
