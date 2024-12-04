use std::cell::Cell;

use ast::node::Node;
use error_display::error_display::{Client, ErrorDisplay};
use token::{token::Token, token_type::TokenType};

use crate::{parser_error::ParserError, property_map::PropertyMap};

/// Recursive descent parser
///
/// ## Description
///
/// ```text
///                         |--------|
///  Token | Tokens -input->| PARSER |-output-> AST | Error
///                         |--------|
/// ```
///
/// ## Parser design
///
/// The parser was built from the following CFG (Context Free Grammar):
///
/// ```text
/// json := literal ;
/// object := "{" ( property ( "," property )* )* "}" ;
/// property := string ":" literal ;
/// array := "[" ( literal ( "," literal )* )* "]" ;
/// literal := string | number | "true" | "false" | "null" | object | array ;
/// ```
///
/// ## Examples
/// ```
/// use parser::parser::Parser;
/// use token::{token_type::TokenType, token::Token};
/// use ast::node::Node;
///
/// let source = "{\"animal\":\"dog\"}";
///
/// let p = Parser::new(
///     source,
///     vec![
///         Token::new(TokenType::LeftBrace, 1, (0, 1), (1, 2)),
///         Token::new(TokenType::String, 1, (1, 9), (2, 10)),
///         Token::new(TokenType::Colon, 1, (9, 10), (10, 11)),
///         Token::new(TokenType::String, 1, (10, 15), (11, 16)),
///         Token::new(TokenType::RightBrace, 1, (15, 16), (16, 17)),
///     ],
/// );
///
/// assert_eq!(
///     Ok(Node::Object(vec![Node::Property(
///         Box::new(Node::Literal("\"animal\"",)),
///         Box::new(Node::Literal("\"dog\"",)),
///     ),])),
///     p.parse()
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct Parser<'source> {
    source: &'source str,
    current: Cell<usize>,
    tokens: Vec<Token>,
    error_display: ErrorDisplay,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str, tokens: Vec<Token>) -> Self {
        Self {
            source,
            current: Cell::new(0),
            tokens,
            error_display: ErrorDisplay,
        }
    }

    pub fn parse(&self) -> Result<Node, ParserError> {
        self.parse_literal()
    }

    fn parse_object(&self) -> Result<Node, ParserError> {
        self.next_or_error(TokenType::LeftBrace)?;

        let mut property_map = PropertyMap::new();

        if matches!(self.peek(), Some(Token { token_type, .. }) if *token_type != TokenType::RightBrace)
        {
            let (key, property) = self.parse_property()?;
            property_map.insert(key, property)?;

            while matches!(self.peek(), Some(Token { token_type, .. }) if *token_type == TokenType::Comma)
            {
                self.next();
                let (key, property) = self.parse_property()?;
                property_map.insert(key, property)?;
            }
        }

        self.next_or_error(TokenType::RightBrace)?;

        Ok(Node::Object(property_map.ordered_properties))
    }

    fn parse_property(&self) -> Result<(&str, Node), ParserError> {
        let (start, end) = self.next_or_error(TokenType::String)?.indices;
        let key = Node::Literal(&self.source[start..end]);

        let _colon = self.next_or_error(TokenType::Colon)?;

        let value = self.parse_literal()?;

        Ok((
            &self.source[start..end],
            Node::Property(Box::new(key), Box::new(value)),
        ))
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
            indices: (start, end),
            ..
        }) = self.peek()
        {
            match token_type {
                TokenType::Null
                | TokenType::String
                | TokenType::Number
                | TokenType::True
                | TokenType::False => {
                    let node = Ok(Node::Literal(&self.source[*start..*end]));
                    self.next();
                    return node;
                }
                TokenType::LeftBracket => return self.parse_array(),
                TokenType::LeftBrace => return self.parse_object(),
                _ => {
                    return Err(ParserError::UnexpectedToken {
                        error: self.error_display.preview(self.source, *start, *end),
                    })
                }
            };
        }

        Err(ParserError::UnexpectedToken {
            error: "".to_string(),
        })
    }

    fn next_or_error(&self, expected_token_type: TokenType) -> Result<&Token, ParserError> {
        if let Some(token) = self.peek() {
            if expected_token_type == token.token_type {
                self.next();
                return Ok(token);
            }
        }

        if let Some(token) = self.peek() {
            let (start, current) = token.indices;
            return Err(ParserError::UnexpectedToken {
                error: self.error_display.preview(self.source, start, current),
            });
        }

        return Err(ParserError::UnexpectedToken {
            error: "".to_string(),
        });
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
    fn parse_empty_object() {
        let p = Parser::new(
            "{}",
            vec![
                Token::new(TokenType::LeftBrace, 1, (0, 1), (1, 2)),
                Token::new(TokenType::RightBrace, 1, (1, 2), (2, 3)),
            ],
        );

        assert_eq!(Ok(Node::Object(vec![])), p.parse_object());
    }

    #[test]
    fn parse_valid_object() {
        let p = Parser::new(
            "{\"animal\":\"dog\"}",
            vec![
                Token::new(TokenType::LeftBrace, 1, (0, 1), (1, 2)),
                Token::new(TokenType::String, 1, (1, 9), (2, 10)),
                Token::new(TokenType::Colon, 1, (9, 10), (10, 11)),
                Token::new(TokenType::String, 1, (10, 15), (11, 16)),
                Token::new(TokenType::RightBrace, 1, (15, 16), (16, 17)),
            ],
        );

        assert_eq!(
            Ok(Node::Object(vec![Node::Property(
                Box::new(Node::Literal("\"animal\"",)),
                Box::new(Node::Literal("\"dog\"",)),
            ),])),
            p.parse_object()
        );
    }

    #[test]
    fn parse_valid_property() {
        let p = Parser::new(
            "\"animal\":\"dog\"",
            vec![
                Token::new(TokenType::String, 1, (0, 8), (1, 9)),
                Token::new(TokenType::Colon, 1, (8, 9), (9, 10)),
                Token::new(TokenType::String, 1, (9, 14), (10, 15)),
            ],
        );

        assert_eq!(
            Ok((
                "\"animal\"",
                Node::Property(
                    Box::new(Node::Literal("\"animal\""),),
                    Box::new(Node::Literal("\"dog\""))
                )
            )),
            p.parse_property()
        );
    }

    #[test]
    fn error_invalid_array() {
        let p = Parser::new(
            "[,]",
            vec![
                Token::new(TokenType::LeftBracket, 1, (0, 1), (1, 2)),
                Token::new(TokenType::Comma, 1, (1, 2), (2, 3)),
                Token::new(TokenType::RightBracket, 1, (2, 3), (3, 4)),
            ],
        );

        assert_eq!(true, p.parse_array().is_err());
    }

    #[test]
    fn parse_empty_array() {
        let p = Parser::new(
            "[]",
            vec![
                Token::new(TokenType::LeftBracket, 1, (0, 1), (1, 2)),
                Token::new(TokenType::RightBracket, 1, (1, 2), (2, 3)),
            ],
        );

        assert_eq!(Ok(Node::Array(vec![])), p.parse_array());
    }

    #[test]
    fn parse_valid_array() {
        let p = Parser::new(
            "[true,false]",
            vec![
                Token::new(TokenType::LeftBracket, 1, (0, 1), (1, 2)),
                Token::new(TokenType::True, 1, (1, 5), (5, 6)),
                Token::new(TokenType::Comma, 1, (5, 6), (6, 7)),
                Token::new(TokenType::False, 1, (6, 11), (7, 12)),
                Token::new(TokenType::RightBracket, 1, (11, 12), (12, 13)),
            ],
        );

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
        let p = Parser::new(
            "null",
            vec![Token::new(TokenType::String, 1, (0, 4), (1, 5))],
        );
        assert_eq!(Ok(Node::Literal("null")), p.parse_literal());
    }

    #[test]
    fn parse_string_literal() {
        let p = Parser::new(
            "\"dog\"",
            vec![Token::new(TokenType::String, 1, (0, 5), (1, 6))],
        );

        assert_eq!(Ok(Node::Literal("\"dog\"")), p.parse_literal());
    }

    #[test]
    fn parse_number_literal() {
        let p = Parser::new(
            "1016",
            vec![Token::new(TokenType::Number, 1, (0, 4), (1, 5))],
        );

        assert_eq!(Ok(Node::Literal("1016")), p.parse_literal());
    }

    #[test]
    fn parse_true_literal() {
        let p = Parser::new("true", vec![Token::new(TokenType::True, 1, (0, 4), (1, 5))]);

        assert_eq!(Ok(Node::Literal("true")), p.parse_literal());
    }

    #[test]
    fn parse_false_literal() {
        let p = Parser::new(
            "false",
            vec![Token::new(TokenType::False, 1, (0, 5), (1, 6))],
        );
        assert_eq!(Ok(Node::Literal("false")), p.parse_literal());
    }

    #[test]
    fn consume_next_token_when_expected() {
        let p = Parser::new("true", vec![Token::new(TokenType::True, 1, (0, 4), (1, 5))]);

        assert_eq!(
            Ok(&Token::new(TokenType::True, 1, (0, 4), (1, 5))),
            p.next_or_error(TokenType::True)
        );
    }

    #[test]
    fn error_on_unexpected_token() {
        let p = Parser::new("true", vec![Token::new(TokenType::True, 1, (0, 4), (1, 5))]);

        assert_eq!(true, p.next_or_error(TokenType::LeftBrace).is_err());
    }

    #[test]
    fn consume_next_until_end() {
        let p = Parser::new(
            "[true,false]",
            vec![
                Token::new(TokenType::LeftBracket, 1, (0, 1), (1, 2)),
                Token::new(TokenType::True, 1, (1, 5), (5, 6)),
                Token::new(TokenType::Comma, 1, (5, 6), (6, 7)),
                Token::new(TokenType::False, 1, (6, 11), (7, 12)),
                Token::new(TokenType::RightBracket, 1, (11, 12), (12, 13)),
            ],
        );

        p.next();
        p.next();
        p.next();
        p.next();
        p.next();

        assert_eq!(None, p.next());
    }

    #[test]
    fn next_is_some() {
        let p = Parser::new("true", vec![Token::new(TokenType::True, 1, (0, 4), (1, 5))]);

        assert_eq!(
            Some(&Token::new(TokenType::True, 1, (0, 4), (1, 5))),
            p.next()
        );
        assert_eq!(1, p.current.get());
        assert_eq!(None, p.next());
    }

    #[test]
    fn next_is_none() {
        let p = Parser::new("", vec![]);

        assert_eq!(None, p.next());
        assert_eq!(0, p.current.get());
    }

    #[test]
    fn peek_is_none() {
        let p = Parser::new("", vec![]);
        assert_eq!(None, p.peek());
    }

    #[test]
    fn peek_is_some() {
        let p = Parser::new("true", vec![Token::new(TokenType::True, 1, (0, 4), (1, 5))]);

        assert_eq!(
            Some(&Token::new(TokenType::True, 1, (0, 4), (1, 5))),
            p.peek()
        );
    }

    #[test]
    fn create_new_parser() {
        let p = Parser::new("true", vec![Token::new(TokenType::True, 1, (0, 4), (1, 5))]);

        assert_eq!(
            Parser {
                source: "true",
                current: Cell::new(0),
                tokens: vec![Token::new(TokenType::True, 1, (0, 4), (1, 5))],
                error_display: ErrorDisplay,
            },
            p
        );
    }
}
