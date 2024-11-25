use std::{iter::Peekable, str::CharIndices};

use crate::{lexer_error::LexerError, Token};

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

    pub fn scan(&mut self) -> Result<Vec<Token>, LexerError> {
        let tokens = vec![];

        Ok(tokens)
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;
}
