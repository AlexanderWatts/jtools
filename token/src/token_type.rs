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
            TokenType::LeftBrace => "LeftBrace",
            TokenType::RightBrace => "RightBrace",
            TokenType::LeftBracket => "LeftBracket",
            TokenType::RightBracket => "RightBracket",
            TokenType::Colon => "Colon",
            TokenType::Comma => "Comma",
            TokenType::String => "String",
            TokenType::Number => "Number",
            TokenType::True => "True",
            TokenType::False => "False",
            TokenType::Null => "Null",
            TokenType::Eof => "Eof",
        };

        write!(f, "{}", value)
    }
}
