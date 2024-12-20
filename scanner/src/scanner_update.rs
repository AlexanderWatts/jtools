use std::iter::Peekable;

use unicode_segmentation::{GraphemeIndices, UnicodeSegmentation};

pub struct Scanner<'source> {
    source: &'source str,
    unicode: Peekable<GraphemeIndices<'source>>,
    start: usize,
    current: usize,
    line: usize,
    column_start: usize,
    column_end: usize,
}

impl<'source> Scanner<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            unicode: source.grapheme_indices(true).peekable(),
            start: 0,
            current: 0,
            line: 0,
            column_start: 1,
            column_end: 1,
        }
    }
}

#[cfg(test)]
mod scanner_tests {
    use super::*;
}
