use crate::scanner::Scanner;

pub trait Visitor {
    fn visit_scanner(&self, scanner: &Scanner) -> String;
}

pub struct Previewer;

impl Previewer {
    pub fn preview(&self, source: &str, start: usize, current: usize) -> String {
        source[start..current].to_string()
    }
}

impl Visitor for Previewer {
    fn visit_scanner(&self, scanner: &Scanner) -> String {
        self.preview(scanner.source, scanner.start, scanner.current)
    }
}

#[cfg(test)]
mod preview_tests {
    use super::*;

    #[test]
    fn visit_scanner() {
        let source = "[true]";
        let s = Scanner::new(source);

        let p = Previewer;

        assert_eq!("", p.visit_scanner(&s))
    }
}
