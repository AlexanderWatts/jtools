use clap::Parser as ClapParser;
use cli_args::{Action, CliArgs};
use format::{formatter::Formatter, minifier::Minifier};
use parser::parser::Parser;
use scanner::scanner::Scanner;
use std::{error::Error, fs};

pub mod cli_args;

pub struct Cli;

impl Cli {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let CliArgs {
            input_type,
            action,
            output,
        } = CliArgs::parse();

        let source = match input_type {
            cli_args::InputType::File { path } => fs::read_to_string(path)?,
            cli_args::InputType::Stdin { input } => input,
        };

        self.pipeline(action, &source)
    }

    fn pipeline(&self, action: Action, source: &str) -> Result<(), Box<dyn Error>> {
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
            let _ = formatter.format(&ast);

            return Ok(());
        } else {
            let minifier = Minifier;
            let _ = minifier.minify(&ast);

            return Ok(());
        }
    }
}
