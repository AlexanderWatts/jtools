use std::path::PathBuf;

use clap::{value_parser, ArgAction, Parser, Subcommand};

#[derive(Subcommand, Debug, PartialEq)]
pub enum Input {
    File { path: PathBuf },
    Stdin { input: String },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Command {
    Parse {
        #[arg(short, long, default_value_t = false)]
        verify: bool,

        #[arg(short, long, default_value_t = false)]
        print: bool,

        #[command(subcommand)]
        input: Input,
    },
    Format {
        #[arg(short, long, value_parser = value_parser!(u8).range(0..=8))]
        spacing: Option<u8>,

        #[arg(short, long, default_value_t = false)]
        print: bool,

        #[arg(short, long, default_value_t = true, action = ArgAction::SetFalse)]
        override_file: bool,

        #[command(subcommand)]
        input: Input,
    },
    Minify {
        #[arg(short, long, default_value_t = false)]
        print: bool,

        #[arg(short, long, default_value_t = true, action = ArgAction::SetFalse)]
        override_file: bool,

        #[command(subcommand)]
        input: Input,
    },
}

#[derive(Parser, Debug, PartialEq)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[cfg(test)]
mod cli_args_tests {
    use super::*;

    #[test]
    fn format_with_spacing() {
        assert_eq!(
            CliArgs {
                command: Command::Format {
                    spacing: Some(8),
                    print: false,
                    override_file: true,
                    input: Input::File {
                        path: PathBuf::from("data.json")
                    }
                }
            },
            CliArgs::parse_from(&["", "format", "-s", "8", "file", "data.json"])
        )
    }

    #[test]
    fn parse_print_and_verify() {
        assert_eq!(
            CliArgs {
                command: Command::Parse {
                    verify: true,
                    print: true,
                    input: Input::File {
                        path: PathBuf::from("data.json")
                    }
                }
            },
            CliArgs::parse_from(&["", "parse", "-p", "-v", "file", "data.json"])
        )
    }
}
