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
        match self {
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Comma => write!(f, ","),
            TokenType::String => write!(f, "string"),
            TokenType::Number => write!(f, "number"),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            TokenType::Null => write!(f, "null"),
            TokenType::Eof => write!(f, "eof"),
        }
    }
}
