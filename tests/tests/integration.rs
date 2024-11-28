use format::formatter::Formatter;
use parser::parser::Parser;
use scanner::scanner::Scanner;

#[test]
fn scan_parse_format() {
    let source = "[]";

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan().unwrap();

    let parser = Parser::new(source, tokens);

    let ast = parser.parse().unwrap();

    let formatter = Formatter::default();
    let json = formatter.format(&ast);

    assert_eq!("[]", json);
}
