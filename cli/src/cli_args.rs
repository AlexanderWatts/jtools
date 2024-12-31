use std::path::PathBuf;

use clap::{value_parser, ArgAction, Parser, Subcommand};

#[derive(Subcommand, Debug, PartialEq)]
pub enum Input {
    /// File path
    File {
        path: PathBuf,

        /// Prevent file from being overriden with either formatted or minified output
        #[arg(short, long, default_value_t = false, action = ArgAction::SetTrue)]
        prevent_override: bool,
    },
    /// Text input
    Text { input: String },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Command {
    Parse {
        /// Return bool if parsing was successful or not
        #[arg(short, long, default_value_t = false)]
        verify: bool,

        /// Write input to stdin if successful
        #[arg(short, long, default_value_t = false)]
        write: bool,

        #[command(subcommand)]
        input: Input,
    },
    Format {
        /// Specifiy the number of spaces (0-8) to apply to the input. Default=4
        #[arg(short, long, value_parser = value_parser!(u8).range(0..=8))]
        spacing: Option<u8>,

        /// Write formatted input to stdin if successful
        #[arg(short, long, default_value_t = false)]
        write: bool,

        #[command(subcommand)]
        input: Input,
    },
    Minify {
        /// Write minified input to stdin if successful
        #[arg(short, long, default_value_t = false)]
        write: bool,

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
                    write: false,
                    input: Input::File {
                        prevent_override: false,
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
                    write: true,
                    input: Input::File {
                        prevent_override: true,
                        path: PathBuf::from("data.json")
                    }
                }
            },
            CliArgs::parse_from(&["", "parse", "-w", "-v", "file", "-p", "data.json"])
        )
    }
}
