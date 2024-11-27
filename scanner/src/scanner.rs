use std::{iter::Peekable, str::CharIndices};

use token::{token::Token, token_type::TokenType};

use crate::scanner_error::ScannerError;

#[derive(Debug)]
pub struct Scanner<'source> {
    source: &'source str,
    chars: Peekable<CharIndices<'source>>,
    start: usize,
    current: usize,
    line: usize,
    column_start: usize,
    column_end: usize,
}

impl<'source> Scanner<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
            start: 0,
            current: 0,
            line: 1,
            column_start: 1,
            column_end: 2,
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, ScannerError> {
        let tokens = vec![];

        if self.source.is_empty() {
            Err(ScannerError::EmptySource)?
        }

        Ok(tokens)
    }

    fn create_token(&mut self, token_type: TokenType) -> Token {
        Token::new(
            token_type,
            self.line,
            (self.start, self.current),
            (self.column_start, self.column_end),
        )
    }
}

#[cfg(test)]
mod scanner_tests {
    use super::*;

    #[test]
    fn scan_gives_error_if_source_is_empty() {
        let mut s = Scanner::new("");

        assert_eq!(Err(ScannerError::EmptySource), s.scan())
    }
}
