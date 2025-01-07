use std::{error::Error, fmt::Display};

use error_display::error_display::ErrorDisplay;

#[derive(Debug, PartialEq)]
pub struct Preview<'a> {
    pub source: &'a str,
    pub start: usize,
    pub column_start: usize,
    pub line: usize,
}

impl<'source> Preview<'source> {
    pub fn new(source: &'source str, start: usize, column_start: usize, line: usize) -> Self {
        Self {
            source,
            start,
            column_start,
            line,
        }
    }

    fn preview(&self) -> String {
        let error_display = ErrorDisplay;

        error_display.preview(self.source, self.start, self.column_start, self.line)
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorType {
    UnterminatedString,
    InvalidUnicodeSequence,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnterminatedString => write!(f, "Unterminated string"),
            Self::InvalidUnicodeSequence => write!(f, "Invalid unicode sequence"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SError<'source> {
    pub error_type: ErrorType,
    pub preview: Preview<'source>,
}

impl<'source> SError<'source> {
    pub fn new(error_type: ErrorType, preview: Preview<'source>) -> Self {
        Self {
            error_type,
            preview,
        }
    }
}

impl<'source> Error for SError<'source> {}

impl<'source> Display for SError<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.error_type, self.preview.preview())
    }
}
