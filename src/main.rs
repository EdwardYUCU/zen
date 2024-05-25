use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use zen::Cli;

#[allow(unused)]
fn main() {
    let args: Cli = Cli::parse();
    let re = Regex::new(r"(\w+)").unwrap();
    let mut index: HashMap<String, Vec<(usize, usize)>> = HashMap::new();

    let content = fs::read_to_string(&args.filename).expect("Faile tot read to string");

    for (line_no, line) in content.lines().enumerate() {
        for capture in re.captures_iter(line) {
            let r#match = capture.get(1).unwrap();
            let word = r#match.as_str().to_string();
            let column_no = r#match.start();
            let location = (line_no + 1, column_no);
            index.entry(word).or_insert(Vec::new()).push(location);
        }
    }
}
