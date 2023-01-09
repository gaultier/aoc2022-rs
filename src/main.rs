use std::collections::HashSet;
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

    let x_min = vectors
        .iter()
        .map(|coord| coord[0] - coord[4])
        .min()
        .unwrap();
    let x_max = vectors
        .iter()
        .map(|coord| coord[0] + coord[4])
        .max()
        .unwrap();

    //const Y: i64 = 10;
    const Y: i64 = 2000000;

    let mut beacon_xs = HashSet::new();
    let mut in_sensor_range_xs = HashSet::new();

    for x in x_min..=x_max {
        for [x_sensor, y_sensor, x_beacon, y_beacon, dist_sensor_beacon] in &vectors {
            if x == *x_beacon && Y == *y_beacon {
                beacon_xs.insert(x);
                continue;
            }

            let dist_sensor_point = i64::abs(x_sensor - x) + i64::abs(y_sensor - Y);

            if dist_sensor_point <= *dist_sensor_beacon {
                in_sensor_range_xs.insert(x);
                // println!("[D003] {} {}", x, count);
                break;
            }
        }
    }
    let mut count = 0usize;
    for x in x_min..=x_max {
        if beacon_xs.get(&x).is_some() {
            continue;
        }
        if in_sensor_range_xs.get(&x).is_some() {
            count += 1;
        }
    }
    println!("{}", count);
}
