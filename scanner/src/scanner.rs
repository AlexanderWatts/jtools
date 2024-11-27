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

        while let Some((_, char)) = self.chars.next() {
            match char {
                '\n' => {
                    self.line += 1;
                    self.column_start = 1;
                    self.column_end = 2;
                }
                _ => {
                    if self.chars.peek().is_some() {
                        self.column_start = self.column_end;
                        self.column_end += 1;
                    }
                }
            }

            self.current += 1;
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
    fn new_line_defaults() {
        let mut s = Scanner::new("[\nfalse\n]");
        let _ = s.scan();

        assert_eq!(3, s.line);
        assert_eq!((1, 2), (s.column_start, s.column_end));
    }

    #[test]
    fn increment_current_after_each_ascii_char() {
        let mut s = Scanner::new("[false]");
        let _ = s.scan();

        assert_eq!(7, s.current)
    }

    #[test]
    fn scan_gives_error_if_source_is_empty() {
        let mut s = Scanner::new("");

        assert_eq!(Err(ScannerError::EmptySource), s.scan())
    }
}
