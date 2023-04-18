use crate::{Context, Parser, ParserType};

pub fn surely<T: 'static>(parser: Parser<T>) -> Parser<T> {
    Box::new(move |ctx: Context| match parser(ctx) {
        Ok(res) => Ok(res),
        Err(mut err) => {
            err.exp = format!("surely expected `{}`", err.exp);
            err.p_type_stack.push(ParserType::Surely);
            Err(err)
        }
    })
}
