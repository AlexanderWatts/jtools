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
            column_end: 1,
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
                self.column_end = 1;
                Ok(None)
            }
            '{' => Ok(Some(self.create_token(TokenType::LeftBrace))),
            '}' => Ok(Some(self.create_token(TokenType::RightBrace))),
            '[' => Ok(Some(self.create_token(TokenType::LeftBracket))),
            ']' => Ok(Some(self.create_token(TokenType::RightBracket))),
            ':' => Ok(Some(self.create_token(TokenType::Colon))),
            ',' => Ok(Some(self.create_token(TokenType::Comma))),
            '\"' => {
                while let Some(_) = self.advance_if(|&(_, char)| char != '\"') {}

                if self.chars.peek().is_none() {
                    Err(ScannerError::UnterminatedString)?
                }

                self.advance();

                Ok(Some(self.create_token(TokenType::String)))
            }
            _ => {
                if char.is_ascii_alphabetic() {
                    while let Some(_) = self.advance_if(|&(_, char)| char.is_ascii_alphabetic()) {}

                    let result = match &self.source[self.start..self.current] {
                        "true" => self.create_token(TokenType::True),
                        "false" => self.create_token(TokenType::False),
                        "null" => self.create_token(TokenType::Null),
                        _ => Err(ScannerError::UnknownLiteral)?,
                    };

                    Ok(Some(result))
                } else {
                    Err(ScannerError::UnknownCharacter)
                }
            }
        };

        if char != '\n' {
            self.column_start = self.column_end;
        }

        res
    }

    fn advance_if<F>(&mut self, predicate: F) -> Option<char>
    where
        F: Fn(&(usize, char)) -> bool,
    {
        if let Some((char_index, char)) = self.chars.next_if(predicate) {
            self.current = char_index + char.len_utf8();
            self.column_end += 1;

            return Some(char);
        }

        None
    }

    fn advance(&mut self) -> Option<char> {
        if let Some((_, char)) = self.chars.next() {
            self.current += 1;
            self.column_end += 1;

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
    fn scan_unterminated_string() {
        assert_eq!(
            Err(ScannerError::UnterminatedString),
            Scanner::new("\"").scan()
        );

        assert_eq!(
            Err(ScannerError::UnterminatedString),
            Scanner::new("\"language").scan()
        );
    }

    // Graphemes take up 1 display column
    #[test]
    fn scan_valid_string_with_graphemes() {
        assert_eq!(
            Ok(vec![Token::new(TokenType::String, 1, (0, 10), (1, 5))]),
            Scanner::new("\"🌎🚀\"").scan()
        );
    }

    #[test]
    fn scan_valid_strings() {
        assert_eq!(
            Ok(vec![Token::new(TokenType::String, 1, (0, 15), (1, 16))]),
            Scanner::new("\"hello, world!\"").scan()
        );

        assert_eq!(
            Ok(vec![Token::new(TokenType::String, 1, (0, 2), (1, 3))]),
            Scanner::new("\"\"").scan()
        );
    }

    #[test]
    fn scan_invalid_words() {
        assert_eq!(
            Err(ScannerError::UnknownLiteral),
            Scanner::new("hello").scan()
        );

        assert_eq!(
            Err(ScannerError::UnknownLiteral),
            Scanner::new("unknown").scan()
        );
    }

    #[test]
    fn scan_valid_keywords() {
        assert_eq!(
            Ok(vec![Token::new(TokenType::True, 1, (0, 4), (1, 5))]),
            Scanner::new("true").scan()
        );

        assert_eq!(
            Ok(vec![Token::new(TokenType::False, 1, (0, 5), (1, 6))]),
            Scanner::new("false").scan()
        );

        assert_eq!(
            Ok(vec![Token::new(TokenType::Null, 1, (0, 4), (1, 5))]),
            Scanner::new("null").scan()
        );
    }
    //
    #[test]
    fn tokenize_primary_json_chars() {
        let mut s = Scanner::new("{}[]:,");

        let res = s.scan();

        assert_eq!(
            Ok(vec![
                Token::new(TokenType::LeftBrace, 1, (0, 1), (1, 2)),
                Token::new(TokenType::RightBrace, 1, (1, 2), (2, 3)),
                Token::new(TokenType::LeftBracket, 1, (2, 3), (3, 4)),
                Token::new(TokenType::RightBracket, 1, (3, 4), (4, 5)),
                Token::new(TokenType::Colon, 1, (4, 5), (5, 6)),
                Token::new(TokenType::Comma, 1, (5, 6), (6, 7)),
            ]),
            res
        );
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
