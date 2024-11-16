use crate::token_type::TokenType;

#[derive(Debug, PartialEq)]
pub struct Token<'source> {
    token_type: TokenType,
    literal: &'source str,
}

impl<'source> Token<'source> {
    pub fn new(token_type: TokenType, literal: &'source str) -> Self {
        Self {
            token_type,
            literal,
        }
    }
}

#[cfg(test)]
mod token_tests {
    use super::*;

    #[test]
    fn create_new_token() {
        assert_eq!(
            Token::new(TokenType::String, "{}"),
            Token::new(TokenType::String, "{}")
        );
    }

    #[test]
    fn store_slice_of_input() {
        let source = String::from("{}");

        assert_eq!(
            Token::new(TokenType::String, "{}"),
            Token::new(TokenType::String, &source[0..=1])
        );
    }

    #[test]
    fn retrieve_literal_from_token() {
        let source = "[]";

        assert_eq!(
            "[",
            Token::new(TokenType::LeftBracket, &source[0..1]).literal
        );
    }
}
