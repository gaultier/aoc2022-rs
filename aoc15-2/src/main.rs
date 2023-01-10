use bit_vec::BitVec;
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
    let file = File::open("15_sample.txt").unwrap();
    // let file = File::open("15.txt").unwrap();
    let vectors = BufReader::new(file)
        .lines()
        .map(|l| parse_line(&l.unwrap()))
        .collect::<Vec<_>>();

    let x_min = {
        let x_min = vectors
            .iter()
            .map(|coord| coord[0] - coord[4])
            .min()
            .unwrap();

        if x_min <= 0 {
            0
        } else {
            x_min
        }
    };
    let x_max = {
        let x_max = vectors
            .iter()
            .map(|coord| coord[0] + coord[4])
            .max()
            .unwrap();
        if x_max >= 4000000 {
            4000000
        } else {
            x_max
        }
    };

    const Y: i64 = 2000000;
    let width = i64::abs(x_max - x_min) as usize;
    let mut beacon_xs = BitVec::from_elem(width, false);
    let mut in_sensor_range_xs = BitVec::from_elem(width, false);

    for x in x_min..=x_max {
        for [x_sensor, y_sensor, x_beacon, y_beacon, dist_sensor_beacon] in &vectors {
            if x == *x_beacon && Y == *y_beacon {
                beacon_xs.set(i64::abs(x - x_min) as usize, true);
                break;
            }

            let dist_sensor_point = i64::abs(x_sensor - x) + i64::abs(y_sensor - Y);

            if dist_sensor_point <= *dist_sensor_beacon {
                in_sensor_range_xs.set(i64::abs(x - x_min) as usize, true);
                break;
            }
        }
    }
    in_sensor_range_xs.difference(&beacon_xs);

    println!("{}", in_sensor_range_xs.iter().filter(|x| *x).count());
}
