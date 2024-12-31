use clap::Parser as ClapParser;
use cli_args::{CliArgs, Command, Input};
use format::{formatter::Formatter, minifier::Minifier};
use parser::parser::Parser;
use scanner::scanner::Scanner;
use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::{self, Write},
};

pub mod cli_args;

pub struct Cli;

impl Cli {
    pub fn run(&self) -> Result<Option<String>, Box<dyn Error>> {
        let CliArgs { command } = CliArgs::parse();

        match command {
            Command::Parse {
                verify,
                print,
                input,
            } => {
                let source = self.source(&input)?;

                let mut scanner = Scanner::new(&source);
                let tokens = scanner.scan()?;

                let parser = Parser::new(&source, tokens);

                if verify && print {
                    return Ok(Some(parser.is_valid().to_string()));
                }

                parser.parse()?;

                if print {
                    return Ok(Some(source.to_string()));
                }

                Ok(Some("Successfully parsed input".to_string()))
            }
            Command::Format {
                print,
                spacing,
                override_file,
                input,
            } => {
                let source = self.source(&input)?;

                let mut scanner = Scanner::new(&source);
                let tokens = scanner.scan()?;

                let parser = Parser::new(&source, tokens);
                let ast = parser.parse()?;

                let formatter = match spacing {
                    Some(space) => Formatter::new(space as usize),
                    None => Formatter::default(),
                };

                let json = formatter.format(&ast);

                if let true = override_file {
                    self.is_file_then_override(&input, &json)?;
                }

                if print {
                    return Ok(Some(json));
                }

                Ok(Some("Successfully formatted".to_string()))
            }
            Command::Minify {
                print,
                override_file,
                input,
            } => {
                let source = self.source(&input)?;

                let mut scanner = Scanner::new(&source);
                let tokens = scanner.scan()?;

                let parser = Parser::new(&source, tokens);
                let ast = parser.parse()?;

                let minifier = Minifier;
                let json = minifier.minify(&ast);

                if let true = override_file {
                    self.is_file_then_override(&input, &json)?;
                }

                if print {
                    return Ok(Some(json));
                }

                Ok(Some("Successfully minified".to_string()))
            }
        }
    }

    fn source(&self, input_type: &Input) -> Result<String, Box<dyn Error>> {
        match input_type {
            Input::File { path } => {
                match path.extension() {
                    Some(extension) if extension == "json" => {}
                    _ => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "Invalid input only .json files are supported",
                        )
                        .into())
                    }
                }

                fs::read_to_string(&path).map_err(|error| {
                    io::Error::new(
                        error.kind(),
                        format!(
                            "No such file or directory \"{}\" found",
                            path.to_string_lossy()
                        ),
                    )
                    .into()
                })
            }
            Input::Stdin { input } => Ok(input.to_string()),
        }
    }

    fn is_file_then_override(&self, input: &Input, json: &str) -> Result<(), Box<dyn Error>> {
        if let Input::File { path } = input {
            let mut file = OpenOptions::new().write(true).truncate(true).open(&path)?;

            let _ = file.write_all(&json.as_bytes())?;
        }

        Ok(())
    }
}
