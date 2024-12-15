use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Subcommand, Debug, PartialEq)]
pub enum InputType {
    File { path: PathBuf },
    Stdin { input: String },
}

#[derive(ValueEnum, Debug, PartialEq, Clone)]
pub enum Action {
    Parse,
    Format,
    Minify,
}

#[derive(Parser, Debug, PartialEq)]
pub struct CliArgs {
    #[arg(value_enum, default_value_t=Action::Format)]
    pub action: Action,

    #[arg(short, long, default_value_t = false)]
    pub output: bool,

    #[command(subcommand)]
    pub input_type: InputType,
}

#[cfg(test)]
mod cli_args_tests {
    use super::*;

    #[test]
    fn output_set_to_true() {
        assert_eq!(
            CliArgs {
                action: Action::Parse,
                input_type: InputType::Stdin {
                    input: "{}".to_string()
                },
                output: true,
            },
            CliArgs::parse_from(&["", "parse", "-o", "stdin", "{}"])
        )
    }

    #[test]
    fn parse_stdin() {
        assert_eq!(
            CliArgs {
                action: Action::Parse,
                input_type: InputType::Stdin {
                    input: "{}".to_string()
                },
                output: false,
            },
            CliArgs::parse_from(&["", "parse", "stdin", "{}"])
        )
    }

    #[test]
    fn defaults_to_format_action_if_none_given() {
        assert_eq!(
            CliArgs {
                action: Action::Format,
                input_type: InputType::File {
                    path: PathBuf::from("data.json")
                },
                output: false,
            },
            CliArgs::parse_from(&["", "file", "data.json"])
        )
    }
}
