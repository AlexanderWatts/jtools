use std::{error::Error, fmt::Display, ops::ControlFlow};

#[derive(Debug, PartialEq)]
pub enum ScannerError {
    EmptySource,
    UnknownCharacter {
        source: String,
        indices: (usize, usize),
        display_position: (usize, usize, usize),
        error_message: String,
        preview_size: usize,
    },
    UnknownLiteral,
    UnterminatedString,
    UnterminatedFractionalNumber,
    LeadingZeros,
    InvalidExponent,
    InvalidNumber,
}

impl ScannerError {
    fn preview(&self, source: &str, indices: (usize, usize), limit: usize) -> String {
        let (start, end) = indices;

        let to = match source[end..].char_indices().try_fold(0, |acc, (_, char)| {
            let len = acc + char.len_utf8();

            if len >= limit {
                ControlFlow::Break(acc)
            } else if char == '\n' {
                ControlFlow::Break(acc)
            } else {
                ControlFlow::Continue(len)
            }
        }) {
            ControlFlow::Continue(index) => index,
            ControlFlow::Break(index) => index,
        };

        let from = match source[..start]
            .char_indices()
            .try_rfold(0, |acc, (_, char)| {
                let len = acc + char.len_utf8();

                if len >= limit {
                    ControlFlow::Break(acc)
                } else if char == '\n' {
                    ControlFlow::Break(acc)
                } else {
                    ControlFlow::Continue(len)
                }
            }) {
            ControlFlow::Continue(index) => index,
            ControlFlow::Break(index) => index,
        };

        source[start - from..end + to].to_string()
    }
}

impl Error for ScannerError {}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySource => write!(f, "Empty source"),
            Self::UnknownCharacter {
                source,
                indices,
                display_position,
                error_message,
                preview_size,
            } => {
                let error_header = format!(
                    "Error[line={}, column={}..{}]",
                    display_position.0, display_position.1, display_position.2
                );

                let preview = format!("....{}....", self.preview(source, *indices, *preview_size));

                write!(f, "{}\n{}\n{}", error_header, preview, error_message)
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
        //assert_eq!(
        //    "Unknown character",
        //    ScannerError::UnknownCharacter.to_string()
        //);
    }

    #[test]
    fn expect_empty_source_message() {
        assert_eq!("Empty source", ScannerError::EmptySource.to_string());
    }
}
