use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_line(s: &str) -> [i64; 5] {
    let mut nums = s
        .split([',', ':', '='])
        .filter_map(|s| s.parse::<i64>().ok());

    let x_sensor = nums.next().unwrap();
    let y_sensor = nums.next().unwrap();
    let x_beacon = nums.next().unwrap();
    let y_beacon = nums.next().unwrap();
    let dist_sensor_beacon = i64::abs(x_sensor - x_beacon) + i64::abs(y_sensor - y_beacon);

    [x_sensor, y_sensor, x_beacon, y_beacon, dist_sensor_beacon]
}

fn main() {
    let file = File::open("15.txt").unwrap();
    let vectors = BufReader::new(file)
        .lines()
        .map(|l| parse_line(&l.unwrap()))
        .collect::<Vec<_>>();

    let x_sensor_min = vectors.iter().map(|coord| coord[0]).min().unwrap();
    let x_sensor_max = vectors.iter().map(|coord| coord[0]).max().unwrap();
    let y_sensor_min = vectors.iter().map(|coord| coord[1]).min().unwrap();
    let y_sensor_max = vectors.iter().map(|coord| coord[1]).max().unwrap();

    let x_beacon_min = vectors.iter().map(|coord| coord[2]).min().unwrap();
    let x_beacon_max = vectors.iter().map(|coord| coord[2]).max().unwrap();
    let y_beacon_min = vectors.iter().map(|coord| coord[3]).min().unwrap();
    let y_beacon_max = vectors.iter().map(|coord| coord[3]).max().unwrap();

    let x_min = i64::min(x_sensor_min, x_beacon_min);
    let y_min = i64::min(y_sensor_min, y_beacon_min);
    let x_max = i64::max(x_sensor_max, x_beacon_max);
    let y_max = i64::max(y_sensor_max, y_beacon_max);
    println!("({}, {}) ({}, {})", x_min, y_min, x_max, y_max);

    let mut covered_points = BTreeSet::new();
    let mut visited = 0i64;
    let mut count = 0u64;
    for [_x_sensor, _y_sensor, x_beacon, y_beacon, dist_sensor_beacon] in &vectors {
        for x in (x_beacon - dist_sensor_beacon)..(x_beacon + dist_sensor_beacon) {
            for y in (y_beacon - dist_sensor_beacon)..(y_beacon + dist_sensor_beacon) {
                visited += 1;
                if covered_points.insert([x, y]) {
                    count += 1;
                }
            }
        }
        println!("[D001] {}/{}", count, visited);
    }
    //    let mut visited = 0u64;
    //    let mut count = 0i64;
    //    for y in y_min..=y_max {
    //        for x in x_min..=x_max {
    //            visited += 1;
    //            for [x_sensor, y_sensor, _x_beacon, _y_beacon, dist_sensor_beacon] in &vectors {
    //                let dist_sensor_point = i64::abs(x_sensor - x) + i64::abs(y_sensor - y);
    //
    //                if &dist_sensor_point <= dist_sensor_beacon {
    //                    count += 1;
    //                    break;
    //                }
    //            }
    //        }
    //    }
    println!("{}/{}", count, visited);
}
