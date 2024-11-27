/// Abstract Syntax Tree (AST) node
///
/// ## Description
///
/// Node is a recursive type that is used by the parser to construct an AST, maintaining only the structure
/// and core literal values of a JSON input.
///
/// ## Examples
///```rust
/// use ast::node::Node;
///
/// let literal = Node::Literal("false");
///
/// let array = Node::Array(vec![Node::Literal("true"), Node::Literal("false")]);
///
/// let property = Node::Property(
///     Box::new(Node::Literal("\"data\"")),
///     Box::new(Node::Literal("\"none\""))
/// );
///
/// let object = Node::Object(vec![Node::Property(
///     Box::new(Node::Literal("\"type\"")),
///     Box::new(Node::Literal("\"lion\""))
/// )]);
///```
#[derive(Debug, PartialEq)]
pub enum Node<'source> {
    Object(Vec<Node<'source>>),
    Property(Box<Node<'source>>, Box<Node<'source>>),
    Array(Vec<Node<'source>>),
    Literal(&'source str),
}

#[cfg(test)]
mod ast_node_tests {
    use token::{token::Token, token_type::TokenType};

    use super::*;

    #[test]
    fn create_ast_literal() {
        let source = "false";
        let tokens = vec![Token::new(TokenType::False, 1, (0, 5), (1, 6))];

        assert_eq!(
            Node::Literal("false"),
            Node::Literal(&source[tokens[0].indices.0..tokens[0].indices.1])
        );
    }

    #[test]
    fn create_ast_array() {
        let source = "[true,false]";

        let tokens = vec![
            Token::new(TokenType::LeftBracket, 1, (0, 1), (1, 2)),
            Token::new(TokenType::True, 1, (1, 5), (5, 6)),
            Token::new(TokenType::Comma, 1, (5, 6), (6, 7)),
            Token::new(TokenType::False, 1, (6, 11), (7, 12)),
            Token::new(TokenType::RightBracket, 1, (11, 12), (12, 13)),
        ];

        assert_eq!(
            Node::Array(vec![Node::Literal("true"), Node::Literal("false")]),
            Node::Array(vec![
                Node::Literal(&source[tokens[1].indices.0..tokens[1].indices.1]),
                Node::Literal(&source[tokens[3].indices.0..tokens[3].indices.1]),
            ]),
        );
    }

    #[test]
    fn create_ast_property() {
        let source = "{\"animal\":\"🐶\"}";

        let tokens = vec![
            Token::new(TokenType::LeftBrace, 1, (0, 1), (1, 2)),
            Token::new(TokenType::String, 1, (1, 9), (2, 10)),
            Token::new(TokenType::Colon, 1, (9, 10), (10, 11)),
            Token::new(TokenType::String, 1, (10, 16), (11, 17)),
            Token::new(TokenType::RightBrace, 1, (16, 17), (17, 18)),
        ];

        assert_eq!(
            Node::Property(
                Box::new(Node::Literal("\"animal\"")),
                Box::new(Node::Literal("\"🐶\""))
            ),
            Node::Property(
                Box::new(Node::Literal(
                    &source[tokens[1].indices.0..tokens[1].indices.1]
                )),
                Box::new(Node::Literal(
                    &source[tokens[3].indices.0..tokens[3].indices.1]
                ))
            ),
        );
    }

    #[test]
    fn create_ast_object() {
        let source = "{\"animal\":\"dog\"}";

        let tokens = vec![
            Token::new(TokenType::LeftBrace, 1, (0, 1), (1, 2)),
            Token::new(TokenType::String, 1, (1, 9), (2, 10)),
            Token::new(TokenType::Colon, 1, (9, 10), (10, 11)),
            Token::new(TokenType::String, 1, (10, 15), (11, 16)),
            Token::new(TokenType::RightBrace, 1, (15, 16), (16, 17)),
        ];

        assert_eq!(
            Node::Object(vec![Node::Property(
                Box::new(Node::Literal("\"animal\"")),
                Box::new(Node::Literal("\"dog\""))
            ),]),
            Node::Object(vec![Node::Property(
                Box::new(Node::Literal(
                    &source[tokens[1].indices.0..tokens[1].indices.1]
                )),
                Box::new(Node::Literal(
                    &source[tokens[3].indices.0..tokens[3].indices.1]
                ))
            ),]),
        );
    }
}
