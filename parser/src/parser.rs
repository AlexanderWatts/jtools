use token::token::Token;

#[derive(Debug, PartialEq)]
pub struct Parser<'source> {
    current: usize,
    tokens: Vec<Token<'source>>,
}

impl<'source> Parser<'source> {
    pub fn new(tokens: Vec<Token<'source>>) -> Self {
        Self { current: 0, tokens }
    }

    pub fn next(&mut self) -> Option<&Token> {
        let current = self.tokens.get(self.current);

        if current.is_some() {
            self.current += 1;
        }

        current
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }
}

#[cfg(test)]
mod parser_tests {
    use token::token_type::TokenType;

    use super::*;

    #[test]
    fn consume_next_until_end() {
        let mut p = Parser::new(vec![
            Token::new(TokenType::True, "true", 1, 1, 5),
            Token::new(TokenType::False, "false", 1, 5, 11),
        ]);

        p.next();
        p.next();

        assert_eq!(None, p.next());
    }

    #[test]
    fn next_is_some() {
        let mut p = Parser::new(vec![Token::new(TokenType::True, "true", 1, 1, 5)]);
        assert_eq!(
            Some(&Token::new(TokenType::True, "true", 1, 1, 5)),
            p.next()
        );
        assert_eq!(1, p.current);
        assert_eq!(None, p.next());
    }

    #[test]
    fn next_is_none() {
        let mut p = Parser::new(vec![]);
        assert_eq!(None, p.next());
        assert_eq!(0, p.current);
    }

    #[test]
    fn peek_is_none() {
        let p = Parser::new(vec![]);
        assert_eq!(None, p.peek());
    }

    #[test]
    fn peek_is_some() {
        let p = Parser::new(vec![Token::new(TokenType::True, "true", 1, 1, 5)]);

        assert_eq!(
            Some(&Token::new(TokenType::True, "true", 1, 1, 5)),
            p.peek()
        );
    }

    #[test]
    fn create_new_parser() {
        let p = Parser::new(vec![Token::new(TokenType::True, "true", 1, 1, 5)]);

        assert_eq!(
            Parser {
                current: 0,
                tokens: vec![Token::new(TokenType::True, "true", 1, 1, 5)]
            },
            p
        );
    }
}
