use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    DuplicateProperty,
    UnexpectedToken,
}

impl std::error::Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::DuplicateProperty => write!(f, "Duplicate Property"),
            ParserError::UnexpectedToken => write!(f, "Unexpected Token"),
        }
    }
}

#[cfg(test)]
mod parser_error_tests {
    use crate::parser_error::ParserError;

    #[test]
    fn duplicate_property_message() {
        assert_eq!(
            "Duplicate Property",
            ParserError::DuplicateProperty.to_string()
        );
    }

    #[test]
    fn unexpected_token_message() {
        assert_eq!("Unexpected Token", ParserError::UnexpectedToken.to_string());
    }
}
