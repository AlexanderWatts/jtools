#[derive(Debug)]
pub enum LexerError {
    UnknownCharacter,
}

impl std::error::Error for LexerError {}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownCharacter => write!(f, "Unknown Character"),
        }
    }
}

#[cfg(test)]
mod lexer_error_tests {
    use super::*;

    #[test]
    fn expected_unknown_character_message() {
        assert_eq!(
            "Unknown Character",
            LexerError::UnknownCharacter.to_string()
        )
    }
}
