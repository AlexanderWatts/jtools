use std::io::{stdout, Error, Write};

use cli::Cli;

fn main() -> Result<(), Error> {
    match Cli.run() {
        Ok(None) => Ok(()),
        Ok(Some(data)) => writeln!(stdout(), "{}", data),
        Err(error) => writeln!(stdout(), "{}", error),
    }
}
