use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The file to search from
    pub filename: String,

    /// The number of the result to show
    #[arg(short, long)]
    pub number: i32,
}
