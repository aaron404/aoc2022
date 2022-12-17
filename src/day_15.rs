
const TEST_Y: i32 = 10;

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    bx: i32,
    by: i32,
    dist: u32,
}

impl Sensor {
    fn from_vals(vals: &[i32; 4]) -> Self {
        let dist = i32::abs_diff(vals[0], vals[2]) + i32::abs_diff(vals[1], vals[3]);
        Sensor {
            x: vals[0],
            y: vals[1],
            bx: vals[2],
            by: vals[3],
            dist,
        }
    }
}

pub fn solve() {

    let sensors = include_str!("input/15_test.txt").lines()
        .map(|line| line.chars()
            .filter(|c| match c {
                '0'..='9' => true,
                '-' => true,
                ' ' => true,
                _ => false,
            })
            .collect::<String>()
            .split_ascii_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .next_chunk::<4>()
            .map(|vals| Sensor::from_vals(&vals)).unwrap()
        ).collect::<Vec<Sensor>>();
    
    let max_dist = sensors.iter().fold(u32::MIN, |acc, x| u32::max(acc, x.dist)) as i32;
    let max_x = sensors.iter().fold(i32::MIN, |acc, x| i32::max(acc, x.x)) + max_dist;
    let min_x = sensors.iter().fold(i32::MAX, |acc, x| i32::min(acc, x.x)) - max_dist;

    // let min_x = i32::max(min_x, 0);
    // let max_x = i32::min(max_x, 4000000);
    println!("max_dist: {max_dist}, min/max x: {min_x}, {max_x}");


    let mut count = 0;
    // for y in 0..20 {
        // print!("{y: >3}");
        for x in min_x..=max_x {
            let mut reachable = false;
            let mut on_sensor = false;
            for sensor in sensors.iter() {
                let dist = i32::abs_diff(x, sensor.x) + i32::abs_diff(TEST_Y, sensor.y);
                if dist == 0 {
                    on_sensor = true;
                }
                if dist <= sensor.dist {
                    if sensors.iter().any(|sensor| sensor.bx == x && sensor.by == TEST_Y) {
                        break;
                    }
                    count += 1;
                    reachable = true;
                    break;
                }
            }
            if on_sensor {
                // print!("S");
            } else {
                if reachable {
                    // print!("#");
                } else {
                    println!("{x}, {TEST_Y}");
                    println!("freq: {}", x as i128 * 4000000 + TEST_Y as i128);
                    // print!(" ");
                }
            }

        }
        println!("");
    // }


    println!("{count}");
    
    
    // for sensor in sensors {
    //     println!("{sensor:?}");
    // }

    

}