use crate::token_type::TokenType;

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Self { token_type }
    }
}

#[cfg(test)]
mod token_tests {
    use super::*;

    #[test]
    fn create_new_token() {
        let t = Token::new(TokenType::String);

        assert_eq!(Token::new(TokenType::String), t);
    }
}
