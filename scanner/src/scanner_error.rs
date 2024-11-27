use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum ScannerError {
    EmptySource,
    UnknownCharacter,
}

impl Error for ScannerError {}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySource => write!(f, "Empty source"),
            Self::UnknownCharacter => write!(f, "Unknown character"),
        }
    }
}

#[cfg(test)]
mod scanner_error_tests {
    use super::*;

    #[test]
    fn duplicate_property_message() {
        assert_eq!(
            "Unknown character",
            ScannerError::UnknownCharacter.to_string()
        );
    }

    #[test]
    fn unexpected_token_message() {
        assert_eq!("Empty source", ScannerError::EmptySource.to_string());
    }
}
