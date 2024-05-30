use clap::Parser;
use std::process;
use zen::Cli;

fn main() {
    let args: Cli = Cli::parse();
    if let Err(e) = zen::run(args) {
        eprintln!("Application error {e}");
        process::exit(1);
    }
}
