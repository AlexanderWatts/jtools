use std::ops::ControlFlow;

#[derive(Debug, PartialEq)]
pub struct ErrorDisplay;

/// Create an error to be displayed to the user
///
/// ## Description
///
/// Error display is used in the scanner and parser to generate a display message for an error
/// creating a preview of the source input around the start and end of an error where it also
/// marks it starting position.
///
/// The algorithm implemented in `preview` iterates from the start and end of an error within an
/// input to some distance/limit `n`. This had to be done as indexing into an input becomes more
/// difficult with grapheme clusters where you cannot know if an index is pointing to some code
/// point in the middle of a grapheme, which causes an error.
///
/// ## Examples
///
/// ```
/// use error_display::error_display::ErrorDisplay;
///
/// let source = "{ \"error\": bad }";
/// let error_display = ErrorDisplay;
///
/// assert_eq!(
///     "\n  |\n  |\n1 | { \"error\": bad }\n  |            ^\n  |",
///     error_display.preview(source, 11, 13, 1)
/// );
/// ```
///
/// ### Output:
/// ```text
///   |
///   |
/// 1 | { "error": bad }
///   |            ^
///   |
/// ```
impl ErrorDisplay {
    pub fn preview(&self, source: &str, start: usize, current: usize, line: usize) -> String {
        let limit = 15;

        let fold = |acc: usize, char: char| {
            let len = acc + char.len_utf8();

            if len >= limit || char == '\n' {
                ControlFlow::Break(len)
            } else {
                ControlFlow::Continue(len)
            }
        };

        let to = match source[current..].chars().try_fold(0, fold) {
            ControlFlow::Continue(index) | ControlFlow::Break(index) => index,
        };

        let from = match source[..start].chars().try_fold(0, fold) {
            ControlFlow::Continue(index) | ControlFlow::Break(index) => index,
        };

        let line_number_length = line.to_string().len();

        let indicator_position = start - (start - from);

        let preview = format!(
            "{}\n{} | {}^\n",
            &source[start - from..current + to],
            " ".repeat(line_number_length),
            " ".repeat(indicator_position),
        );

        format!(
            "\n {}|\n{} |\n{} | {}{} |",
            " ".repeat(line_number_length),
            " ".repeat(line_number_length),
            line,
            preview,
            " ".repeat(line_number_length),
        )
    }
}

#[cfg(test)]
mod preview_tests {
    use super::*;

    #[test]
    fn display_error() {
        let source = "{ \"error\": bad }";
        let ed = ErrorDisplay;

        assert_eq!(
            "\n  |\n  |\n1 | { \"error\": bad }\n  |            ^\n  |",
            ed.preview(source, 11, 13, 1)
        );
    }
}
