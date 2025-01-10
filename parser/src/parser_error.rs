use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ParserError {
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

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::DuplicateProperty {
                property,
                error_preview,
            } => {
                write!(f, "Duplicate Property {} {}", property, error_preview)
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
            "Duplicate Property  some error",
            ParserError::DuplicateProperty {
                property: "".to_string(),
                error_preview: "some error".to_string()
            }
            .to_string()
        );
    }

    #[test]
    fn unexpected_token_message() {
        assert_eq!(
            "Header error",
            ParserError::UnexpectedToken {
                expected: "Header".to_string(),
                found: "Header".to_string(),
                error_preview: "error".to_string()
            }
            .to_string()
        );
    }
}
