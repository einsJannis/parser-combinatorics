#[macro_use]
extern crate grammar_macros;

pub use grammar_macros::grammar_rule;

use std::path::Path;

pub enum ContentLocation<'s> {
    File(&'s Path),
    String(&'s str)
}

pub struct ContentSpan<'s> {
    content: ContentLocation<'s>,
    span: (usize, usize)
}

