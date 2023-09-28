use crate::{Context, ContextParserT, Failure, ParserRc, ParserType, StringParserT, Success};

/// Maps the result of a parser to a new value
/// ## Example
/// ```
/// use parse_me::{MapParser, IntegerParser, SequenceParser, StringParser, ParserRc, ContextParserT, StringParserT, parsers};
///
/// let hello_parser = StringParser::new("Hello");
/// let space_parser = StringParser::new(" ");
/// let world_parser = StringParser::new("World");
/// let hello_world_parser = SequenceParser::new(parsers!(hello_parser, space_parser, world_parser));
/// let res = MapParser::new(ParserRc::new(hello_world_parser),
///                          |res| Ok(res.val.join("")))
///             .parse("Hello World");
/// assert_eq!(res.unwrap().val, "Hello World");
///
/// let res = MapParser::new(ParserRc::new(IntegerParser::new()),
///                          |res| Ok(res.val.parse::<u32>().unwrap()))
///             .parse("234");
/// assert_eq!(res.unwrap().val, 234);
/// ```
#[derive(Clone)]
pub struct MapParser<T, M> {
    parser: ParserRc<dyn ContextParserT<T>>,
    mapper: fn(Success<T>) -> Result<M, String>,
    generic_error: String,
}

impl<T, M> MapParser<T, M> {
    pub fn new(parser: ParserRc<dyn ContextParserT<T>>,
               mapper: fn(Success<T>) -> Result<M, String>) -> Self {
        let generic_error = format!("mapping `{}`", parser.get_generic_error_message());

        MapParser {
            parser,
            mapper,
            generic_error,
        }
    }
}

impl<T, M> ContextParserT<M> for MapParser<T, M> {
    fn get_generic_error_message(&self) -> String {
        self.generic_error.clone()
    }

    fn get_parser_type(&self) -> ParserType {
        ParserType::Map
    }

    fn parse_from_context(&self, ctx: Context) -> Result<Success<M>, Failure> {
        let res = match self.parser.parse_from_context(ctx.clone()) {
            Ok(res) => res,
            Err(mut err) => {
                err.p_type_stack.push(ParserType::Map);
                return Err(err);
            }
        };

        let ctx = res.ctx.clone();
        match (self.mapper)(res) {
            Ok(mapped) => Ok(Success::new(mapped, ctx)),
            Err(map_err) => Err(Failure::new(map_err, ctx, vec![ParserType::Map])),
        }
    }
}

impl<T, M> StringParserT<M> for MapParser<T, M> {}
