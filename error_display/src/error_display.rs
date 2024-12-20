use unicode_width::UnicodeWidthStr;

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
///     "\n  |\n  |\n1 |{ \"error\": bad }\n  |           ^___\n  |",
///     error_display.preview(source, 11, 13, 1)
/// );
/// ```
///
/// ### Output:
/// ```text
///   |
///   |
/// 1 | { "error": bad }
///   |            ^___
///   |
/// ```
impl ErrorDisplay {
    pub fn preview(&self, source: &str, start: usize, current: usize, line: usize) -> String {
        let limit = 42;
        let line_number_width = line.to_string().len();

        let (backwards, forwards) = source.split_at(start);

        let back_preview = backwards
            .chars()
            .rev()
            .take_while(|&char| char != '\n')
            .take(limit)
            .collect::<Vec<char>>()
            .into_iter()
            .rev()
            .collect::<String>();

        let forward_preview = forwards
            .chars()
            .take_while(|&char| char != '\n')
            .take(limit)
            .collect::<String>();

        let back_preview = back_preview.trim_start();
        let forward_preview = forward_preview.trim_end();

        let error_line = format!("{}{}", back_preview, forward_preview);

        let pointer_line = format!(
            "{}\n{} |{}^___",
            error_line,
            " ".repeat(line_number_width),
            " ".repeat(back_preview.width())
        );

        let error = format!(
            "\n{}{}|\n{} |\n{} |{}\n{}{}|",
            " ".repeat(line_number_width),
            self.is_surrounding_line(&mut backwards.lines().rev())
                .then(|| "+")
                .unwrap_or(" "),
            " ".repeat(line_number_width),
            line,
            pointer_line,
            " ".repeat(line_number_width),
            self.is_surrounding_line(&mut forwards.lines())
                .then(|| "+")
                .unwrap_or(" ")
        );

        error
    }

    fn is_surrounding_line(&self, lines: &mut impl Iterator) -> bool {
        lines.next();
        lines.next().is_some()
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
            "\n  |\n  |\n1 |{ \"error\": bad }\n  |           ^___\n  |",
            ed.preview(source, 11, 13, 1)
        );
    }
}
