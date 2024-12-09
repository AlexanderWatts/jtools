use std::ops::ControlFlow;

#[derive(Debug, PartialEq)]
pub struct ErrorDisplay;

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
    fn display() {}
}
