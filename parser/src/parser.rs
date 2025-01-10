use std::cell::Cell;

use ast::node::Node;
use error_preview::error_preview::ErrorPreview;
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
///         Token::new(TokenType::Eof, 1, (16, 16), (17, 17)),
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
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str, tokens: Vec<Token>) -> Self {
        Self {
            source,
            current: Cell::new(0),
            tokens,
        }
    }

    pub fn parse(&self) -> Result<Node, ParserError> {
        let ast = self.parse_literal()?;

        self.next_or_error(TokenType::Eof, "Expected end of input")?;

        Ok(ast)
    }

    pub fn is_valid(&self) -> bool {
        self.parse().is_ok()
    }

    fn parse_object(&self) -> Result<Node, ParserError> {
        let mut property_map = PropertyMap::new();

        if matches!(self.peek(), Some(Token { token_type, .. }) if *token_type != TokenType::RightBrace)
        {
            let (key, property, token) = self.parse_property()?;

            property_map
                .insert(key, property)
                .ok_or_else(|| ParserError::DuplicateProperty {
                    property: key.to_string(),
                    error: self.error_preview(token),
                })?;

            while matches!(self.peek(), Some(Token { token_type, .. }) if *token_type == TokenType::Comma)
            {
                self.next();
                let (key, property, token) = self.parse_property()?;

                property_map.insert(key, property).ok_or_else(|| {
                    ParserError::DuplicateProperty {
                        property: key.to_string(),
                        error: self.error_preview(token),
                    }
                })?;
            }
        }

        self.next_or_error(TokenType::RightBrace, "Expected object to be terminated")?;

        Ok(Node::Object(property_map.ordered_properties))
    }

    fn parse_property(&self) -> Result<(&str, Node, &Token), ParserError> {
        let token = self.next_or_error(TokenType::String, "Object keys must be of type string")?;

        let (start, end) = token.indices;
        let key = Node::Literal(&self.source[start..end]);

        let _colon = self.next_or_error(
            TokenType::Colon,
            "Object key-values must be separated by a semicolon",
        )?;

        let value = self.parse_literal()?;

        Ok((
            &self.source[start..end],
            Node::Property(Box::new(key), Box::new(value)),
            token,
        ))
    }

    fn parse_array(&self) -> Result<Node, ParserError> {
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

        self.next_or_error(TokenType::RightBracket, "Expected array to be terminated")?;

        Ok(Node::Array(values))
    }

    fn parse_literal(&self) -> Result<Node, ParserError> {
        match self.peek() {
            Some(Token {
                indices: (start, end),
                token_type:
                    TokenType::Null
                    | TokenType::String
                    | TokenType::Number
                    | TokenType::True
                    | TokenType::False,
                ..
            }) => {
                let node = Ok(Node::Literal(&self.source[*start..*end]));
                self.next();
                return node;
            }
            Some(Token {
                token_type: TokenType::LeftBrace,
                ..
            }) => {
                self.next();
                return self.parse_object();
            }
            Some(Token {
                token_type: TokenType::LeftBracket,
                ..
            }) => {
                self.next();
                return self.parse_array();
            }
            Some(token) => {
                return Err(ParserError::UnexpectedToken {
                    header: "Expected string|number|bool|object|array".to_string(),
                    error: self.error_preview(token),
                })
            }
            _ => {
                // This will never be run
                return Err(ParserError::UnexpectedToken {
                    header: "".to_string(),
                    error: "".to_string(),
                });
            }
        }
    }

    fn next_or_error(
        &self,
        expected_token_type: TokenType,
        error: &str,
    ) -> Result<&Token, ParserError> {
        if let Some(token) = self.peek() {
            if expected_token_type == token.token_type {
                self.next();
                return Ok(token);
            }
        }

        if let Some(token) = self.peek() {
            return Err(ParserError::UnexpectedToken {
                header: error.to_string(),
                error: self.error_preview(token),
            });
        }

        // This will never be run
        Err(ParserError::UnexpectedToken {
            header: "".to_string(),
            error: "".to_string(),
        })
    }

    fn error_preview(&self, token: &Token) -> String {
        let Token {
            indices: (start, _),
            column_indices: (column_start, _),
            line_number,
            ..
        } = token;

        ErrorPreview.preview(self.source, *start, *column_start, *line_number)
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
    fn parse_valid_tokens() {
        let p = Parser::new(
            "{}",
            vec![
                Token::new(TokenType::LeftBrace, 1, (0, 1), (1, 2)),
                Token::new(TokenType::RightBrace, 1, (1, 2), (2, 3)),
                Token::new(TokenType::Eof, 1, (2, 2), (3, 3)),
            ],
        );

        assert_eq!(true, p.is_valid());
    }

    #[test]
    fn parse_empty_object() {
        let p = Parser::new(
            "{}",
            vec![
                Token::new(TokenType::LeftBrace, 1, (0, 1), (1, 2)),
                Token::new(TokenType::RightBrace, 1, (1, 2), (2, 3)),
                Token::new(TokenType::Eof, 1, (2, 2), (3, 3)),
            ],
        );

        assert_eq!(Ok(Node::Object(vec![])), p.parse());
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
                Token::new(TokenType::Eof, 1, (15, 15), (16, 16)),
            ],
        );

        assert_eq!(
            Ok(Node::Object(vec![Node::Property(
                Box::new(Node::Literal("\"animal\"",)),
                Box::new(Node::Literal("\"dog\"",)),
            ),])),
            p.parse()
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
                ),
                &Token::new(TokenType::String, 1, (0, 8), (1, 9)),
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

        assert_eq!(true, p.parse().is_err());
    }

    #[test]
    fn parse_empty_array() {
        let p = Parser::new(
            "[]",
            vec![
                Token::new(TokenType::LeftBracket, 1, (0, 1), (1, 2)),
                Token::new(TokenType::RightBracket, 1, (1, 2), (2, 3)),
                Token::new(TokenType::Eof, 1, (1, 1), (2, 2)),
            ],
        );

        assert_eq!(Ok(Node::Array(vec![])), p.parse());
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
                Token::new(TokenType::Eof, 1, (11, 11), (12, 12)),
            ],
        );

        assert_eq!(
            Ok(Node::Array(vec![
                Node::Literal("true"),
                Node::Literal("false")
            ])),
            p.parse()
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
            p.next_or_error(TokenType::True, "Expected string|number|bool|object|array")
        );
    }

    #[test]
    fn error_on_unexpected_token() {
        let p = Parser::new("true", vec![Token::new(TokenType::True, 1, (0, 4), (1, 5))]);

        assert_eq!(true, p.next_or_error(TokenType::LeftBrace, "{").is_err());
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
            },
            p
        );
    }
}
