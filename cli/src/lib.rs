use std::path::PathBuf;

use clap::{ArgGroup, Parser};

#[derive(Parser, Debug, PartialEq)]
#[command(next_line_help = true)]
#[command(group = ArgGroup::new("action").required(true).args(&["scan", "parse", "format", "minify"]))]
#[command(group = ArgGroup::new("input").required(true).args(&["stdin", "file"]))]
struct Cli {
    #[arg(long)]
    scan: bool,

    #[arg(long)]
    parse: bool,

    #[arg(long)]
    format: bool,

    #[arg(long)]
    minify: bool,

    #[arg(long)]
    stdin: Option<String>,

    #[arg(long)]
    file: Option<PathBuf>,

    #[arg(long, default_value_t = false)]
    output: bool,
}

#[cfg(test)]
mod cli_tests {
    use super::*;

    #[test]
    fn parse_cli_args() {
        let args = Cli::parse_from(vec!["jtools", "--format", "--output", "--stdin", "{}"]);

        assert_eq!(
            Cli {
                scan: false,
                parse: false,
                format: true,
                minify: false,
                stdin: Some("{}".to_string()),
                file: None,
                output: true,
            },
            args
        )
    }
}
