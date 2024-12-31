use clap::Parser as ClapParser;
use cli_args::{CliArgs, Command, Input};
use format::{formatter::Formatter, minifier::Minifier};
use parser::parser::Parser;
use scanner::scanner::Scanner;
use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::{self, stderr, stdout, Write},
};

pub mod cli_args;

pub struct Cli;

impl Cli {
    pub fn run(&self) -> Result<(), io::Error> {
        let CliArgs { command } = CliArgs::parse();

        match self.process_command(command) {
            Ok(data) => writeln!(stdout(), "{}", data),
            Err(error) => writeln!(stderr(), "{}", error),
        }
    }

    fn process_command(&self, command: Command) -> Result<String, Box<dyn Error>> {
        match command {
            Command::Parse {
                verify,
                write,
                input,
            } => {
                let source = self.source(&input)?;

                let mut scanner = Scanner::new(&source);
                let tokens = scanner.scan()?;

                let parser = Parser::new(&source, tokens);

                if verify && write {
                    return Ok(parser.is_valid().to_string());
                }

                parser.parse()?;

                if write {
                    return Ok(source.to_string());
                }

                Ok("Parse successful".to_string())
            }
            Command::Format {
                write,
                spacing,
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

                self.is_file_then_override(&input, &json)?;

                if write {
                    return Ok(json);
                }

                Ok("Format successful".to_string())
            }
            Command::Minify { write, input } => {
                let source = self.source(&input)?;

                let mut scanner = Scanner::new(&source);
                let tokens = scanner.scan()?;

                let parser = Parser::new(&source, tokens);
                let ast = parser.parse()?;

                let minifier = Minifier;
                let json = minifier.minify(&ast);

                self.is_file_then_override(&input, &json)?;

                if write {
                    return Ok(json);
                }

                Ok("Minify successful".to_string())
            }
        }
    }

    fn source(&self, input_type: &Input) -> Result<String, Box<dyn Error>> {
        match input_type {
            Input::File { path, .. } => {
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
            Input::Text { input } => Ok(input.to_string()),
        }
    }

    fn is_file_then_override(&self, input: &Input, json: &str) -> Result<(), Box<dyn Error>> {
        if let Input::File {
            path,
            prevent_override: false,
        } = input
        {
            let mut file = OpenOptions::new().write(true).truncate(true).open(&path)?;

            let _ = file.write_all(&json.as_bytes())?;
        }

        Ok(())
    }
}
