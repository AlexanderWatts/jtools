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
                '\"' => {
                    self.scan_string()?;
                }
                _ => {
                    if character.is_alphabetic() {
                        self.scan_alphabetic();
                    } else if character.is_ascii_digit() {
                        self.scan_number()?;
                    } else {
                        Err(LexerError::UnknownCharacter)?
                    }
                }
            }
        }

        Ok(tokens)
    }

    fn scan_number(&mut self) -> Result<&'source str, LexerError> {
        while let Some((character_index, character)) = self
            .chars
            .next_if(|&(_, character)| character.is_ascii_digit())
        {
            self.current = character_index + character.len_utf8();
        }

        if let Some((character_index, character)) =
            self.chars.next_if(|&(_, character)| character == '.')
        {
            self.current = character_index + character.len_utf8();

            match self.chars.peek() {
                Some((_, character)) if !character.is_ascii_digit() => {
                    Err(LexerError::UnterminatedFractionalNumber)?
                }
                None => Err(LexerError::UnterminatedFractionalNumber)?,
                _ => {}
            }

            while let Some((character_index, character)) = self
                .chars
                .next_if(|&(_, character)| character.is_ascii_digit())
            {
                self.current = character_index + character.len_utf8();
            }
        }

        if let Some((character_index, character)) = self
            .chars
            .next_if(|&(_, character)| character == 'e' || character == 'E')
        {
            self.current = character_index + character.len_utf8();

            if let Some((character_index, character)) = self
                .chars
                .next_if(|&(_, character)| character == '+' || character == '-')
            {
                self.current = character_index + character.len_utf8();
            }

            if let Some((character_index, character)) = self
                .chars
                .next_if(|&(_, character)| character.is_ascii_digit())
            {
                self.current = character_index + character.len_utf8();

                while let Some((character_index, character)) = self
                    .chars
                    .next_if(|&(_, character)| character.is_ascii_digit())
                {
                    self.current = character_index + character.len_utf8();
                }
            } else {
                Err(LexerError::InvalidExponent)?
            }
        }

        Ok(&self.source[self.start..self.current])
    }

    fn scan_string(&mut self) -> Result<&'source str, LexerError> {
        while let Some((character_index, character)) =
            self.chars.next_if(|&(_, character)| character != '\"')
        {
            self.current = character_index + character.len_utf8();
        }

        match self.chars.next() {
            Some((character_index, character)) => {
                self.current = character_index + character.len_utf8();

                Ok(&self.source[self.start..self.current])
            }
            None => Err(LexerError::UnterminatedString)?,
        }
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
    fn scan_valid_number_with_exponent() {
        let mut l = Lexer::new("0123456789e100");
        assert_eq!(Ok("0123456789e100"), l.scan_number());

        let mut l = Lexer::new("0123456789E4");
        assert_eq!(Ok("0123456789E4"), l.scan_number());

        let mut l = Lexer::new("0123456789e-20");
        assert_eq!(Ok("0123456789e-20"), l.scan_number());

        let mut l = Lexer::new("0123456789e+3");
        assert_eq!(Ok("0123456789e+3"), l.scan_number());
    }

    #[test]
    fn scan_invalid_number_with_exponent() {
        let mut l = Lexer::new("0123456789e");
        assert_eq!(Err(LexerError::InvalidExponent), l.scan_number());

        let mut l = Lexer::new("0123456789e+");
        assert_eq!(Err(LexerError::InvalidExponent), l.scan_number());

        let mut l = Lexer::new("0123456789e-nope");
        assert_eq!(Err(LexerError::InvalidExponent), l.scan_number());

        let mut l = Lexer::new("0123456789Eee");
        assert_eq!(Err(LexerError::InvalidExponent), l.scan_number());
    }

    #[test]
    fn scan_invalid_fractional_number() {
        let mut l = Lexer::new("0123456789.nope");

        assert_eq!(
            Err(LexerError::UnterminatedFractionalNumber),
            l.scan_number()
        );
    }

    #[test]
    fn scan_unterminated_fractional_number() {
        let mut l = Lexer::new("0123456789.");

        assert_eq!(
            Err(LexerError::UnterminatedFractionalNumber),
            l.scan_number()
        );
    }

    #[test]
    fn scan_fractional_number() {
        let mut l = Lexer::new("0123456789.0123456789");

        assert_eq!(Ok("0123456789.0123456789"), l.scan_number());
    }

    #[test]
    fn scan_number() {
        let mut l = Lexer::new("0123456789");

        assert_eq!(Ok("0123456789"), l.scan_number());
    }

    #[test]
    fn expect_unterminated_string() {
        let mut l = Lexer::new("terminator");

        assert_eq!(Err(LexerError::UnterminatedString), l.scan_string());
    }

    #[test]
    fn scan_string() {
        let mut l = Lexer::new("🌎Hello, World🌎\"");

        assert_eq!(Ok("🌎Hello, World🌎\""), l.scan_string());
    }

    #[test]
    fn scan_alphabetical() {
        let mut l = Lexer::new("true");

        assert_eq!("true", l.scan_alphabetic());
    }
}
