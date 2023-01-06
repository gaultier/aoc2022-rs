use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_line(s: &str) -> [u64; 4] {
    return [0, 0, 0, 0];
}

fn main() {
    let file = File::open("15_sample.txt").unwrap();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let nums = parse_line(&line);
    }
}
