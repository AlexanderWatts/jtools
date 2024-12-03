pub trait Visitor {
    fn visit(&self, source: &str, start: usize, current: usize) -> String;
}

pub trait Client {
    fn accept(&self, visitor: &impl Visitor) -> String;
}

#[derive(Debug)]
pub struct ErrorDisplay;

impl ErrorDisplay {
    pub fn preview(&self, source: &str, start: usize, current: usize) -> String {
        source[start..current].to_string()
    }
}

impl Visitor for ErrorDisplay {
    fn visit(&self, source: &str, start: usize, current: usize) -> String {
        self.preview(source, start, current)
    }
}

#[cfg(test)]
mod preview_tests {
    use super::*;

    #[test]
    fn visit_scanner() {
        let source = "[true]";

        let p = ErrorDisplay;

        assert_eq!("true", p.visit(source, 1, 5))
    }
}
