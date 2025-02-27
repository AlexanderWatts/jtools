use std::collections::HashSet;

use ast::node::Node;
use error_preview::error_preview::ErrorPreview;
use scanner::{scanner::Scanner, token_buffer::TokenBuffer};
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
#[derive(Debug)]
pub struct Parser<'source> {
    pub source: &'source str,
    pub token_buffer: TokenBuffer<'source>,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            token_buffer: TokenBuffer::new(Scanner::new(source)),
        }
    }

    fn parse(&self) -> Result<Node, ParserError> {
        let ast = self.literal()?;

        let _ = self.next_or_err([TokenType::Eof])?;

        Ok(ast)
    }

    fn is_valid(&self) -> bool {
        self.parse().is_ok()
    }

    fn object(&self) -> Result<Node, ParserError> {
        let mut seen_property_keys: HashSet<&str> = HashSet::new();
        let mut values = vec![];

        if matches!(
            self.token_buffer.peek_token().as_ref(),
            Ok(Token { token_type, .. }) if *token_type != TokenType::RightBrace
        ) {
            values.push(self.property(&mut seen_property_keys)?);

            while matches!(
                self.token_buffer.peek_token().as_ref(),
                Ok(Token { token_type, .. }) if *token_type == TokenType::Comma
            ) {
                let _comma = self.token_buffer.get_token();

                values.push(self.property(&mut seen_property_keys)?);
            }
        }

        self.next_or_err([TokenType::RightBrace])?;

        Ok(Node::Object(values))
    }

    fn property(
        &self,
        seen_property_keys: &mut HashSet<&'source str>,
    ) -> Result<Node, ParserError> {
        let Token {
            indices: (start, end),
            ..
        } = self.next_or_err([TokenType::String])?;

        let key_literal = &self.source[start..end];

        if seen_property_keys.contains(key_literal) {
            return Err(ParserError::DuplicateProperty {
                property: key_literal.to_string(),
                error_preview: "".to_string(),
            });
        } else {
            seen_property_keys.insert(key_literal);
        }

        let key = Node::Literal(&self.source[start..end]);

        let _colon = self.next_or_err([TokenType::Colon])?;

        let value = self.literal()?;

        Ok(Node::Property(Box::new(key), Box::new(value)))
    }

    fn array(&self) -> Result<Node, ParserError> {
        let mut values = vec![];

        if matches!(
            self.token_buffer.peek_token().as_ref(),
            Ok(Token { token_type, .. }) if *token_type != TokenType::RightBracket)
        {
            values.push(self.literal()?);

            while matches!(
            self.token_buffer.peek_token().as_ref(),
            Ok(Token { token_type, .. }) if *token_type == TokenType::Comma)
            {
                let _comma = self.token_buffer.get_token();

                values.push(self.literal()?);
            }
        }

        self.next_or_err([TokenType::RightBracket])?;

        Ok(Node::Array(values))
    }

    fn literal(&self) -> Result<Node, ParserError> {
        match self.next_or_err([
            TokenType::String,
            TokenType::Number,
            TokenType::True,
            TokenType::False,
            TokenType::Null,
            TokenType::LeftBracket,
            TokenType::LeftBrace,
        ])? {
            Token {
                token_type: TokenType::LeftBracket,
                ..
            } => self.array(),
            Token {
                token_type: TokenType::LeftBrace,
                ..
            } => self.object(),
            Token {
                indices: (start, end),
                ..
            } => Ok(Node::Literal(&self.source[start..end])),
        }
    }

    fn next_or_err<I>(&self, expected_types: I) -> Result<Token, ParserError>
    where
        I: IntoIterator<Item = TokenType>,
    {
        match self.token_buffer.get_token()? {
            token
                if expected_types
                    .into_iter()
                    .any(|expected_type| token.token_type == expected_type) =>
            {
                Ok(token)
            }
            _ => {
                return Err(ParserError::UnexpectedToken {
                    expected: "".to_string(),
                    found: "".to_string(),
                    error_preview: "".to_string(),
                });
            }
        }
    }
}

#[cfg(test)]
mod parser_tests {
    use token::token_type::TokenType;

    use super::*;

    #[test]
    fn parse_object() {
        let parser = Parser::new("{\"prop\": false, \"is_published\": false}");

        assert_eq!(
            Ok(Node::Object(vec![
                Node::Property(
                    Box::new(Node::Literal("\"prop\"")),
                    Box::new(Node::Literal("false"))
                ),
                Node::Property(
                    Box::new(Node::Literal("\"is_published\"")),
                    Box::new(Node::Literal("false"))
                ),
            ])),
            parser.parse()
        );
    }

    #[test]
    fn parse_property() {
        let parser = Parser::new("\"message\": [\"Hello, World!\"]");

        assert_eq!(
            Ok(Node::Property(
                Box::new(Node::Literal("\"message\"")),
                Box::new(Node::Array(vec![Node::Literal("\"Hello, World!\"")]))
            )),
            parser.property(&mut HashSet::new())
        );
    }

    #[test]
    fn parse_array() {
        let parser = Parser::new("[true, false]");

        assert_eq!(
            Ok(Node::Array(vec![
                Node::Literal("true"),
                Node::Literal("false")
            ])),
            parser.parse()
        );
    }

    #[test]
    fn parse_literal() {
        let parser = Parser::new("true false null");

        assert_eq!(true, parser.next_or_err([TokenType::True]).is_ok());
        assert_eq!(true, parser.next_or_err([TokenType::False]).is_ok());
        assert_eq!(true, parser.next_or_err([TokenType::Null]).is_ok());
    }

    #[test]
    fn fail_to_parse_more_than_one_literal() {
        let parser = Parser::new("\"hello\", false");

        assert_eq!(true, parser.parse().is_err());
    }

    #[test]
    fn next_or_error() {
        let parser = Parser::new("{}");

        assert_eq!(true, parser.next_or_err([TokenType::LeftBrace]).is_ok());
        assert_eq!(true, parser.next_or_err([TokenType::Colon]).is_err());
    }
}
