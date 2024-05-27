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
    pub number: Option<i32>,

    /// Show word count
    #[arg(short, long)]
    pub count: bool,
}

pub fn search(args: &Cli) -> Result<HashMap<String, Vec<(usize, usize)>>, io::Error> {
    let re = Regex::new(r"(\w+)").unwrap();
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

pub fn display(index: &HashMap<String, Vec<(usize, usize)>>, max_num: Option<usize>) {
    let mut count = Vec::new();
    for (word, positions) in index {
        count.push((positions.len(), word));
    }
    count.sort();
    match max_num {
        Some(num) => {
            for (_, word) in count.iter().rev().take(num) {
                println!("{} {:?}", word, index.get(*word).unwrap());
            }
        }
        None => {
            for (_, word) in count.iter().rev() {
                println!("{} {:?}", word, index.get(*word).unwrap());
            }
        }
    }
}
