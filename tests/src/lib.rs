//! Integration tests

use core::str;
use format::{formatter::Formatter, minifier::Minifier};
use parser::parser::Parser;
use scanner::scanner::Scanner;
use std::{error::Error, fs};

pub enum Action {
    Scan,
    Parse,
    Format,
    Minify,
}

pub struct Runner;

impl Runner {
    pub fn run(&self, action: Action, source: &str) -> Result<(), Box<dyn Error>> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan()?;

        if let Action::Scan = action {
            return Ok(());
        }

        let parser = Parser::new(source, tokens);
        let ast = parser.parse()?;

        if let Action::Parse = action {
            return Ok(());
        }

        if let Action::Format = action {
            let formatter = Formatter::default();
            formatter.format(&ast);

            return Ok(());
        };

        let minifier = Minifier;
        minifier.minify(&ast);

        Ok(())
    }
}

pub fn read(path: &str) -> String {
    let bytes = fs::read(path).unwrap();
    str::from_utf8(&bytes).unwrap().to_string()
}
