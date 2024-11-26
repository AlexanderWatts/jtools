use std::{iter::Peekable, str::CharIndices};

use token::token::Token;

use crate::lexer_error::LexerError;

#[derive(Debug)]
pub struct Lexer<'source> {
    source: &'source str,
    chars: Peekable<CharIndices<'source>>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
            line: 1,
            start: 0,
            current: 0,
            column: 1,
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, LexerError> {
        let tokens = vec![];

        while let Some((_, character)) = self.chars.next() {
            self.start = self.current;

            match character {
                _ => {
                    if character.is_alphabetic() {
                        self.scan_alphabetic();
                    } else {
                        Err(LexerError::UnknownCharacter)?
                    }
                }
            }
        }

        Ok(tokens)
    }

    fn scan_alphabetic(&mut self) -> &'source str {
        while let Some((character_index, character)) = self
            .chars
            .next_if(|&(_, character)| character.is_alphabetic())
        {
            self.current = character_index + character.len_utf8();
        }

        &self.source[self.start..self.current]
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn lex() {
        let mut l = Lexer::new("true");

        assert_eq!(Ok(vec![]), l.scan());
    }
}
