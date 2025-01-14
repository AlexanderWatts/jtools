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
    /// Parse an input
    Parse {
        /// Return bool if parsing was successful or not
        #[arg(short, long, default_value_t = false)]
        verify: bool,

        /// Prevent writing input to stdin if successful
        #[arg(short, long, default_value_t = false)]
        prevent_write: bool,

        #[command(subcommand)]
        input: Input,
    },
    /// Format an input
    Format {
        /// Specifiy the number of spaces (0-8) to apply to the input. Default=4
        #[arg(short, long, value_parser = value_parser!(u8).range(0..=8))]
        spacing: Option<u8>,

        /// Prevent writing input to stdin if successful
        #[arg(short, long, default_value_t = false)]
        prevent_write: bool,

        #[command(subcommand)]
        input: Input,
    },
    /// Minify an input
    Minify {
        /// Prevent writing input to stdin if successful
        #[arg(short, long, default_value_t = false)]
        prevent_write: bool,

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
                    prevent_write: false,
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
                    prevent_write: true,
                    input: Input::File {
                        prevent_override: true,
                        path: PathBuf::from("data.json")
                    }
                }
            },
            CliArgs::parse_from(&["", "parse", "-p", "-v", "file", "-p", "data.json"])
        )
    }
}
