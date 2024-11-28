use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum ScannerError {
    EmptySource,
    UnknownCharacter,
    UnknownLiteral,
    UnterminatedString,
}

impl Error for ScannerError {}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySource => write!(f, "Empty source"),
            Self::UnknownCharacter => write!(f, "Unknown character"),
            Self::UnknownLiteral => write!(f, "Unknown literal"),
            Self::UnterminatedString => write!(f, "Unterminated string"),
        }
    }
}

#[cfg(test)]
mod scanner_error_tests {
    use super::*;

    #[test]
    fn expect_unterminated_string_message() {
        assert_eq!(
            "Unterminated string",
            ScannerError::UnterminatedString.to_string()
        );
    }

    #[test]
    fn expect_unknown_literal_message() {
        assert_eq!("Unknown literal", ScannerError::UnknownLiteral.to_string());
    }

    #[test]
    fn expect_unknown_character_message() {
        assert_eq!(
            "Unknown character",
            ScannerError::UnknownCharacter.to_string()
        );
    }

    #[test]
    fn expect_empty_source_message() {
        assert_eq!("Empty source", ScannerError::EmptySource.to_string());
    }
}
