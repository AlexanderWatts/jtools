use std::fmt::Display;

/// A type given to a token during scanning.
///
/// ## Description
///
/// Token types are relied on during parsing to determine if the token being evaluated is of the
/// expected type. Take the following pseudocode:
///
/// ```text
/// if token.token_type != left_brace {
///     return parser_error
/// }
/// continue parsing...
/// ```
///
/// ## Examples
/// ```
/// use token::token_type::TokenType;
///
/// let left_bracket = TokenType::LeftBrace;
/// let end_of_file = TokenType::Eof;
/// ```
#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    String,
    Number,
    True,
    False,
    Null,
    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            TokenType::LeftBrace => "{",
            TokenType::RightBrace => "}",
            TokenType::LeftBracket => "[",
            TokenType::RightBracket => "]",
            TokenType::Colon => ":",
            TokenType::Comma => ",",
            TokenType::String => "string",
            TokenType::Number => "number",
            TokenType::True => "true",
            TokenType::False => "false",
            TokenType::Null => "null",
            TokenType::Eof => "eof",
        };

        write!(f, "{}", value)
    }
}
