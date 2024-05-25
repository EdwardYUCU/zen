use clap::Parser;

/// A tool to show the word info in a file
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The file to search from
    pub filename: String,

    /// The number of the result to show
    /// sort the result in decending order a show the first n entries.
    #[arg(short, long)]
    pub number: Option(i32),
}
