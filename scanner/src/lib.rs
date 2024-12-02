pub mod previewer;
pub mod scanner;
pub mod scanner_error;

#[derive(Debug, PartialEq)]
enum CustomError {
    Test { preview: String },
}
impl std::error::Error for CustomError {}
impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Test { preview } => write!(f, "{}", preview),
        }
    }
}

struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
    column_start: usize,
    column_end: usize,
}

impl<'a> Scanner<'a> {
    fn accept(&self, visitor: &impl Visitor) -> String {
        visitor.visit_scanner(self)
    }

    fn scan(&mut self) -> Result<(), CustomError> {
        let prev = Previewer;
        Err(CustomError::Test {
            preview: self.accept(&prev),
        })
    }
}

struct Parser<'a> {
    source: &'a str,
}

impl<'a> Parser<'a> {
    fn accept(&self, visitor: &impl Visitor) -> String {
        visitor.visit_parser(self)
    }
}

trait Visitor {
    fn visit_scanner(&self, scanner: &Scanner) -> String;
    fn visit_parser(&self, parser: &Parser) -> String;
}

struct Previewer;

impl Previewer {
    fn preview(&self, source: &str, line: usize, start: usize, end: usize) -> String {
        source[start..end].to_string()
    }
}

impl Visitor for Previewer {
    fn visit_scanner(&self, scanner: &Scanner) -> String {
        self.preview(scanner.source, scanner.line, scanner.start, scanner.current)
    }

    fn visit_parser(&self, parser: &Parser) -> String {
        //self.preview(parser.source)
        "".to_string()
    }
}

#[test]
fn preview_test() {
    let source = "{hello}";

    let prev = Previewer;

    let mut s = Scanner {
        source,
        start: 1,
        current: 6,
        column_start: 2,
        column_end: 7,
        line: 1,
    };
    assert_eq!("hello", s.accept(&prev));
    assert_eq!(
        Err(CustomError::Test {
            preview: "hello".to_string()
        }),
        s.scan()
    );

    let p = Parser { source };
    assert_eq!("", p.accept(&prev));
}
