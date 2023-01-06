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
    let y_min = vectors
        .iter()
        .map(|coord| coord[1] - coord[4])
        .min()
        .unwrap();
    let y_max = vectors
        .iter()
        .map(|coord| coord[1] + coord[4])
        .max()
        .unwrap();
    println!("({}, {}) ({}, {})", x_min, y_min, x_max, y_max);

    // let mut overlapping_points = BTreeSet::new();
    let mut visited = 0u64;
    let mut count = 0u64;
    const Y: i64 = 10; //2000000;
                       // for [x_sensor, y_sensor, x_beacon, y_beacon, dist_sensor_beacon] in &vectors {
                       //     for x in (x_beacon - dist_sensor_beacon)..=(x_beacon + dist_sensor_beacon) {
                       //         if ((y_beacon - dist_sensor_beacon)..=(y_beacon + dist_sensor_beacon)).contains(&Y) {
                       //             // println!(
                       //             //     "[D002] {} {} {} {} {}",
                       //             //     x_sensor, y_sensor, x_beacon, y_beacon, dist_sensor_beacon
                       //             // );

    //             println!("{}", x);
    //             visited += 1;
    //             if overlapping_points.insert([x, Y]) {
    //                 count += 1;
    //             }
    //         }
    //     }
    // }

    //    let mut visited = 0u64;
    //    let mut count = 0i64;
    //    for y in y_min..=y_max {
    for x in x_min..=x_max {
        visited += 1;
        for [x_sensor, y_sensor, x_beacon, y_beacon, dist_sensor_beacon] in &vectors {
            if x == *x_beacon && Y == *y_beacon {
                continue;
            }

            let dist_sensor_point = i64::abs(x_sensor - x) + i64::abs(y_sensor - Y);

            if dist_sensor_point <= *dist_sensor_beacon {
                count += 1;
                // println!("[D003] {} {}", x, count);
                break;
            }
        }
    }
    //    }
    println!("{}/{}", count as usize, visited);
}
