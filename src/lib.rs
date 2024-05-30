use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::{self, Write};

/// A tool to show the word location in a file
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The file to search from
    pub filename: String,

    /// The number of the result to show
    #[arg(short, long)]
    pub number: Option<usize>,

    /// Show word count
    #[arg(short, long)]
    pub count: bool,

    /// Less output
    #[arg(short, long)]
    pub quiet: bool,
}

#[derive(Debug)]
pub struct Location(usize, usize);

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.0, self.1)
    }
}

pub fn run(args: Cli) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(args.filename)?;
    let index = search(&content)?;
    display(&index, args.number, args.count, args.quiet)?;

    Ok(())
}

pub fn search(content: &str) -> Result<HashMap<String, Vec<Location>>, Box<dyn Error>> {
    let re = Regex::new(r"[a-zA-Z]+")?;
    let mut index: HashMap<String, Vec<Location>> = HashMap::new();

    for (line_no, line) in content.lines().enumerate() {
        for find in re.find_iter(line) {
            let word = find.as_str().to_string();
            let column_no = find.start();
            let location = Location(line_no + 1, column_no);
            index.entry(word).or_insert(Vec::new()).push(location);
        }
    }

    Ok(index)
}

pub fn display(
    index: &HashMap<String, Vec<Location>>,
    max_num: Option<usize>,
    count: bool,
    quiet: bool,
) -> Result<(), io::Error> {
    let mut words = Vec::new();
    for (word, positions) in index {
        words.push((positions.len(), word));
    }
    words.sort();

    let mut handle = io::stdout().lock();
    match max_num {
        Some(num) => {
            for (occur, word) in words.iter().rev().take(num) {
                if count && quiet {
                    writeln!(handle, "{} {}", occur, word)?;
                } else if quiet {
                    writeln!(handle, "{}", word)?;
                } else if count {
                    writeln!(handle, "{} {} {:?}", occur, word, index.get(*word).unwrap())?;
                } else {
                    writeln!(handle, "{} {:?}", word, index.get(*word).unwrap())?;
                }
            }
        }
        None => {
            for (occur, word) in words.iter().rev() {
                if count && quiet {
                    writeln!(handle, "{} {}", occur, word)?;
                } else if quiet {
                    writeln!(handle, "{}", word)?;
                } else if count {
                    writeln!(handle, "{} {} {:?}", occur, word, index.get(*word).unwrap())?;
                } else {
                    writeln!(handle, "{} {:?}", word, index.get(*word).unwrap())?;
                }
            }
        }
    }
    Ok(())
}
