use crate::scanner_error::ScannerError;
use core::f64;
use error_preview::error_preview::ErrorPreview;
use std::cell::Cell;
use token::{token::Token, token_type::TokenType};

/// On-demand lexical analyser/scanner
///
/// ## Description
///
/// When required by the parser (on-demand), the scanner evaluates one token at a time. It is used
/// with the token buffer to allow for a one-token lookahead. The scanner returns an error if a
/// sequence of characters is not valid JSON. For the parser to return a scanner error, it is
/// converted to a parser error using the From trait.
///
/// ## Examples
///
/// ```
/// use scanner::scanner::Scanner;
/// use token::{token_type::TokenType, token::Token};
///
/// let scanner = Scanner::new("[true, 2]");
///
/// assert_eq!(
///     Ok(Token::new(TokenType::LeftBracket, 1, (0, 1), (1, 2))),
///     scanner.get_token()
/// );
///
/// assert_eq!(
///     Ok(Token::new(TokenType::True, 1, (1, 5), (2, 6))),
///     scanner.get_token()
/// );
///
/// assert_eq!(
///     Ok(Token::new(TokenType::Comma, 1, (5, 6), (6, 7))),
///     scanner.get_token()
/// );
///
/// assert_eq!(
///     Ok(Token::new(TokenType::Number, 1, (7, 8), (8, 9))),
///     scanner.get_token()
/// );
///
/// assert_eq!(
///     Ok(Token::new(TokenType::RightBracket, 1, (8, 9), (9, 10))),
///     scanner.get_token()
/// );
/// ```
#[derive(Debug)]
pub struct Scanner<'source> {
    source: &'source str,
    characters: Vec<char>,
    line: Cell<usize>,
    current_position: Cell<usize>,
    start_index: Cell<usize>,
    end_index: Cell<usize>,
    column_start: Cell<usize>,
    column_end: Cell<usize>,
}

impl<'source> Scanner<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            characters: source.chars().collect(),
            line: Cell::new(1),
            current_position: Cell::new(0),
            start_index: Cell::new(0),
            end_index: Cell::new(0),
            column_start: Cell::new(0),
            column_end: Cell::new(1),
        }
    }

    pub fn get_token(&self) -> Result<Token, ScannerError> {
        let token_type = self.eval()?;

        Ok(Token::new(
            token_type,
            self.line.get(),
            (self.start_index.get(), self.end_index.get()),
            (self.column_start.get(), self.column_end.get()),
        ))
    }

    fn eval(&self) -> Result<TokenType, ScannerError> {
        while let Some(' ' | '\r' | '\t' | '\n') = self.peek() {
            match self.next() {
                Some('\n') => {
                    self.line.set(self.line.get() + 1);
                    self.column_start.set(0);
                    self.column_end.set(1);
                }
                _ => {}
            }
        }

        self.start_index.set(self.end_index.get());
        self.column_start.set(self.column_end.get());

        match self.next() {
            Some(character) => match character {
                '{' => Ok(TokenType::LeftBrace),
                '}' => Ok(TokenType::RightBrace),
                '[' => Ok(TokenType::LeftBracket),
                ']' => Ok(TokenType::RightBracket),
                ':' => Ok(TokenType::Colon),
                ',' => Ok(TokenType::Comma),
                '0' => match self.peek() {
                    Some('0'..='9') => Err(ScannerError::LeadingZeros {
                        error: self.error_preview(None, None),
                    }),
                    _ => self.number(),
                },
                '1'..='9' => self.number(),
                '-' => self.number(),
                '\"' => self.string(),
                'a'..='z' => self.keyword(),
                _ => Err(ScannerError::UnknownCharacter {
                    error: self.error_preview(None, None),
                })?,
            },
            None => Ok(TokenType::Eof),
        }
    }

    fn keyword(&self) -> Result<TokenType, ScannerError> {
        while let Some('a'..='z') = self.peek() {
            self.next();
        }

        match self
            .source
            .get(self.start_index.get()..self.end_index.get())
        {
            Some("true") => Ok(TokenType::True),
            Some("false") => Ok(TokenType::False),
            Some("null") => Ok(TokenType::Null),
            _ => Err(ScannerError::UnknownLiteral {
                error: self.error_preview(None, None),
            })?,
        }
    }

    fn string(&self) -> Result<TokenType, ScannerError> {
        while matches!(self.peek(), Some(char) if *char != '\"') {
            self.next();

            if let Some('\n') = self.peek() {
                Err(ScannerError::UnterminatedString {
                    error: self
                        .error_preview(Some(self.end_index.get()), Some(self.column_end.get())),
                })?
            }

            if let Some('\\') = self.peek() {
                self.next();

                match self.peek() {
                    Some('u') => {
                        self.next();

                        for _ in 0..4 {
                            if let Some('0'..='9' | 'a'..='f' | 'A'..='F') = self.peek() {
                                self.next();
                            } else {
                                Err(ScannerError::InvalidUnicodeSequence {
                                    error: self.error_preview(
                                        Some(self.end_index.get()),
                                        Some(self.column_end.get()),
                                    ),
                                })?
                            }
                        }
                    }
                    Some('\"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't') => {
                        self.next();
                    }
                    _ => Err(ScannerError::InvalidEscapeSequence {
                        error: self
                            .error_preview(Some(self.end_index.get()), Some(self.column_end.get())),
                    })?,
                }
            }
        }

        if self.peek().is_none() {
            Err(ScannerError::UnterminatedString {
                error: self.error_preview(Some(self.end_index.get()), Some(self.column_end.get())),
            })?
        }

        self.next();

        Ok(TokenType::String)
    }

    fn number(&self) -> Result<TokenType, ScannerError> {
        while let Some('0'..='9') = self.peek() {
            self.next();
        }

        if let Some('.') = self.peek() {
            self.next();

            if !matches!(self.peek(), Some('0'..='9')) {
                Err(ScannerError::UnterminatedFractionalNumber {
                    error: self
                        .error_preview(Some(self.end_index.get()), Some(self.column_end.get())),
                })?
            }

            while let Some('0'..='9') = self.peek() {
                self.next();
            }
        }

        if let Some('e' | 'E') = self.peek() {
            self.next();

            if let Some('+' | '-') = self.peek() {
                self.next();
            }

            if !matches!(self.peek(), Some('0'..='9')) {
                Err(ScannerError::InvalidExponent {
                    error: self
                        .error_preview(Some(self.end_index.get()), Some(self.column_end.get())),
                })?
            }

            while let Some('0'..='9') = self.peek() {
                self.next();
            }
        }

        match self
            .source
            .get(self.start_index.get()..self.end_index.get())
        {
            Some(number) => match number.parse::<f64>() {
                Ok(number) if number.is_finite() => Ok(TokenType::Number),
                _ => Err(ScannerError::InvalidNumber {
                    error: self.error_preview(None, None),
                })?,
            },
            None => Err(ScannerError::InvalidNumber {
                error: self.error_preview(None, None),
            })?,
        }
    }

    fn error_preview(&self, start: Option<usize>, column_start: Option<usize>) -> String {
        ErrorPreview.preview(
            self.source,
            start.unwrap_or(self.start_index.get()),
            column_start.unwrap_or(self.column_start.get()),
            self.line.get(),
        )
    }

    fn peek(&self) -> Option<&char> {
        self.characters.get(self.current_position.get())
    }

    fn next(&self) -> Option<&char> {
        let next = self.characters.get(self.current_position.get());

        match next {
            Some(char) => {
                self.current_position.set(self.current_position.get() + 1);
                self.end_index.set(self.end_index.get() + char.len_utf8());
                self.column_end.set(self.column_end.get() + 1);
            }
            _ => {}
        };

        next
    }
}

#[cfg(test)]
mod scanner_tests {
    use super::*;

    #[test]
    fn evaluate_number_exponents() {
        let scanner = Scanner::new("[true, 2]");

        assert_eq!(
            TokenType::Number,
            Scanner::new("42e+100").get_token().unwrap().token_type
        );

        assert_eq!(
            TokenType::Number,
            Scanner::new("192E2").get_token().unwrap().token_type
        );

        assert_eq!(
            TokenType::Number,
            Scanner::new("192E-10").get_token().unwrap().token_type
        );

        assert_eq!(
            TokenType::Number,
            Scanner::new("10e001").get_token().unwrap().token_type
        );
    }

    #[test]
    fn evaluate_numbers() {
        assert_eq!(
            TokenType::Number,
            Scanner::new("99").get_token().unwrap().token_type
        );

        assert_eq!(
            TokenType::Number,
            Scanner::new("1232hello").get_token().unwrap().token_type
        );
    }

    #[test]
    fn evaluate_strings() {
        let scanner = Scanner::new("\"hello\"");
        assert_eq!(Ok(TokenType::String), scanner.eval());
    }

    #[test]
    fn evaluate_keywords() {
        let scanner = Scanner::new("true false null");

        assert_eq!(Ok(TokenType::True), scanner.eval());
        assert_eq!(Ok(TokenType::False), scanner.eval());
        assert_eq!(Ok(TokenType::Null), scanner.eval());
    }

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

        assert_eq!(Cell::new(0), scanner.start_index);
        let _ = scanner.eval();

        let _ = scanner.eval();
        assert_eq!(Cell::new(1), scanner.start_index);
    }

    #[test]
    fn get_source_from_indices() {
        let scanner = Scanner::new("\"Hi🌎✨!\"");

        let (start, end) = scanner.get_token().unwrap().indices;
        assert_eq!("\"Hi🌎✨!\"", scanner.source.get(start..end).unwrap());

        let scanner = Scanner::new("\"name\": \"Afonso Vilarchán\",");

        let (start, end) = scanner.get_token().unwrap().indices;
        assert_eq!("\"name\"", scanner.source.get(start..end).unwrap());

        let (start, end) = scanner.get_token().unwrap().indices;
        assert_eq!(":", scanner.source.get(start..end).unwrap());

        let (start, end) = scanner.get_token().unwrap().indices;
        assert_eq!(
            "\"Afonso Vilarchán\"",
            scanner.source.get(start..end).unwrap()
        );
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
}
