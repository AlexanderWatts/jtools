use std::{iter::Peekable, str::CharIndices};

#[derive(Debug)]
pub struct Lexer<'source> {
    source: &'source str,
    chars: Peekable<CharIndices<'source>>,
    line: usize,
    column: usize,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
            line: 1,
            column: 1,
        }
    }
}

#[cfg(test)]
pub mod lexer_tests {
    use super::*;
}
