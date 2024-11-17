use crate::token_type::TokenType;

#[derive(Debug, PartialEq)]
pub struct Token<'source> {
    token_type: TokenType,
    literal: &'source str,
    line_number: usize,
}

impl<'source> Token<'source> {
    pub fn new(token_type: TokenType, literal: &'source str, line_number: usize) -> Self {
        Self {
            token_type,
            literal,
            line_number,
        }
    }
}

#[cfg(test)]
mod token_tests {
    use super::*;

    #[test]
    fn retrieve_literal_from_token() {
        let source = "[]";

        assert_eq!(
            "[",
            Token::new(TokenType::LeftBracket, &source[0..1], 1).literal
        );
    }

    #[test]
    fn store_slice_of_input() {
        let source = String::from("}");

        assert_eq!(
            Token::new(TokenType::RightBrace, "}", 1),
            Token::new(TokenType::RightBrace, &source[0..1], 1)
        );
    }

    #[test]
    fn create_new_token() {
        assert_eq!(
            Token::new(TokenType::String, "\"hello\"", 1),
            Token::new(TokenType::String, "\"hello\"", 1)
        );
    }
}
