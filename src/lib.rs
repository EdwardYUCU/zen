use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io;

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

pub fn search(args: &Cli) -> Result<HashMap<String, Vec<(usize, usize)>>, io::Error> {
    let re = Regex::new(r"[a-zA-Z_]+").unwrap();
    let mut index: HashMap<String, Vec<(usize, usize)>> = HashMap::new();

    let content = fs::read_to_string(&args.filename)?;

    for (line_no, line) in content.lines().enumerate() {
        for find in re.find_iter(line) {
            let word = find.as_str().to_string();
            let column_no = find.start();
            let location = (line_no + 1, column_no);
            index.entry(word).or_insert(Vec::new()).push(location);
        }
    }

    Ok(index)
}

pub fn display(
    index: &HashMap<String, Vec<(usize, usize)>>,
    max_num: Option<usize>,
    count: bool,
    quiet: bool,
) {
    let mut words = Vec::new();
    for (word, positions) in index {
        words.push((positions.len(), word));
    }
    words.sort();
    match max_num {
        Some(num) => {
            for (occur, word) in words.iter().rev().take(num) {
                if count && quiet {
                    println!("{} {}", occur, word);
                } else if quiet {
                    println!("{}", word);
                } else if count {
                    println!("{} {} {:?}", occur, word, index.get(*word).unwrap());
                } else {
                    println!("{} {:?}", word, index.get(*word).unwrap());
                }
            }
        }
        None => {
            for (occur, word) in words.iter().rev() {
                if count && quiet {
                    println!("{} {}", occur, word);
                } else if quiet {
                    println!("{}", word);
                } else if count {
                    println!("{} {} {:?}", occur, word, index.get(*word).unwrap());
                } else {
                    println!("{} {:?}", word, index.get(*word).unwrap());
                }
            }
        }
    }
}
