use microparsec::{Context, Failure};
use rand::{self, rngs::StdRng, seq::IteratorRandom, Rng, SeedableRng};

const CHARS: &str = "abcdef";

pub fn __test_get_seeded_rng() -> (u64, StdRng) {
    let mut rng = rand::thread_rng();
    let seed = rng.gen();

    (seed, StdRng::seed_from_u64(seed))
}

pub fn __test_get_rand_string(rng: &mut StdRng, n: usize) -> String {
    (0..n).map(|_| CHARS.chars().choose(rng).unwrap()).collect()
}

pub fn __test_get_error_message(err: &str, pos: usize) -> String {
    Failure::new(err, Context::new("", pos), vec![]).get_error_message()
}

/*
#[test]
fn regex_test() {
    let iban_parser = RegexParser::new(r"DE\d{4}\s\d{4}\s\d{4}", "IBAN");

    let res = iban_parser.parse("DE0012 2322 2323");
    assert_eq!(res.clone().unwrap().val, "DE0012 2322 2323");
    assert_eq!(res.unwrap().ctx.pos, 16);

    let res = iban_parser.parse("DE012 2322 2323");
    assert_eq!(
        res.unwrap_err().get_error_message(),
        __test_get_error_message("IBAN", 0)
    );

    let res = iban_parser.parse("Bank Account: DE0012 2322 2323");

    assert_eq!(
        res.unwrap_err().get_error_message(),
        __test_get_error_message("IBAN", 0)
    );
}

#[test]
fn any_test() {
    let hello = StringParser::new("Hello");
    let hallo = StringParser::new("Hallo");
    let hola = StringParser::new("Hola");
    let s_world = StringParser::new(" World");

    let parser1 = SequenceParser::new(parsers!(
        AnyParser::new(parsers!(hallo.clone(), hello)),
        s_world.clone()
    ));
    let parser2 = SequenceParser::new(parsers!(AnyParser::new(parsers!(hallo, hola)), s_world));

    let res = parser1.parse("Hello World");
    assert_eq!(
        res.clone().unwrap().val,
        vec!["Hello".to_string(), " World".to_string()]
    );
    assert_eq!(res.unwrap().ctx.pos, 11);

    let res = parser2.parse("Hello World");
    assert_eq!(
        res.unwrap_err().get_error_message(),
        __test_get_error_message("{ `Hallo` | `Hola` }", 0)
    );
}

#[test]
fn map_test() {
    let hello = StringParser::new("Hello");
    let space = StringParser::new(" ");
    let world = StringParser::new("World");

    let parser = SequenceParser::new(parsers!(hello, space, world));

    let res = MapParser::new(ParserRc::new(parser.clone()), |res| Ok(res.val.join("")))
        .parse("Hello World");
    assert_eq!(res.clone().unwrap().val, "Hello World".to_string());
    assert_eq!(res.unwrap().ctx.pos, 11);

    let res: Result<Success<()>, Failure> =
        MapParser::new(ParserRc::new(parser), |_| Err("mapping()".to_string()))
            .parse("Hello World");
    assert_eq!(
        res.unwrap_err().get_error_message(),
        __test_get_error_message("mapping()", 11)
    );
}

#[test]
fn forget_test() {
    let parser = ForgetParser::new(ParserRc::new(StringParser::new("Hello World")));
    let res = parser.parse("Hello World");
    assert_eq!(res.clone().unwrap().val, ());
    assert_eq!(res.unwrap().ctx.pos, 11);

    let res = parser.parse("Hallo World");
    assert_eq!(
        res.unwrap_err().get_error_message(),
        __test_get_error_message("Hello World", 0)
    );
}

#[test]
fn many_test() {
    let parser1 = ManyParser::new(ParserRc::new(RegexParser::new(r".{1}", "anything")));
    let parser2 = ManyParser::new(ParserRc::new(RegexParser::new(r"\d{1}", "number")));

    let res = parser1.parse("Hello World");
    assert_eq!(res.clone().unwrap().val.join(""), "Hello World");
    assert_eq!(res.unwrap().ctx.pos, 11);

    let res = parser2.parse("Hello World");
    assert_eq!(
        res.unwrap_err().get_error_message(),
        __test_get_error_message("number", 0)
    );
}

#[test]
fn between_test() {
    let quote = ParserRc::new(StringParser::new("\""));
    let hello = ParserRc::new(StringParser::new("Hello"));

    let parser1 = BetweenParser::new(quote.clone(), hello.clone(), quote.clone());
    let parser2 = BetweenParser::new(ParserRc::new(IntegerParser::new()), hello, quote);

    let res = parser1.parse("\"Hello\"");
    assert_eq!(res.clone().unwrap().val, "Hello");
    assert_eq!(res.unwrap().ctx.pos, 7);

    let res = parser2.parse("1Hello\"");
    assert_eq!(res.clone().unwrap().val, "Hello");
    assert_eq!(res.unwrap().ctx.pos, 7);

    let res = parser1.parse("\"Hello1");
    assert_eq!(
        res.unwrap_err().get_error_message(),
        __test_get_error_message("\"", 6)
    );
}

#[test]
fn exact_test() {
    let hello_world = ParserRc::new(StringParser::new("Hello World"));
    let hello_wor = ParserRc::new(StringParser::new("Hello Wor"));
    let llo_world = ParserRc::new(StringParser::new("llo World"));

    let res = ExactParser::new(hello_world.clone(), Pos::EOI).parse("Hello World");
    assert_eq!(res.clone().unwrap().val, "Hello World");
    assert_eq!(res.unwrap().ctx.pos, 11);

    let res = ExactParser::new(hello_wor, Pos::Chars(9)).parse("Hello World");
    assert_eq!(res.clone().unwrap().val, "Hello Wor");
    assert_eq!(res.unwrap().ctx.pos, 9);

    let res = ExactParser::new(llo_world, Pos::Chars(9))
        .parse_from_context(Context::new("Hello World", 2));
    assert_eq!(res.clone().unwrap().val, "llo World");
    assert_eq!(res.unwrap().ctx.pos, 11);

    let res = ExactParser::new(hello_world.clone(), Pos::EOI).parse("Hello World.");
    assert_eq!(
        res.unwrap_err().get_error_message(),
        __test_get_error_message("parsing to EOI", 0)
    );

    let res = ExactParser::new(hello_world, Pos::Chars(12)).parse("Hello World.");
    assert_eq!(
        res.unwrap_err().get_error_message(),
        __test_get_error_message("parsing 12 characters", 0)
    );
}

#[test]
fn spaces_test() {
    let hello = StringParser::new("Hello");
    let spaces = SpacesParser::new();
    let world = StringParser::new("World");

    let parser = SequenceParser::new(parsers!(hello, spaces, world));

    let res = parser.parse("Hello World");
    assert_eq!(
        res.clone().unwrap().val,
        vec!["Hello".to_string(), " ".to_string(), "World".to_string()]
    );
    assert_eq!(res.unwrap().ctx.pos, 11);

    let res = parser.parse("HelloWorld");
    assert_eq!(
        res.unwrap_err().get_error_message(),
        __test_get_error_message("spaces", 5)
    );

    let res = parser.parse("Hello    World");
    assert_eq!(
        res.clone().unwrap().val,
        (vec!["Hello".to_string(), "    ".to_string(), "World".to_string()])
    );
    assert_eq!(res.unwrap().ctx.pos, 14);
}

#[test]
fn letters_test() {
    let letters = LettersParser::new();

    let res = letters.parse("Hello");
    assert_eq!(res.clone().unwrap().val, "Hello");
    assert_eq!(res.unwrap().ctx.pos, 5);

    let res = letters.parse("Hello!");
    assert_eq!(res.clone().unwrap().val, "Hello");
    assert_eq!(res.unwrap().ctx.pos, 5);

    let res = letters.parse("1Hello");
    assert_eq!(
        res.unwrap_err().get_error_message(),
        __test_get_error_message("letters", 0)
    );
}

#[test]
fn integer_test() {
    let integer = IntegerParser::new();

    let res = integer.parse("123456789");
    assert_eq!(res.clone().unwrap().val, "123456789");
    assert_eq!(res.unwrap().ctx.pos, 9);

    let res = integer.parse("a123456789");
    assert_eq!(
        res.unwrap_err().get_error_message(),
        __test_get_error_message("integer", 0)
    );
}

#[test]
fn float_test() {
    let float = FloatParser::new();

    let res = float.parse("12345.6789");
    assert_eq!(res.clone().unwrap().val, "12345.6789");
    assert_eq!(res.unwrap().ctx.pos, 10);

    let res = float.parse("a1234.56789");
    assert_eq!(
        res.unwrap_err().get_error_message(),
        __test_get_error_message("float", 0)
    );
}

#[test]
fn expect_test() {
    let hello = ParserRc::new(StringParser::new("Hello"));
    let parser = ExpectParser::new(hello, "\"Hello\"");

    let res = parser.parse("Hello World");
    assert_eq!(res.clone().unwrap().val, "Hello");
    assert_eq!(res.unwrap().ctx.pos, 5);

    let res = parser.parse("Hallo Welt");
    assert_eq!(
        res.unwrap_err().get_error_message(),
        __test_get_error_message("\"Hello\"", 0)
    );
}

#[test]
fn surely_test() {
    let hello = StringParser::new("Hello");
    let hallo = StringParser::new("Hallo");
    let hola = StringParser::new("Hola");
    let world = StringParser::new("World");
    let welt = StringParser::new("Welt");
    let spaces = SpacesParser::new();

    let parser1 = AnyParser::new(parsers!(
        hallo.clone(),
        SurelyParser::new(ParserRc::new(hello.clone()))
    ));
    let parser2 = AnyParser::new(parsers!(
        hallo.clone(),
        SurelyParser::new(ParserRc::new(hola)),
        hello.clone()
    ));
    let p31 = SequenceParser::new(parsers!(
        hello,
        spaces.clone(),
        SurelyParser::new(ParserRc::new(world))
    ));
    let p32 = SequenceParser::new(parsers!(
        hallo,
        spaces,
        SurelyParser::new(ParserRc::new(welt))
    ));
    let parser3 = AnyParser::new(parsers!(p31, p32));

    let res = parser1.parse("Hello World");
    assert_eq!(res.clone().unwrap().val, "Hello");
    assert_eq!(res.unwrap().ctx.pos, 5);

    let res = parser2.parse("Hello World");
    assert_eq!(
        res.unwrap_err().get_error_message(),
        __test_get_error_message("surely `Hola`", 0)
    );

    let res = parser3.parse("Hello Welt");
    assert_eq!(
        res.unwrap_err().get_error_message(),
        __test_get_error_message("surely `World`", 6)
    );
}

#[test]
fn p_type_stack_test() {
    let hello = StringParser::new("Hello");
    let hallo = StringParser::new("Hallo");
    let hi = StringParser::new("Hi");
    let world = StringParser::new("World");
    let welt = StringParser::new("Welt");
    let spaces = SpacesParser::new();
    let integer = IntegerParser::new();

    let parser1 = SequenceParser::new(parsers!(hello.clone(), spaces.clone(), welt.clone()));
    let p21 = SequenceParser::new(parsers!(
        hello,
        spaces.clone(),
        SurelyParser::new(ParserRc::new(world))
    ));
    let p22 = SequenceParser::new(parsers!(
        hallo.clone(),
        spaces,
        SurelyParser::new(ParserRc::new(welt))
    ));
    let parser2 = AnyParser::new(parsers!(p21, p22));
    let parser3 = AnyParser::new(parsers!(hi, hallo, integer));

    let res = parser1.parse("Hello World");
    assert_eq!(
        res.clone().unwrap_err().p_type_stack,
        vec![ParserType::String, ParserType::Sequence]
    );
    assert_eq!(
        res.unwrap_err().get_error_message_stack_trace(),
        "[Parser error] Expected `Welt` at position: 6\n\nCall Stack:\n2. `string` parser\n1. `sequence` parser"
    );

    let res = parser2.parse("Hello Welt");
    assert_eq!(
        res.clone().unwrap_err().p_type_stack,
        vec![
            ParserType::String,
            ParserType::Surely,
            ParserType::Sequence,
            ParserType::Any,
        ]
    );
    assert_eq!(
        res.unwrap_err().get_error_message_stack_trace(),
        "[Parser error] Expected `surely `World`` at position: 6\n\nCall Stack:\n4. `string` parser\n3. `surely` parser\n2. `sequence` parser\n1. `any` parser"
    );

    let res = parser3.parse("Hello");
    assert_eq!(
        res.clone().unwrap_err().p_type_stack,
        vec![ParserType::Any,]
    );
    assert_eq!(
        res.unwrap_err().get_error_message_stack_trace(),
        "[Parser error] Expected `{ `Hi` | `Hallo` | `integer` }` at position: 0\n\nCall Stack:\n1. `any` parser"
    );
}
*/
