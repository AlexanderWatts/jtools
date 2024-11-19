use token::token::Token;

#[derive(Debug, PartialEq)]
pub struct Parser<'source> {
    current: usize,
    tokens: Vec<Token<'source>>,
}

impl<'source> Parser<'source> {
    pub fn new(tokens: Vec<Token<'source>>) -> Self {
        Self { current: 0, tokens }
    }
}

#[cfg(test)]
mod parser_tests {
    use token::token_type::TokenType;

    use super::*;

    #[test]
    fn create_new_parser() {
        let p = Parser::new(vec![Token::new(TokenType::True, "true", 1, 1, 5)]);

        assert_eq!(
            Parser {
                current: 0,
                tokens: vec![Token::new(TokenType::True, "true", 1, 1, 5)]
            },
            p
        );
    }
}
