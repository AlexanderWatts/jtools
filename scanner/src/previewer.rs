use crate::scanner::Scanner;

pub trait Visitor {
    fn visit_scanner(&self, scanner: &Scanner) -> String;
}

pub struct Previewer;

impl Visitor for Previewer {
    fn visit_scanner(&self, scanner: &Scanner) -> String {
        "preview".to_string()
    }
}

#[cfg(test)]
mod preview_tests {
    use super::*;

    #[test]
    fn preview_scanner() {
        let source = "[true]";
        let s = Scanner::new(source);

        let p = Previewer;

        assert_eq!("preview", p.visit_scanner(&s))
    }
}
