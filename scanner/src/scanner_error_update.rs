use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum ErrorType {
    EmptySource,
    UnknownCharacter,
    UnknownLiteral,
    UnterminatedString,
    UnterminatedFractionalNumber,
    LeadingZeros,
    InvalidExponent,
    InvalidNumber,
    InvalidEscapeSequence,
    InvalidUnicodeSequence,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySource => write!(f, "Empty source"),
            Self::UnknownCharacter => write!(f, "Unknown character"),
            Self::UnknownLiteral => write!(f, "Unknown literal"),
            Self::UnterminatedString => write!(f, "Unterminated string"),
            Self::UnterminatedFractionalNumber => write!(f, "Unterminated fractional number"),
            Self::LeadingZeros => write!(f, "Leading zeros"),
            Self::InvalidExponent => write!(f, "Invalid exponent"),
            Self::InvalidNumber => write!(f, "Invalid number"),
            Self::InvalidEscapeSequence => write!(f, "Invalid escape sequence"),
            Self::InvalidUnicodeSequence => write!(f, "Invalid unicode sequence"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ScannerError {
    pub error_type: ErrorType,
    pub preview: String,
    pub hint: Option<String>,
}

impl<'source> ScannerError {
    pub fn new(error_type: ErrorType, preview: String, hint: Option<String>) -> Self {
        Self {
            error_type,
            preview,
            hint,
        }
    }
}

impl Error for ScannerError {}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.error_type,
            self.preview,
            self.hint
                .as_ref()
                .map(|hint| format!("\n✨ {}", hint))
                .unwrap_or("".to_string())
        )
    }
}
