use format::formatter::Formatter;
use parser::parser::Parser;
use token::{token::Token, token_type::TokenType};

#[test]
fn parse_and_format() {
    let source = "[]";

    let tokens = vec![
        Token::new(TokenType::LeftBracket, 1, (0, 1), (1, 2)),
        Token::new(TokenType::RightBracket, 1, (1, 2), (2, 3)),
    ];

    let parser = Parser::new(source, tokens);

    let ast = parser.parse().unwrap();

    let formatter = Formatter::default();
    let json = formatter.format(&ast);

    assert_eq!("[]", json);
}
