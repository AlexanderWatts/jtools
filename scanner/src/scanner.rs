use std::{iter::Peekable, str::CharIndices};

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
///     ],),
///     tokens
/// )
/// ```
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
            ' ' | '\t' | '\r' => Ok(None),
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
            '\"' => self.scan_string(),
            '0' => match self.advance_if(|&(_, char)| char != '0') {
                Some(_) => self.scan_number(),
                None => Err(ScannerError::LeadingZeros),
            },
            '-' => self.scan_number(),
            _ => {
                if char.is_ascii_alphabetic() {
                    self.scan_keyword()
                } else if char.is_ascii_digit() {
                    self.scan_number()
                } else {
                    Err(ScannerError::UnknownCharacter {
                        source: self.source.to_string(),
                        indices: (self.start, self.current),
                        display_position: (self.line, self.column_start, self.column_end),
                        error_message: String::from("Unknown character"),
                        preview_size: 20,
                    })
                }
            }
        };

        if char != '\n' {
            self.column_start = self.column_end;
        }

        res
    }

    fn scan_number(&mut self) -> Result<Option<Token>, ScannerError> {
        while let Some(_) = self.advance_if(|&(_, char)| char.is_ascii_digit()) {}

        if self.advance_if(|&(_, char)| char == '.').is_some() {
            match self.chars.peek() {
                Some(&(_, char)) if !char.is_ascii_digit() => {
                    Err(ScannerError::UnterminatedFractionalNumber)?
                }
                None => Err(ScannerError::UnterminatedFractionalNumber)?,
                _ => {}
            }

            while let Some(_) = self.advance_if(|&(_, char)| char.is_ascii_digit()) {}
        }

        if self
            .advance_if(|&(_, char)| char == 'e' || char == 'E')
            .is_some()
        {
            if self
                .advance_if(|&(_, char)| char == '+' || char == '-')
                .is_some()
            {}

            match self.chars.peek() {
                Some(&(_, char)) if !char.is_ascii_digit() => Err(ScannerError::InvalidExponent)?,
                None => Err(ScannerError::InvalidExponent)?,
                _ => {}
            }

            while let Some(_) = self.advance_if(|&(_, char)| char.is_ascii_digit()) {}
        }

        match &self.source[self.start..self.current].parse::<f64>() {
            Ok(_) => Ok(Some(self.create_token(TokenType::Number))),
            Err(_) => Err(ScannerError::InvalidNumber),
        }
    }

    fn scan_string(&mut self) -> Result<Option<Token>, ScannerError> {
        while let Some(_) = self.advance_if(|&(_, char)| char != '\"') {}

        if self.chars.peek().is_none() {
            Err(ScannerError::UnterminatedString)?
        }

        self.advance();

        Ok(Some(self.create_token(TokenType::String)))
    }

    fn scan_keyword(&mut self) -> Result<Option<Token>, ScannerError> {
        while let Some(_) = self.advance_if(|&(_, char)| char.is_ascii_alphabetic()) {}

        let result = match &self.source[self.start..self.current] {
            "true" => self.create_token(TokenType::True),
            "false" => self.create_token(TokenType::False),
            "null" => self.create_token(TokenType::Null),
            _ => Err(ScannerError::UnknownLiteral)?,
        };

        Ok(Some(result))
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
    fn ignore_spacing_and_maintain_display_column() {
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::LeftBracket, 1, (0, 1), (1, 2)),
                Token::new(TokenType::RightBracket, 1, (2, 3), (3, 4))
            ]),
            Scanner::new("[ ]").scan()
        );
    }

    #[test]
    fn invalid_exponents() {
        assert_eq!(
            Err(ScannerError::InvalidExponent),
            Scanner::new("27e").scan()
        );

        assert_eq!(
            Err(ScannerError::InvalidExponent),
            Scanner::new("92.3eE").scan()
        );

        assert_eq!(
            Err(ScannerError::InvalidExponent),
            Scanner::new("83e-").scan()
        );

        assert_eq!(
            Err(ScannerError::InvalidExponent),
            Scanner::new("83E+").scan()
        );
    }

    #[test]
    fn valid_exponents() {
        assert_eq!(
            Ok(vec![Token::new(TokenType::Number, 1, (0, 5), (1, 6))]),
            Scanner::new("360e2").scan()
        );

        assert_eq!(
            Ok(vec![Token::new(TokenType::Number, 1, (0, 4), (1, 5))]),
            Scanner::new("29E8").scan()
        );

        assert_eq!(
            Ok(vec![Token::new(TokenType::Number, 1, (0, 7), (1, 8))]),
            Scanner::new("29e+100").scan()
        );

        assert_eq!(
            Ok(vec![Token::new(TokenType::Number, 1, (0, 5), (1, 6))]),
            Scanner::new("29e-2").scan()
        );
    }

    #[test]
    fn do_not_allow_leading_zeros_in_number() {
        assert_eq!(
            Err(ScannerError::LeadingZeros),
            Scanner::new("000.23432").scan()
        );
    }

    #[test]
    fn scan_valid_numbers() {
        assert_eq!(
            Ok(vec![Token::new(TokenType::Number, 1, (0, 3), (1, 4))]),
            Scanner::new("360").scan()
        );

        assert_eq!(
            Ok(vec![Token::new(TokenType::Number, 1, (0, 7), (1, 8))]),
            Scanner::new("360.360").scan()
        );

        assert_eq!(
            Ok(vec![Token::new(TokenType::Number, 1, (0, 5), (1, 6))]),
            Scanner::new("-1066").scan()
        );
    }

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
