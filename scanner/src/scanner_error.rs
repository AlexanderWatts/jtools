use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum ScannerError {
    EmptySource { error: String },
    UnknownCharacter { error: String },
    UnknownLiteral { error: String },
    UnterminatedString { error: String },
    UnterminatedFractionalNumber { error: String },
    LeadingZeros { error: String },
    InvalidExponent { error: String },
    InvalidNumber { error: String },
    InvalidEscapeSequence { error: String },
    InvalidUnicodeSequence { error: String },
}

impl Error for ScannerError {}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySource { error } => {
                write!(f, "Empty source {}", error)
            }
            Self::UnknownCharacter { error } => {
                write!(f, "Unknown character {}", error)
            }
            Self::UnknownLiteral { error } => write!(f, "Unknown literal {}", error),
            Self::UnterminatedString { error } => write!(f, "Unterminated string {}", error),
            Self::UnterminatedFractionalNumber { error } => {
                write!(f, "Unterminated fractional number {}", error)
            }
            Self::LeadingZeros { error } => write!(f, "Leading zeros {}", error),
            Self::InvalidExponent { error } => write!(f, "Invalid exponent {}", error),
            Self::InvalidNumber { error } => write!(f, "Invalid number {}", error),
            Self::InvalidEscapeSequence { error } => write!(f, "Invalid escape sequence {}", error),
            Self::InvalidUnicodeSequence { error } => {
                write!(f, "Invalid unicode sequence {}", error)
            }
        }
    }
}

#[cfg(test)]
mod scanner_error_tests {
    use super::*;

    #[test]
    fn expect_invaild_unicode_sequence_message() {
        assert_eq!(
            "Invalid unicode sequence \"\\uaaaa\"",
            ScannerError::InvalidUnicodeSequence {
                error: "\"\\uaaaa\"".to_string()
            }
            .to_string()
        );
    }

    #[test]
    fn expect_invaild_escape_sequence_message() {
        assert_eq!(
            "Invalid escape sequence \"\\\\e\"",
            ScannerError::InvalidEscapeSequence {
                error: "\"\\\\e\"".to_string()
            }
            .to_string()
        );
    }

    #[test]
    fn expect_invalid_number_message() {
        assert_eq!(
            "Invalid number 0.2e",
            ScannerError::InvalidNumber {
                error: "0.2e".to_string()
            }
            .to_string()
        );
    }

    #[test]
    fn expect_invalid_exponent_message() {
        assert_eq!(
            "Invalid exponent 20Ee",
            ScannerError::InvalidExponent {
                error: "20Ee".to_string()
            }
            .to_string()
        );
    }

    #[test]
    fn expect_leading_zeros_message() {
        assert_eq!(
            "Leading zeros 00.42",
            ScannerError::LeadingZeros {
                error: "00.42".to_string()
            }
            .to_string()
        );
    }

    #[test]
    fn expect_unterminated_fractional_number_message() {
        assert_eq!(
            "Unterminated fractional number 100.",
            ScannerError::UnterminatedFractionalNumber {
                error: "100.".to_string()
            }
            .to_string()
        );
    }

    #[test]
    fn expect_unterminated_string_message() {
        assert_eq!(
            "Unterminated string \"hello",
            ScannerError::UnterminatedString {
                error: "\"hello".to_string()
            }
            .to_string()
        );
    }

    #[test]
    fn expect_unknown_literal_message() {
        assert_eq!(
            "Unknown literal hello",
            ScannerError::UnknownLiteral {
                error: "hello".to_string()
            }
            .to_string()
        );
    }

    #[test]
    fn expect_unknown_character_message() {
        assert_eq!(
            "Unknown character @",
            ScannerError::UnknownCharacter {
                error: "@".to_string()
            }
            .to_string()
        );
    }

    #[test]
    fn expect_empty_source_message() {
        assert_eq!(
            "Empty source ",
            ScannerError::EmptySource {
                error: "".to_string()
            }
            .to_string()
        );
    }
}
