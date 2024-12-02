use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum ScannerError {
    EmptySource,
    UnknownCharacter { preview: String },
    UnknownLiteral,
    UnterminatedString,
    UnterminatedFractionalNumber,
    LeadingZeros,
    InvalidExponent,
    InvalidNumber,
}

impl Error for ScannerError {}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySource => write!(f, "Empty source"),
            Self::UnknownCharacter { preview } => {
                write!(f, "Unknown character {}", preview)
            }
            Self::UnknownLiteral => write!(f, "Unknown literal"),
            Self::UnterminatedString => write!(f, "Unterminated string"),
            Self::UnterminatedFractionalNumber => write!(f, "Unterminated fractional number"),
            Self::LeadingZeros => write!(f, "Leading zeros"),
            Self::InvalidExponent => write!(f, "Invalid exponent"),
            Self::InvalidNumber => write!(f, "Invalid number"),
        }
    }
}

#[cfg(test)]
mod scanner_error_tests {
    use super::*;

    #[test]
    fn expect_invalid_number_message() {
        assert_eq!("Invalid number", ScannerError::InvalidNumber.to_string());
    }

    #[test]
    fn expect_invalid_exponent_message() {
        assert_eq!(
            "Invalid exponent",
            ScannerError::InvalidExponent.to_string()
        );
    }

    #[test]
    fn expect_leading_zeros_message() {
        assert_eq!("Leading zeros", ScannerError::LeadingZeros.to_string());
    }

    #[test]
    fn expect_unterminated_fractional_number_message() {
        assert_eq!(
            "Unterminated fractional number",
            ScannerError::UnterminatedFractionalNumber.to_string()
        );
    }

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
            "Unknown character hello",
            ScannerError::UnknownCharacter {
                preview: "hello".to_string()
            }
            .to_string()
        );
    }

    #[test]
    fn expect_empty_source_message() {
        assert_eq!("Empty source", ScannerError::EmptySource.to_string());
    }
}
