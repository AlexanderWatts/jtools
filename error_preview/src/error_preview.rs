use unicode_width::UnicodeWidthStr;

#[derive(Debug, PartialEq)]
pub struct ErrorPreview;

/// Create an error to be displayed to the user
///
/// ## Description
///
/// Error preview is used in the scanner and parser to generate a display message for an error
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
/// use error_preview::error_preview::ErrorPreview;
///
/// let source = "{ \"error\": bad }";
/// let error_preview = ErrorPreview;
///
/// assert_eq!(
///     "\n  |\n  |\n1 |{ \"error\": bad }\n  |           ^---Column=12\n  |",
///     error_preview.preview(source, 11, 12, 1)
/// );
/// ```
///
/// ### Output:
/// ```text
///   |
///   |
/// 1 |{ "error": bad }
///   |           ^---Column=12
///   |
/// ```
impl ErrorPreview {
    pub fn preview(
        &self,
        source: &str,
        start: usize,
        column_start: usize,
        line_number: usize,
    ) -> String {
        let limit = 32;

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

        let line_number_width = line_number.to_string().len();
        let indent = " ".repeat(line_number_width);

        let above_sign = self.sign(&mut backwards.lines().rev());
        let below_sign = self.sign(&mut forwards.lines());

        let error_preview = format!("{}{}", back_preview, forward_preview);

        let pointer = format!("^---Column={}", column_start);
        let pointer_position = " ".repeat(back_preview.width());

        [
            format!("\n"),
            format!("{indent}{above_sign}|\n"),
            format!("{indent} |\n"),
            format!("{line_number} |{error_preview}\n"),
            format!("{indent} |{pointer_position}{pointer}\n"),
            format!("{indent}{below_sign}|"),
        ]
        .into_iter()
        .collect::<String>()
    }

    fn sign(&self, lines: &mut impl Iterator) -> &str {
        lines.next();
        lines.next().is_some().then(|| "+").unwrap_or(" ")
    }
}

#[cfg(test)]
mod preview_tests {
    use super::*;

    #[test]
    fn display_error() {
        let source = "{ \"error\": bad }";
        let ep = ErrorPreview;

        assert_eq!(
            "\n  |\n  |\n1 |{ \"error\": bad }\n  |           ^---Column=12\n  |",
            ep.preview(source, 11, 12, 1)
        );
    }
}
