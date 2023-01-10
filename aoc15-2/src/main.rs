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
    // let file = File::open("15_sample.txt").unwrap();
    let file = File::open("15.txt").unwrap();
    let vectors = BufReader::new(file)
        .lines()
        .map(|l| parse_line(&l.unwrap()))
        .collect::<Vec<_>>();

    // const MAX: i64 = 20;
    const MAX: i64 = 4000000;

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

    let width = i64::abs(x_max - x_min) as usize;
    let height = i64::abs(y_max - y_min) as usize;
    let mut covered = BitVec::from_elem((width + 1) * (height + 1), false);

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let dx = (x - x_min) as usize;
            let dy = (y - y_min) as usize;
            let pos = dy * width + dx;
            // println!(
            //     "[D001] x_min={} x_max={} y_min={} y_max={} pos={} x={} y={} dx={} dy={}",
            //     x_min, x_max, y_min, y_max, pos, x, y, dx, dy
            // );
            if covered.get(pos).unwrap() {
                continue;
            }
            for [x_sensor, y_sensor, x_beacon, y_beacon, dist_sensor_beacon] in &vectors {
                if x == *x_beacon && y == *y_beacon {
                    covered.set(pos, true);
                    break;
                }

                let dist_sensor_point = i64::abs(x_sensor - x) + i64::abs(y_sensor - y);
                if dist_sensor_point <= *dist_sensor_beacon {
                    covered.set(pos, true);
                    break;
                }
            }
        }
    }
    // println!("{:?}", covered);
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let dx = (x - x_min) as usize;
            let dy = (y - y_min) as usize;
            let pos = dy * width + dx;

            if x < 0 || x > MAX || y < 0 || y > MAX {
                continue;
            }
            if covered.get(pos).unwrap() {
                continue;
            }

            println!("x={} y={}", x, y);
        }
    }

    // println!("{}", in_sensor_range_xs.iter().filter(|x| *x).count());
}
