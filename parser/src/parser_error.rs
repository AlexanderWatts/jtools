use std::fmt::Display;

use scanner::scanner_error::ScannerError;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    ScannerError(ScannerError),
    DuplicateProperty {
        property: String,
        error_preview: String,
    },
    UnexpectedToken {
        expected: String,
        found: String,
        error_preview: String,
    },
}

impl std::error::Error for ParserError {}

impl From<ScannerError> for ParserError {
    fn from(scanner_error: ScannerError) -> Self {
        ParserError::ScannerError(scanner_error)
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::ScannerError(_) => {
                write!(f, "Scanner error")
            }
            ParserError::DuplicateProperty {
                property,
                error_preview,
            } => {
                write!(f, "Duplicate property {} {}", property, error_preview)
            }
            ParserError::UnexpectedToken {
                expected,
                found,
                error_preview,
            } => {
                write!(f, "Expected {} found {} {}", expected, found, error_preview)
            }
        }
    }
}

#[cfg(test)]
mod parser_error_tests {
    use crate::parser_error::ParserError;

    #[test]
    fn duplicate_property_message() {
        assert_eq!(
            "Duplicate property \"hello\" error preview",
            ParserError::DuplicateProperty {
                property: "\"hello\"".to_string(),
                error_preview: "error preview".to_string()
            }
            .to_string()
        );
    }

    #[test]
    fn unexpected_token_message() {
        assert_eq!(
            "Expected string found , error preview",
            ParserError::UnexpectedToken {
                expected: "string".to_string(),
                found: ",".to_string(),
                error_preview: "error preview".to_string()
            }
            .to_string()
        );
    }
}
