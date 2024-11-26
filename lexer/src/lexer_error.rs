#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnknownCharacter,
    UnterminatedString,
    UnterminatedFractionalNumber,
}

impl std::error::Error for LexerError {}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownCharacter => write!(f, "Unknown Character"),
            Self::UnterminatedString => write!(f, "Unterminated String"),
            Self::UnterminatedFractionalNumber => write!(f, "Unterminated Fractional Number"),
        }
    }
}

#[cfg(test)]
mod lexer_error_tests {
    use super::*;

    #[test]
    fn expected_fractional_number_message() {
        assert_eq!(
            "Unterminated Fractional Number",
            LexerError::UnterminatedFractionalNumber.to_string()
        )
    }

    #[test]
    fn expected_unterminated_string_message() {
        assert_eq!(
            "Unterminated String",
            LexerError::UnterminatedString.to_string()
        )
    }

    #[test]
    fn expected_unknown_character_message() {
        assert_eq!(
            "Unknown Character",
            LexerError::UnknownCharacter.to_string()
        )
    }
}
