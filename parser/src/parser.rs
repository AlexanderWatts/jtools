use std::cell::Cell;

use ast::node::Node;
use token::{token::Token, token_type::TokenType};

use crate::parser_error::ParserError;

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

    pub fn parse(&self) -> Result<Node, ParserError> {
        self.parse_literal()
    }

    fn parse_property(&self) -> Result<(&str, Node), ParserError> {
        let key_literal = self.next_or_error(TokenType::String)?.literal;
        let key = Node::Literal(key_literal);

        let _colon = self.next_or_error(TokenType::Colon)?;

        let value = self.parse_literal()?;

        Ok((key_literal, Node::Property(Box::new(key), Box::new(value))))
    }

    fn parse_array(&self) -> Result<Node, ParserError> {
        self.next_or_error(TokenType::LeftBracket)?;

        let mut values = vec![];

        if matches!(self.peek(), Some(Token { token_type, .. }) if *token_type != TokenType::RightBracket)
        {
            values.push(self.parse_literal()?);

            while matches!(self.peek(), Some(Token { token_type, .. }) if *token_type == TokenType::Comma)
            {
                self.next();
                values.push(self.parse_literal()?);
            }
        }

        self.next_or_error(TokenType::RightBracket)?;

        Ok(Node::Array(values))
    }

    fn parse_literal(&self) -> Result<Node, ParserError> {
        if let Some(Token {
            token_type,
            literal,
            ..
        }) = self.peek()
        {
            match token_type {
                TokenType::Null
                | TokenType::String
                | TokenType::Number
                | TokenType::True
                | TokenType::False => {
                    let node = Ok(Node::Literal(literal));
                    self.next();
                    return node;
                }
                TokenType::LeftBracket => return self.parse_array(),
                _ => return Err(ParserError::UnexpectedToken),
            };
        }

        Err(ParserError::UnexpectedToken)
    }

    fn next_or_error(&self, expected_token_type: TokenType) -> Result<&Token, ParserError> {
        if let Some(token) = self.peek() {
            if expected_token_type == token.token_type {
                self.next();
                return Ok(token);
            }
        }

        return Err(ParserError::UnexpectedToken);
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
    fn parse_valid_property() {
        let p = Parser::new(vec![
            Token::new(TokenType::String, "\"type\"", 1, 1, 7),
            Token::new(TokenType::Colon, ":", 1, 7, 8),
            Token::new(TokenType::String, "\"Hello, World!\"", 1, 8, 23),
        ]);

        assert_eq!(
            Ok((
                "\"type\"",
                Node::Property(
                    Box::new(Node::Literal("\"type\""),),
                    Box::new(Node::Literal("\"Hello, World!\""))
                )
            )),
            p.parse_property()
        );
    }

    #[test]
    fn error_invalid_array() {
        let p = Parser::new(vec![
            Token::new(TokenType::LeftBracket, "[", 1, 1, 2),
            Token::new(TokenType::Comma, ",", 1, 2, 3),
            Token::new(TokenType::RightBracket, "]", 1, 3, 4),
        ]);

        assert_eq!(Err(ParserError::UnexpectedToken), p.parse_array());
    }

    #[test]
    fn parse_empty_array() {
        let p = Parser::new(vec![
            Token::new(TokenType::LeftBracket, "[", 1, 1, 2),
            Token::new(TokenType::RightBracket, "]", 1, 2, 3),
        ]);

        assert_eq!(Ok(Node::Array(vec![])), p.parse_array());
    }

    #[test]
    fn parse_valid_array() {
        let p = Parser::new(vec![
            Token::new(TokenType::LeftBracket, "[", 1, 1, 2),
            Token::new(TokenType::True, "true", 1, 2, 6),
            Token::new(TokenType::Comma, ",", 1, 6, 7),
            Token::new(TokenType::True, "false", 1, 7, 12),
            Token::new(TokenType::RightBracket, "]", 1, 12, 13),
        ]);

        assert_eq!(
            Ok(Node::Array(vec![
                Node::Literal("true"),
                Node::Literal("false")
            ])),
            p.parse_array()
        );
    }

    #[test]
    fn parse_null_literal() {
        let p = Parser::new(vec![Token::new(TokenType::Null, "null", 1, 1, 5)]);
        assert_eq!(Ok(Node::Literal("null")), p.parse_literal());
    }

    #[test]
    fn parse_string_literal() {
        let p = Parser::new(vec![Token::new(TokenType::String, "\"hi\"", 1, 1, 5)]);
        assert_eq!(Ok(Node::Literal("\"hi\"")), p.parse_literal());
    }
    #[test]
    fn parse_number_literal() {
        let p = Parser::new(vec![Token::new(TokenType::String, "26", 1, 1, 4)]);
        assert_eq!(Ok(Node::Literal("26")), p.parse_literal());
    }

    #[test]
    fn parse_true_literal() {
        let p = Parser::new(vec![Token::new(TokenType::True, "true", 1, 1, 5)]);
        assert_eq!(Ok(Node::Literal("true")), p.parse_literal());
    }

    #[test]
    fn parse_false_literal() {
        let p = Parser::new(vec![Token::new(TokenType::False, "false", 1, 1, 5)]);
        assert_eq!(Ok(Node::Literal("false")), p.parse_literal());
    }

    #[test]
    fn consume_next_token_when_expected() {
        let p = Parser::new(vec![Token::new(TokenType::True, "true", 1, 1, 5)]);

        assert_eq!(
            Ok(&Token::new(TokenType::True, "true", 1, 1, 5)),
            p.next_or_error(TokenType::True)
        );
    }

    #[test]
    fn error_on_unexpected_token() {
        let p = Parser::new(vec![Token::new(TokenType::True, "true", 1, 1, 5)]);

        assert_eq!(
            Err(ParserError::UnexpectedToken),
            p.next_or_error(TokenType::LeftBrace)
        );
    }

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
