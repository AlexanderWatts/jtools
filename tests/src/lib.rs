//! Integration tests

pub use format::formatter::Formatter;
pub use parser::parser::Parser;
pub use scanner::scanner::Scanner;

#[test]
fn scan_parse_format() {
    let source = "[]";

    let parser = Parser::new(source);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(error) => {
            return eprintln!("{}", error);
        }
    };

    let formatter = Formatter::default();
    let json = formatter.format(&ast);

    assert_eq!("[]", json);
}
