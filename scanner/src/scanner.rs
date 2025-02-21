use core::f64;
use error_preview::error_preview::ErrorPreview;
use std::{cell::Cell, iter::Peekable, str::CharIndices};
use token::{token::Token, token_type::TokenType};

use crate::scanner_error::ScannerError;

/// Handwritten scanner/lexical analyser
///
/// ## Description
///
/// The scanner iterates over every character in a given source, O(n), and groups those characters
/// into tokens which are fed to the parser if scanning is successful.
///
/// ## Examples
/// ```
/// use scanner::scanner::Scanner;
/// use token::{token_type::TokenType, token::Token};
///
/// let mut scanner = Scanner::new("{ \"data\": [1] }");
/// let tokens = scanner.scan();
///
/// assert_eq!(
///     Ok(vec![
///         Token {
///             token_type: TokenType::LeftBrace,
///             line_number: 1,
///             indices: (0, 1,),
///             column_indices: (1, 2,),
///         },
///         Token {
///             token_type: TokenType::String,
///             line_number: 1,
///             indices: (2, 8,),
///             column_indices: (3, 9,),
///         },
///         Token {
///             token_type: TokenType::Colon,
///             line_number: 1,
///             indices: (8, 9,),
///             column_indices: (9, 10,),
///         },
///         Token {
///             token_type: TokenType::LeftBracket,
///             line_number: 1,
///             indices: (10, 11,),
///             column_indices: (11, 12,),
///         },
///         Token {
///             token_type: TokenType::Number,
///             line_number: 1,
///             indices: (11, 12,),
///             column_indices: (12, 13,),
///         },
///         Token {
///             token_type: TokenType::RightBracket,
///             line_number: 1,
///             indices: (12, 13,),
///             column_indices: (13, 14,),
///         },
///         Token {
///             token_type: TokenType::RightBrace,
///             line_number: 1,
///             indices: (14, 15,),
///             column_indices: (15, 16,),
///         },
///         Token {
///             token_type: TokenType::Eof,
///             line_number: 1,
///             indices: (15, 15,),
///             column_indices: (16, 16,),
///         },
///     ],),
///     tokens
/// )
/// ```
#[derive(Debug)]
pub struct Scanner<'source> {
    pub source: &'source str,
    chars: Peekable<CharIndices<'source>>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub column_start: usize,
    pub column_end: usize,

    characters: Vec<char>,
    start_position: Cell<usize>,
    current_position: Cell<usize>,
}

impl<'source> Scanner<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
            start: 0,
            current: 0,
            line: 1,
            column_start: 0,
            column_end: 1,

            characters: source.chars().collect(),
            start_position: Cell::new(0),
            current_position: Cell::new(0),
        }
    }

    fn peek(&self) -> Option<&char> {
        self.characters.get(self.current_position.get())
    }

    fn next(&self) -> Option<&char> {
        let next = self.characters.get(self.current_position.get());

        self.current_position.set(self.current_position.get() + 1);

        next
    }

    fn error_preview(&self, start: Option<usize>, column_start: Option<usize>) -> String {
        ErrorPreview.preview(
            self.source,
            start.unwrap_or(self.start),
            column_start.unwrap_or(self.column_start),
            self.line,
        )
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, ScannerError> {
        let mut tokens = vec![];

        if self.source.is_empty() {
            Err(ScannerError::EmptySource {
                error: self.error_preview(None, Some(1)),
            })?
        }

        while self.chars.peek().is_some() {
            self.start = self.current;

            if let Some(token) = self.evaluate()? {
                tokens.push(token);
            }
        }

        tokens.push(Token::new(
            TokenType::Eof,
            self.line,
            (self.current, self.current),
            (self.column_end, self.column_end),
        ));

        Ok(tokens)
    }

    pub fn get_token(&self) -> Result<Token, ScannerError> {
        let token_type = self.eval()?;

        Ok(Token::new(token_type, 1, (0, 1), (1, 2)))
    }

    fn eval(&self) -> Result<TokenType, ScannerError> {
        while let Some(' ' | '\r' | '\t' | '\n') = self.peek() {
            self.next();
        }

        self.start_position.set(self.current_position.get());

        match self.next() {
            Some(character) => match character {
                '{' => Ok(TokenType::LeftBrace),
                '}' => Ok(TokenType::RightBrace),
                '[' => Ok(TokenType::LeftBracket),
                ']' => Ok(TokenType::RightBracket),
                ':' => Ok(TokenType::Colon),
                ',' => Ok(TokenType::Comma),
                _ => Err(ScannerError::UnknownCharacter {
                    error: "".to_string(),
                }),
            },
            None => Ok(TokenType::Eof),
        }
    }

    fn evaluate(&mut self) -> Result<Option<Token>, ScannerError> {
        let char = self.advance().unwrap();

        match char {
            ' ' | '\t' | '\r' => Ok(None),
            '\n' => {
                self.line += 1;
                self.column_start = 0;
                self.column_end = 1;
                Ok(None)
            }
            '{' => Ok(Some(self.create_token(TokenType::LeftBrace, None))),
            '}' => Ok(Some(self.create_token(TokenType::RightBrace, None))),
            '[' => Ok(Some(self.create_token(TokenType::LeftBracket, None))),
            ']' => Ok(Some(self.create_token(TokenType::RightBracket, None))),
            ':' => Ok(Some(self.create_token(TokenType::Colon, None))),
            ',' => Ok(Some(self.create_token(TokenType::Comma, None))),
            '\"' => self.scan_string(),
            '0' => {
                if matches!(self.chars.peek(), Some(&(_, char)) if char.is_ascii_digit()) {
                    Err(ScannerError::LeadingZeros {
                        error: self.error_preview(None, None),
                    })?
                }

                self.scan_number()
            }
            '-' => self.scan_number(),
            _ => {
                if char.is_ascii_alphabetic() {
                    self.scan_keyword()
                } else if char.is_ascii_digit() {
                    self.scan_number()
                } else {
                    Err(ScannerError::UnknownCharacter {
                        error: self.error_preview(None, None),
                    })?
                }
            }
        }
    }

    fn scan_number(&mut self) -> Result<Option<Token>, ScannerError> {
        let number_column_start = self.column_start;

        while let Some(_) = self.advance_if(|&(_, char)| char.is_ascii_digit()) {}

        if self.advance_if(|&(_, char)| char == '.').is_some() {
            match self.chars.peek() {
                Some(&(_, char)) if !char.is_ascii_digit() => {
                    Err(ScannerError::UnterminatedFractionalNumber {
                        error: self.error_preview(None, Some(number_column_start)),
                    })?
                }
                None => Err(ScannerError::UnterminatedFractionalNumber {
                    error: self.error_preview(None, Some(number_column_start)),
                })?,
                _ => {}
            }

            while let Some(_) = self.advance_if(|&(_, char)| char.is_ascii_digit()) {}
        }

        if self
            .advance_if(|&(_, char)| char == 'e' || char == 'E')
            .is_some()
        {
            let exponent_start = self.current - 1;
            let exponent_column_start = self.column_start;

            if self
                .advance_if(|&(_, char)| char == '+' || char == '-')
                .is_some()
            {}

            match self.chars.peek() {
                Some(&(_, char)) if !char.is_ascii_digit() => Err(ScannerError::InvalidExponent {
                    error: self.error_preview(Some(exponent_start), Some(exponent_column_start)),
                })?,
                None => Err(ScannerError::InvalidExponent {
                    error: self.error_preview(Some(exponent_start), Some(exponent_column_start)),
                })?,
                _ => {}
            }

            while let Some(_) = self.advance_if(|&(_, char)| char.is_ascii_digit()) {}
        }

        match &self.source[self.start..self.current].parse::<f64>() {
            Ok(number) if number.is_finite() => Ok(Some(
                self.create_token(TokenType::Number, Some(number_column_start)),
            )),
            _ => Err(ScannerError::InvalidNumber {
                error: self.error_preview(None, Some(number_column_start)),
            })?,
        }
    }

    fn scan_string(&mut self) -> Result<Option<Token>, ScannerError> {
        let string_column_start = self.column_start;

        while let Some(char) = self.advance_if(|&(_, char)| char != '\"') {
            if char == '\n' {
                Err(ScannerError::UnterminatedString {
                    error: self.error_preview(None, Some(string_column_start)),
                })?
            }

            if char == '\\' {
                let escape_start = self.current - 1;
                let escape_column_start = self.column_start;

                match self.chars.peek() {
                    Some(&(_, char)) if char == 'u' => {
                        self.advance();

                        for _ in 0..4 {
                            if self
                                .advance_if(|&(_, char)| char.is_ascii_hexdigit())
                                .is_none()
                            {
                                Err(ScannerError::InvalidUnicodeSequence {
                                    error: self.error_preview(
                                        Some(escape_start),
                                        Some(escape_column_start),
                                    ),
                                })?
                            }
                        }
                    }
                    Some(&(_, char))
                        if matches!(char, '\"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't') =>
                    {
                        self.advance();
                    }
                    _ => Err(ScannerError::InvalidEscapeSequence {
                        error: self.error_preview(Some(escape_start), Some(escape_column_start)),
                    })?,
                };
            }
        }

        if self.chars.peek().is_none() {
            Err(ScannerError::UnterminatedString {
                error: self.error_preview(None, Some(string_column_start)),
            })?
        }

        self.advance();

        Ok(Some(self.create_token(
            TokenType::String,
            Some(string_column_start),
        )))
    }

    fn scan_keyword(&mut self) -> Result<Option<Token>, ScannerError> {
        let keyword_column_start = self.column_start;

        while let Some(_) = self.advance_if(|&(_, char)| char.is_ascii_alphabetic()) {}

        let result = match &self.source[self.start..self.current] {
            "true" => self.create_token(TokenType::True, Some(keyword_column_start)),
            "false" => self.create_token(TokenType::False, Some(keyword_column_start)),
            "null" => self.create_token(TokenType::Null, Some(keyword_column_start)),
            _ => Err(ScannerError::UnknownLiteral {
                error: self.error_preview(None, Some(keyword_column_start)),
            })?,
        };

        Ok(Some(result))
    }

    fn advance_if<F>(&mut self, predicate: F) -> Option<char>
    where
        F: Fn(&(usize, char)) -> bool,
    {
        if let Some((char_index, char)) = self.chars.next_if(predicate) {
            self.current = char_index + char.len_utf8();
            self.column_start += 1;
            self.column_end += 1;

            return Some(char);
        }

        None
    }

    fn advance(&mut self) -> Option<char> {
        if let Some((char_index, char)) = self.chars.next() {
            self.current = char_index + char.len_utf8();
            self.column_start += 1;
            self.column_end += 1;

            return Some(char);
        }

        None
    }

    fn create_token(&mut self, token_type: TokenType, column_start: Option<usize>) -> Token {
        Token::new(
            token_type,
            self.line,
            (self.start, self.current),
            (column_start.unwrap_or(self.column_start), self.column_end),
        )
    }
}

#[cfg(test)]
mod scanner_tests {
    use super::*;

    #[test]
    fn ignore_and_consume_spaces() {
        let scanner = Scanner::new("    {  \n\t \t},");

        assert_eq!(Ok(TokenType::LeftBrace), scanner.eval());
        assert_eq!(Ok(TokenType::RightBrace), scanner.eval());
        assert_eq!(Ok(TokenType::Comma), scanner.eval());
    }

    #[test]
    fn evaluate() {
        let scanner = Scanner::new("{}");

        assert_eq!(Ok(TokenType::LeftBrace), scanner.eval());
        assert_eq!(Ok(TokenType::RightBrace), scanner.eval());
    }

    #[test]
    fn starting_position() {
        let scanner = Scanner::new("[]");

        assert_eq!(Cell::new(0), scanner.start_position);
        let _ = scanner.eval();

        let _ = scanner.eval();
        assert_eq!(Cell::new(1), scanner.start_position);
    }

    #[test]
    fn peek_character() {
        let scanner = Scanner::new("Hi 🌎!");

        assert_eq!(Some(&'H'), scanner.peek());
        assert_eq!(Cell::new(0), scanner.current_position);

        assert_eq!(Some(&'H'), scanner.peek());
        assert_eq!(Cell::new(0), scanner.current_position);

        scanner.next();

        assert_eq!(Some(&'i'), scanner.peek());
        assert_eq!(Cell::new(1), scanner.current_position);
    }

    #[test]
    fn get_next_character() {
        let scanner = Scanner::new("Hi 🌎!");

        assert_eq!(Some(&'H'), scanner.next());
        assert_eq!(Cell::new(1), scanner.current_position);

        assert_eq!(Some(&'i'), scanner.next());
        assert_eq!(Cell::new(2), scanner.current_position);

        assert_eq!(Some(&' '), scanner.next());
        assert_eq!(Cell::new(3), scanner.current_position);

        assert_eq!(Some(&'🌎'), scanner.next());
        assert_eq!(Cell::new(4), scanner.current_position);

        assert_eq!(Some(&'!'), scanner.next());
        assert_eq!(Cell::new(5), scanner.current_position);
    }

    #[test]
    fn error_preview() {
        let source = "@";
        let mut s = Scanner::new(source);

        assert_eq!(true, s.scan().is_err())
    }

    #[test]
    fn ignore_spacing_and_maintain_display_column() {
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::LeftBracket, 1, (0, 1), (1, 2)),
                Token::new(TokenType::RightBracket, 1, (2, 3), (3, 4)),
                Token::new(TokenType::Eof, 1, (3, 3), (4, 4))
            ]),
            Scanner::new("[ ]").scan()
        );
    }

    #[test]
    fn invalid_exponents() {
        assert_eq!(true, Scanner::new("27e").scan().is_err());

        assert_eq!(true, Scanner::new("92.3eE").scan().is_err());

        assert_eq!(true, Scanner::new("83e-").scan().is_err());

        assert_eq!(true, Scanner::new("83E+").scan().is_err());
    }

    #[test]
    fn valid_exponents() {
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Number, 1, (0, 5), (1, 6)),
                Token::new(TokenType::Eof, 1, (5, 5), (6, 6)),
            ]),
            Scanner::new("360e2").scan()
        );

        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Number, 1, (0, 4), (1, 5)),
                Token::new(TokenType::Eof, 1, (4, 4), (5, 5)),
            ]),
            Scanner::new("29E8").scan()
        );

        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Number, 1, (0, 7), (1, 8)),
                Token::new(TokenType::Eof, 1, (7, 7), (8, 8)),
            ]),
            Scanner::new("29e+100").scan()
        );

        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Number, 1, (0, 5), (1, 6)),
                Token::new(TokenType::Eof, 1, (5, 5), (6, 6)),
            ]),
            Scanner::new("29e-2").scan()
        );
    }

    #[test]
    fn do_not_allow_leading_zeros_in_number() {
        assert_eq!(true, Scanner::new("000.23432").scan().is_err());
        assert_eq!(true, Scanner::new("00202").scan().is_err());
    }

    #[test]
    fn scan_valid_numbers() {
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Number, 1, (0, 1), (1, 2)),
                Token::new(TokenType::Eof, 1, (1, 1), (2, 2))
            ]),
            Scanner::new("0").scan()
        );

        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Number, 1, (0, 3), (1, 4)),
                Token::new(TokenType::Eof, 1, (3, 3), (4, 4))
            ]),
            Scanner::new("360").scan()
        );

        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Number, 1, (0, 7), (1, 8)),
                Token::new(TokenType::Eof, 1, (7, 7), (8, 8))
            ]),
            Scanner::new("360.360").scan()
        );

        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Number, 1, (0, 5), (1, 6)),
                Token::new(TokenType::Eof, 1, (5, 5), (6, 6))
            ]),
            Scanner::new("-1066").scan()
        );
    }

    #[test]
    fn scan_unterminated_string() {
        assert_eq!(true, Scanner::new("\"").scan().is_err());

        assert_eq!(true, Scanner::new("\"language").scan().is_err());
    }

    // Graphemes take up 1 display column
    #[test]
    fn scan_valid_string_with_graphemes() {
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::String, 1, (0, 10), (1, 5)),
                Token::new(TokenType::Eof, 1, (10, 10), (5, 5))
            ]),
            Scanner::new("\"🌎🚀\"").scan()
        );

        assert_eq!(
            Ok(vec![
                Token::new(TokenType::String, 1, (0, 10), (1, 5)),
                Token::new(TokenType::Eof, 1, (10, 10), (5, 5))
            ]),
            Scanner::new("\"🌎🚀\"").scan()
        );
    }

    #[test]
    fn valid_escape_sequence() {
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::String, 1, (0, 19), (1, 20)),
                Token::new(TokenType::Eof, 1, (19, 19), (20, 20))
            ]),
            Scanner::new(r#""hello\u0020world!""#).scan(),
        );

        assert_eq!(
            Ok(vec![
                Token::new(TokenType::String, 1, (0, 14), (1, 15)),
                Token::new(TokenType::Eof, 1, (14, 14), (15, 15))
            ]),
            Scanner::new(r#""\uD83D\uDE00""#).scan(),
        );

        assert_eq!(
            Ok(vec![
                Token::new(TokenType::String, 1, (0, 10), (1, 11)),
                Token::new(TokenType::Eof, 1, (10, 10), (11, 11))
            ]),
            Scanner::new(r#""\\\uaaaa""#).scan(),
        );
    }

    #[test]
    fn invalid_escape_sequence() {
        assert_eq!(true, Scanner::new(r#""hello\\\world!""#).scan().is_err(),);
        assert_eq!(true, Scanner::new(r#""\t\e bad""#).scan().is_err(),);
        assert_eq!(true, Scanner::new(r#""\u01AG""#).scan().is_err(),);
    }

    #[test]
    fn scan_valid_strings() {
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::String, 1, (0, 15), (1, 16)),
                Token::new(TokenType::Eof, 1, (15, 15), (16, 16))
            ]),
            Scanner::new("\"hello, world!\"").scan()
        );

        assert_eq!(
            Ok(vec![
                Token::new(TokenType::String, 1, (0, 2), (1, 3)),
                Token::new(TokenType::Eof, 1, (2, 2), (3, 3))
            ]),
            Scanner::new("\"\"").scan()
        );
    }

    #[test]
    fn scan_invalid_words() {
        assert_eq!(true, Scanner::new("hello").scan().is_err());

        assert_eq!(true, Scanner::new("unknown").scan().is_err());
    }

    #[test]
    fn scan_valid_keywords() {
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::True, 1, (0, 4), (1, 5)),
                Token::new(TokenType::Eof, 1, (4, 4), (5, 5))
            ]),
            Scanner::new("true").scan()
        );

        assert_eq!(
            Ok(vec![
                Token::new(TokenType::False, 1, (0, 5), (1, 6)),
                Token::new(TokenType::Eof, 1, (5, 5), (6, 6))
            ]),
            Scanner::new("false").scan()
        );

        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Null, 1, (0, 4), (1, 5)),
                Token::new(TokenType::Eof, 1, (4, 4), (5, 5))
            ]),
            Scanner::new("null").scan()
        );
    }

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
                Token::new(TokenType::Eof, 1, (6, 6), (7, 7)),
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

        assert_eq!(true, s.scan().is_err())
    }
}
