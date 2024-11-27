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
        let mut tokens = vec![];

        if self.source.is_empty() {
            Err(ScannerError::EmptySource)?
        }

        while self.chars.peek().is_some() {
            self.start = self.current;

            if let Some(token) = self.evaluate()? {
                tokens.push(token);
            }
        }

        Ok(tokens)
    }

    fn evaluate(&mut self) -> Result<Option<Token>, ScannerError> {
        let char = self.advance().unwrap();

        let res = match char {
            '\n' => {
                self.line += 1;
                self.column_start = 1;
                self.column_end = 2;
                Ok(None)
            }
            '{' => Ok(Some(self.create_token(TokenType::LeftBrace))),
            '}' => Ok(Some(self.create_token(TokenType::RightBrace))),
            '[' => Ok(Some(self.create_token(TokenType::LeftBracket))),
            ']' => Ok(Some(self.create_token(TokenType::RightBracket))),
            ':' => Ok(Some(self.create_token(TokenType::Colon))),
            ',' => Ok(Some(self.create_token(TokenType::Comma))),
            _ => Err(ScannerError::UnknownCharacter)
        };

        if char != '\n' {
            self.column_start = self.column_end;
            self.column_end += 1;
        }
        
        res
    }

    fn advance(&mut self) -> Option<char> {
        if let Some((_, char)) = self.chars.next() {
            self.current += 1;

            return Some(char);
        }

        None
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
    fn tokenize_primary_json_chars() {
        let mut s = Scanner::new("{}[]:,");

        let res = s.scan();

        assert_eq!(Ok(vec![
            Token::new(TokenType::LeftBrace, 1, (0, 1), (1, 2)),
            Token::new(TokenType::RightBrace, 1, (1, 2), (2, 3)),
            Token::new(TokenType::LeftBracket, 1, (2, 3), (3, 4)),
            Token::new(TokenType::RightBracket, 1, (3, 4), (4, 5)),
            Token::new(TokenType::Colon, 1, (4, 5), (5, 6)),
            Token::new(TokenType::Comma, 1, (5, 6), (6, 7)),

        ]), res);
    }

    #[test]
    fn new_line_defaults() {
        let mut s = Scanner::new("[\n,\n]");
        let _ = s.scan();

        assert_eq!(3, s.line);
    }

    #[test]
    fn increment_current_after_each_ascii_char() {
        let mut s = Scanner::new("[]");
        let _ = s.scan();

        assert_eq!(2, s.current)
    }

    #[test]
    fn scan_gives_error_if_source_is_empty() {
        let mut s = Scanner::new("");

        assert_eq!(Err(ScannerError::EmptySource), s.scan())
    }
}
