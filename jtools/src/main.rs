use cli::Cli;
use std::io::Error;

fn main() -> Result<(), Error> {
    Cli.run()
}
