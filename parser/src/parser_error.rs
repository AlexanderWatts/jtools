use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    DuplicateProperty { property: String, error: String },
    UnexpectedToken { header: String, error: String },
}

impl std::error::Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::DuplicateProperty { property, error } => {
                write!(f, "Duplicate Property {} {}", property, error)
            }
            ParserError::UnexpectedToken { header, error } => {
                write!(f, "{} {}", header, error)
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
                error: "some error".to_string()
            }
            .to_string()
        );
    }

    #[test]
    fn unexpected_token_message() {
        assert_eq!(
            "Header error",
            ParserError::UnexpectedToken {
                header: "Header".to_string(),
                error: "error".to_string(),
            }
            .to_string()
        );
    }
}
