//! Integration tests

pub use format::formatter::Formatter;
pub use parser::parser::Parser;
pub use scanner::scanner::Scanner;

#[test]
fn scan_parse_format() {
    let source = "[]";

    let mut scanner = Scanner::new(source);
    let tokens = match scanner.scan() {
        Ok(tokens) => tokens,
        Err(error) => {
            return eprintln!("{}", error);
        }
    };

    let parser = Parser::new(source, tokens);
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
