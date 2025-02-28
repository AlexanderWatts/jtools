use crate::{scanner::Scanner, scanner_error::ScannerError};
use std::cell::{Ref, RefCell};
use token::token::Token;

/// Token buffer
///
/// ## Description
///
/// The token buffer provides a one-token lookahead, where the parser can peek at the current token
/// without consuming it, and get the next token. Importantly, unlike Peekable, it does not require
/// a mutable reference to use, so it avoids situations where the parser needs to borrow mutably
/// more than once, specifically when parsing arrays and objects.
///
#[derive(Debug)]
pub struct TokenBuffer<'source> {
    pub scanner: Scanner<'source>,
    pub current_token: RefCell<Result<Token, ScannerError>>,
}

impl<'source> TokenBuffer<'source> {
    pub fn new(scanner: Scanner<'source>) -> Self {
        let initial_token = scanner.get_token();

        Self {
            scanner,
            current_token: RefCell::new(initial_token),
        }
    }

    pub fn get_token(&self) -> Result<Token, ScannerError> {
        self.current_token
            .replace_with(|_| self.scanner.get_token())
    }

    pub fn peek_token(&self) -> Ref<'_, Result<Token, ScannerError>> {
        self.current_token.borrow()
    }
}

#[cfg(test)]
mod token_buffer_tests {
    use token::token_type::TokenType;

    use super::*;

    #[test]
    pub fn token_buffer_get_and_peek() {
        let token_buffer = TokenBuffer::new(Scanner::new("{}"));

        assert_eq!(
            TokenType::LeftBrace,
            token_buffer.get_token().unwrap().token_type
        );
        assert_eq!(
            TokenType::RightBrace,
            token_buffer.peek_token().as_ref().unwrap().token_type
        );
        assert_eq!(
            TokenType::RightBrace,
            token_buffer.peek_token().as_ref().unwrap().token_type
        );
        assert_eq!(
            TokenType::RightBrace,
            token_buffer.get_token().unwrap().token_type
        );
        assert_eq!(TokenType::Eof, token_buffer.get_token().unwrap().token_type);
    }
}
