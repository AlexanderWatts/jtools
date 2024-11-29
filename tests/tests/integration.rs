use format::formatter::Formatter;
use parser::parser::Parser;
use scanner::scanner::Scanner;

#[test]
fn scan_parse_format() {
    let source = "[]";

    let mut scanner = Scanner::new(source);

    let tokens = match scanner.scan() {
        Ok(tokens) => tokens,
        Err(scanner_error) => {
            return eprintln!("{}", scanner_error.to_string());
        }
    };

    let parser = Parser::new(source, tokens);

    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(parser_error) => {
            return eprintln!("{}", parser_error.to_string());
        }
    };

    let formatter = Formatter::default();
    let json = formatter.format(&ast);

    assert_eq!("[]", json);
}
