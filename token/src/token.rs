use crate::token_type::TokenType;

#[derive(Debug, PartialEq)]
pub struct Token<'source> {
    token_type: TokenType,
    literal: &'source str,
    line_number: usize,
    column_start: usize,
    column_end: usize,
}

impl<'source> Token<'source> {
    pub fn new(
        token_type: TokenType,
        literal: &'source str,
        line_number: usize,
        column_start: usize,
        column_end: usize,
    ) -> Self {
        Self {
            token_type,
            literal,
            line_number,
            column_start,
            column_end,
        }
    }
}

#[cfg(test)]
mod token_tests {
    use super::*;

    #[test]
    fn create_column_span_from_token_position() {
        let source = "{ \"is_ok\": false }";

        let start = 11;
        let current = 16;

        let t = Token::new(TokenType::False, &source[start..current], 1, start, current);

        assert_eq!(11..16, t.column_start..t.column_end);
    }

    #[test]
    fn retrieve_literal_from_token() {
        let source = "[]";

        assert_eq!(
            "[",
            Token::new(TokenType::LeftBracket, &source[0..1], 1, 0, 1).literal
        );
    }

    #[test]
    fn store_slice_of_input() {
        let source = String::from("}");

        assert_eq!(
            Token::new(TokenType::RightBrace, "}", 1, 0, 1),
            Token::new(TokenType::RightBrace, &source[0..1], 1, 0, 1)
        );

        let source = String::from("\"hello\"");
        assert_eq!(
            "\"hello\"",
            Token::new(TokenType::RightBrace, &source[0..7], 1, 0, 7).literal
        );
    }

    #[test]
    fn create_new_token() {
        assert_eq!(
            Token::new(TokenType::String, "\"hello\"", 1, 0, 7),
            Token::new(TokenType::String, "\"hello\"", 1, 0, 7)
        );
    }
}
