# microparsec

A simple parser crate for Rust.

### How to use
Look at the documentation to see how to use each parser.

### Example
```rust
use microparsec::{ParserRc, SpacesParser, StringParser, SequenceParser, StringParserT, ContextParserT, parsers};

let hello_parser = StringParser::new("Hello");
let spaces_parser = SpacesParser::new();
let world_parser = StringParser::new("World");
let res = SequenceParser::new(parsers!(hello_parser, spaces_parser, world_parser)).parse("Hello  World");

assert_eq!(
    res.unwrap().val,
    vec!["Hello".to_string(), "  ".to_string(), "World".to_string()]
);
```
