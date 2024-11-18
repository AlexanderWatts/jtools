#[derive(Debug, PartialEq)]
pub struct Formatter {
    depth: usize,
    space: usize,
}

impl Default for Formatter {
    fn default() -> Self {
        Self { depth: 0, space: 4 }
    }
}

impl Formatter {
    pub fn new(space: usize) -> Self {
        Self { depth: 0, space }
    }
}

#[cfg(test)]
mod format_tests {
    use super::*;

    #[test]
    fn create_formatter() {
        assert_eq!(Formatter { depth: 0, space: 2 }, Formatter::new(2));
    }

    #[test]
    fn create_default_formatter() {
        assert_eq!(Formatter { depth: 0, space: 4 }, Formatter::default());
    }
}
