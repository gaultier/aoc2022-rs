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
    let file = File::open("15.txt").unwrap();
    let mut nums = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        let coords = parse_line(&line);
        println!("{:?}", coords);

        nums.push(coords);
    }
}
