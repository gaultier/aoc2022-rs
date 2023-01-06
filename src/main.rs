use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_line(s: &str) -> [i64; 4] {
    let mut nums = s
        .split([',', ':', '='])
        .filter_map(|s| s.parse::<i64>().ok());

    [
        nums.next().unwrap(),
        nums.next().unwrap(),
        nums.next().unwrap(),
        nums.next().unwrap(),
    ]
}

fn main() {
    let file = File::open("15_sample.txt").unwrap();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let nums = parse_line(&line);
        println!("{:?}", nums);
    }
}
