use clap::Parser;
use zen::{display, search, Cli};

fn main() {
    let args: Cli = Cli::parse();
    match search(&args) {
        Err(e) => eprintln!("Error: {}", e),
        Ok(index) => display(&index, args.number, args.count, args.quiet),
    }
}
