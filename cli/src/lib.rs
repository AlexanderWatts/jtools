use clap::Parser as ClapParser;
use cli_args::{Action, CliArgs};
use format::{formatter::Formatter, minifier::Minifier};
use parser::parser::Parser;
use scanner::scanner::Scanner;
use std::{error::Error, fs, io};

pub mod cli_args;

pub struct Cli;

impl Cli {
    pub fn run(&self) -> Result<Option<String>, Box<dyn Error>> {
        let CliArgs {
            input_type,
            action,
            output,
        } = CliArgs::parse();

        let source = match input_type {
            cli_args::InputType::File { path } => fs::read_to_string(&path).map_err(|error| {
                io::Error::new(
                    error.kind(),
                    format!(
                        "No such file or directory \"{}\" found",
                        path.to_string_lossy()
                    ),
                )
            })?,
            cli_args::InputType::Stdin { input } => input,
        };

        let result = self.pipeline(action, &source)?;

        if output {
            return Ok(Some(result));
        }

        Ok(None)
    }

    fn pipeline(&self, action: Action, source: &str) -> Result<String, Box<dyn Error>> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan()?;

        if let Action::Scan = action {
            let res = tokens
                .iter()
                .map(|token| format!("{}\n", token))
                .collect::<String>();

            return Ok(res);
        }

        let parser = Parser::new(source, tokens);
        let ast = parser.parse()?;

        if let Action::Parse = action {
            return Ok("".to_string());
        }

        if let Action::Format = action {
            let formatter = Formatter::default();
            let json = formatter.format(&ast);

            return Ok(json);
        } else {
            let minifier = Minifier;
            let json_minified = minifier.minify(&ast);

            return Ok(json_minified);
        }
    }
}
