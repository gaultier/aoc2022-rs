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
    // const MAX: i64 = 20;

    let file = File::open("15.txt").unwrap();
    const MAX: i64 = 4000000;

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

    let real_xmin = if x_min < 0 { 0i64 } else { x_min };
    let real_xmax = if x_max > MAX { MAX } else { x_max };
    let real_ymin = if y_min < 0 { 0i64 } else { y_min };
    let real_ymax = if y_max > MAX { MAX } else { y_max };

    let width = i64::abs(real_xmax - real_xmin) as usize;
    let height = i64::abs(real_ymax - real_ymin) as usize;
    let mut covered = BitVec::from_elem((width + 1) * (height + 1), false);

    for [x_sensor, y_sensor, _x_beacon, _y_beacon, dist_sensor_beacon] in &vectors {
        let x_min_search = x_sensor - dist_sensor_beacon;
        let x_max_search = x_sensor + dist_sensor_beacon;
        // if x_min_search > real_xmax || x_max_search < real_xmin {
        //     continue;
        // }
        let y_min_search = y_sensor - dist_sensor_beacon;
        let y_max_search = y_sensor + dist_sensor_beacon;
        // if y_min_search > real_ymax || y_max_search < real_ymin {
        //     continue;
        // }

        let x_min = if x_min_search < real_xmin {
            real_xmin
        } else {
            x_min_search
        };

        let x_max = if x_max_search > real_xmax {
            real_xmax
        } else {
            x_max_search
        };
        let y_min = if y_min_search < real_ymin {
            real_ymin
        } else {
            y_min_search
        };
        let y_max = if y_max_search > real_ymax {
            real_ymax
        } else {
            y_max_search
        };

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let dx = (x - real_xmin) as usize;
                let dy = (y - real_ymin) as usize;
                let pos = dy * width + dx;
                // println!(
                //       "[D001] real_xmin={} real_xmax={} real_ymin={} real_ymax={} pos={} x={} y={} dx={} dy={}",
                //       real_xmin, real_xmax, real_ymin, real_ymax, pos, x, y, dx, dy
                //   );
                let dist_sensor_point = i64::abs(x_sensor - x) + i64::abs(y_sensor - y);

                if dist_sensor_point <= *dist_sensor_beacon {
                    covered.set(pos, true);
                }
            }
        }
    }
    // println!("{:?}", covered);
    'outer: for y in real_ymin..=real_ymax {
        for x in real_xmin..=real_xmax {
            let dx = (x - real_xmin) as usize;
            let dy = (y - real_ymin) as usize;
            let pos = dy * width + dx;
            if covered.get(pos).unwrap() {
                continue;
            }

            println!("x={} y={}", x, y);
            break 'outer;
        }
    }

    // println!("{}", in_sensor_range_xs.iter().filter(|x| *x).count());
}
