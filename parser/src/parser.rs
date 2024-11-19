use std::cell::Cell;

use token::token::Token;

#[derive(Debug, PartialEq)]
pub struct Parser<'source> {
    current: Cell<usize>,
    tokens: Vec<Token<'source>>,
}

impl<'source> Parser<'source> {
    pub fn new(tokens: Vec<Token<'source>>) -> Self {
        Self {
            current: Cell::new(0),
            tokens,
        }
    }

    fn next(&self) -> Option<&Token> {
        let current = self.tokens.get(self.current.get());

        if current.is_some() {
            self.current.set(self.current.get() + 1);
        }

        current
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current.get())
    }
}

#[cfg(test)]
mod parser_tests {
    use token::token_type::TokenType;

    use super::*;

    #[test]
    fn consume_next_until_end() {
        let p = Parser::new(vec![
            Token::new(TokenType::True, "true", 1, 1, 5),
            Token::new(TokenType::False, "false", 1, 5, 11),
        ]);

        p.next();
        p.next();

        assert_eq!(None, p.next());
    }

    #[test]
    fn next_is_some() {
        let p = Parser::new(vec![Token::new(TokenType::True, "true", 1, 1, 5)]);
        assert_eq!(
            Some(&Token::new(TokenType::True, "true", 1, 1, 5)),
            p.next()
        );
        assert_eq!(1, p.current.get());
        assert_eq!(None, p.next());
    }

    #[test]
    fn next_is_none() {
        let p = Parser::new(vec![]);
        assert_eq!(None, p.next());
        assert_eq!(0, p.current.get());
    }

    #[test]
    fn peek_is_none() {
        let p = Parser::new(vec![]);
        assert_eq!(None, p.peek());
    }

    #[test]
    fn peek_is_some() {
        let p = Parser::new(vec![Token::new(TokenType::True, "true", 1, 1, 5)]);

        assert_eq!(
            Some(&Token::new(TokenType::True, "true", 1, 1, 5)),
            p.peek()
        );
    }

    #[test]
    fn create_new_parser() {
        let p = Parser::new(vec![Token::new(TokenType::True, "true", 1, 1, 5)]);

        assert_eq!(
            Parser {
                current: Cell::new(0),
                tokens: vec![Token::new(TokenType::True, "true", 1, 1, 5)]
            },
            p
        );
    }
}
