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
        hint: Option<String>,
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
                write!(f, "Duplicate property {} {}", property, error_preview,)
            }
            ParserError::UnexpectedToken {
                expected,
                found,
                error_preview,
                hint,
            } => {
                write!(
                    f,
                    "Expected {} found {} {} {}",
                    expected,
                    found,
                    error_preview,
                    hint.as_ref()
                        .map(|hint| format!("\n✨{}", hint))
                        .unwrap_or(String::from(""))
                )
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
            "Expected string found , error preview \n✨This is a hint",
            ParserError::UnexpectedToken {
                expected: "string".to_string(),
                found: ",".to_string(),
                error_preview: "error preview".to_string(),
                hint: Some("This is a hint".to_string()),
            }
            .to_string()
        );
    }
}
