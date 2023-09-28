#![allow(unused_macros)]

mod string_utils;

mod types;
pub use types::*;

mod parsers;
pub use parsers::*;

#[macro_export]
macro_rules! parsers {
    ($p:ident) => {
        vec![ParserRc::new($p)]
    };
    ($($p:expr),+) => {
        vec![$(ParserRc::new($p)),*]
    };
}
