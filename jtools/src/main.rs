use cli::Cli;

fn main() {
    match Cli.run() {
        Ok(data) => println!("{:?}", data),
        Err(error) => eprintln!("{}", error),
    }
}
