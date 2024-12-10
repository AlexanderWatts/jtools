use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    DuplicateProperty { error: String },
    UnexpectedToken { error: String },
}

impl std::error::Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::DuplicateProperty { error } => write!(f, "Duplicate Property {}", error),
            ParserError::UnexpectedToken { error } => write!(f, "Unexpected Token {}", error),
        }
    }
}

#[cfg(test)]
mod parser_error_tests {
    use crate::parser_error::ParserError;

    #[test]
    fn duplicate_property_message() {
        assert_eq!(
            "Duplicate Property some error",
            ParserError::DuplicateProperty {
                error: "some error".to_string()
            }
            .to_string()
        );
    }

    #[test]
    fn unexpected_token_message() {
        assert_eq!(
            "Unexpected Token ",
            ParserError::UnexpectedToken {
                error: "".to_string()
            }
            .to_string()
        );
    }
}
