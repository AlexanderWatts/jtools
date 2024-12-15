use std::fmt::{format, Display};

use crate::token_type::TokenType;

/// Token wrapper
///
/// ## Description
///
/// A token is a wrapper around some word found by the scanner that provides it with more meaning
/// necessary for subsequent stages such as parsing and formatting. Tokens are created and owned by
/// the scanner which iterates over an input instantiating tokens as it goes along.
///
/// ## Examples
///
/// ```rust
/// use token::{token_type::TokenType, token::Token};
///
/// let source = "[ true, false ]";
///
/// let token_left_bracket = Token::new(TokenType::LeftBracket, 1, (0, 1), (1, 2));
/// let token_true = Token::new(TokenType::True, 1, (2, 6), (3, 7));
/// ```
#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line_number: usize,
    pub indices: (usize, usize),
    pub column_indices: (usize, usize),
}

impl Token {
    pub fn new(
        token_type: TokenType,
        line_number: usize,
        indices: (usize, usize),
        column_indices: (usize, usize),
    ) -> Self {
        Self {
            token_type,
            line_number,
            indices,
            column_indices,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Token {
            token_type,
            line_number,
            indices,
            column_indices,
        } = self;

        write!(f,
            "Token {{ token_type: {}, line_number: {}, indices: ({}, {}), column_indices: ({}, {}) }}",
            token_type, line_number, indices.0, indices.1, column_indices.0, column_indices.1)
    }
}

#[cfg(test)]
mod token_tests {
    use super::*;

    #[test]
    fn test() {
        let t = Token::new(TokenType::LeftBracket, 1, (0, 1), (1, 2));

        assert_eq!(
            "Token { token_type: LeftBracket, line_number: 1, indices: (0, 1), column_indices: (1, 2) }",
            t.to_string());
    }

    #[test]
    fn use_token_indices_to_get_string_slice() {
        let source = "[]";

        let (start, end) = Token::new(TokenType::LeftBracket, 1, (0, 1), (1, 2)).indices;

        assert_eq!("[", &source[start..end],);
    }

    #[test]
    fn create_new_token() {
        assert_eq!(
            Token::new(TokenType::String, 1, (0, 7), (1, 8)),
            Token::new(TokenType::String, 1, (0, 7), (1, 8))
        );
    }
}
