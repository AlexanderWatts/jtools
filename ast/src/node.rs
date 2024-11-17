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
        let tokens = vec![Token::new(TokenType::False, &source[0..5], 1, 1, 6)];

        assert_eq!(Node::Literal("false"), Node::Literal(tokens[0].literal));
    }

    #[test]
    fn create_ast_array() {
        let source = "[true,false]";

        let tokens = vec![
            Token::new(TokenType::LeftBracket, &source[0..1], 1, 1, 2),
            Token::new(TokenType::True, &source[1..5], 1, 2, 6),
            Token::new(TokenType::Comma, &source[5..6], 1, 6, 7),
            Token::new(TokenType::False, &source[6..11], 1, 7, 12),
            Token::new(TokenType::RightBracket, &source[11..12], 1, 12, 13),
        ];

        assert_eq!(
            Node::Array(vec![Node::Literal("true"), Node::Literal("false")]),
            Node::Array(vec![
                Node::Literal(tokens[1].literal),
                Node::Literal(tokens[3].literal),
            ]),
        );
    }

    #[test]
    fn create_ast_property() {
        let source = "{\"data\":\"none\"}";

        let tokens = vec![
            Token::new(TokenType::LeftBrace, &source[0..1], 1, 1, 2),
            Token::new(TokenType::String, &source[1..7], 1, 2, 8),
            Token::new(TokenType::Colon, &source[7..8], 1, 8, 9),
            Token::new(TokenType::String, &source[8..14], 1, 9, 15),
            Token::new(TokenType::RightBrace, &source[14..15], 1, 16, 17),
        ];

        assert_eq!(
            Node::Property(
                Box::new(Node::Literal("\"data\"")),
                Box::new(Node::Literal("\"none\""))
            ),
            Node::Property(
                Box::new(Node::Literal(tokens[1].literal)),
                Box::new(Node::Literal(tokens[3].literal))
            ),
        );
    }

    #[test]
    fn create_ast_object() {
        let source = "{\"type\":\"lion\"}";

        let tokens = vec![
            Token::new(TokenType::LeftBrace, &source[0..1], 1, 1, 2),
            Token::new(TokenType::String, &source[1..7], 1, 2, 8),
            Token::new(TokenType::Colon, &source[7..8], 1, 8, 9),
            Token::new(TokenType::String, &source[8..14], 1, 9, 15),
            Token::new(TokenType::RightBrace, &source[14..15], 1, 16, 17),
        ];

        assert_eq!(
            Node::Object(vec![Node::Property(
                Box::new(Node::Literal("\"type\"")),
                Box::new(Node::Literal("\"lion\""))
            ),]),
            Node::Object(vec![Node::Property(
                Box::new(Node::Literal(tokens[1].literal)),
                Box::new(Node::Literal(tokens[3].literal))
            ),]),
        );
    }
}
