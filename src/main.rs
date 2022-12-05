#![feature(iter_array_chunks)]

fn day_1() {
    let input = include_str!("1.txt");

    let mut highest_vals = [0, 0, 0];
    let mut current_total = 0;
    input.lines().for_each(|line| {
        match line {
            "" => {
                if current_total > highest_vals[0] {
                    highest_vals[0] = current_total;
                    highest_vals.sort();
                };
                current_total = 0;
            },
            _ => {
                let current_val = line.parse::<u32>().unwrap();
                current_total += current_val;
            }
        }
    });
    if current_total > highest_vals[0] {
        highest_vals[0] = current_total;
        highest_vals.sort();
    };

    println!("highest: {}, sum of top 3: {}", highest_vals[2], highest_vals.iter().sum::<u32>());
}

fn day_2() {
    let input = include_str!("2.txt");

    let result: u32 = input.lines().map(|line| 
        match line {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => 0
        }
    ).sum();

    print!("sum: {result} ");
    // part two
    let result: u32 = input.lines().map(|line| 
        match line {
            "A X" => 0 + 3,
            "A Y" => 3 + 1,
            "A Z" => 6 + 2,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 0 + 2,
            "C Y" => 3 + 3,
            "C Z" => 6 + 1,
            _ => 0
        }
    ).sum();

    println!("{result}");
}

fn day_3() {
    let input = include_str!("3.txt");

    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let result: i32 = input.lines().map(|line| {
        let compartments = line.split_at(line.len() / 2);
        let m = letters.as_bytes().iter()
            .filter(|c| compartments.0.as_bytes().contains(c) && compartments.1.as_bytes().contains(c))
            .map(|i| {
                match i {
                    65..=90 => *i as i32 - 65 + 27,
                    97..=122 => *i as i32 - 96,
                    _ => 0 as i32,
                }
            })
            .sum::<i32>();
        m
    }).sum();
    print!("day 3: {result} ");

    let result: i32 = input.lines().collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            for char in letters.as_bytes() {
                if chunk.iter().all(|&x| x.as_bytes().contains(char)) {
                    return *char
                }
            }
            0
        }).map(|i| match i {
            65..=90 => i as i32 - 65 + 27,
            97..=122 => i as i32 - 96,
            _ => 0 as i32,
        })
        .sum::<i32>();
    println!("{result}");
}

struct IntRange {
    min: i32,
    max: i32,
}

fn range_contains(range1: &IntRange, range2: &IntRange) -> bool {
    range2.min >= range1.min && range2.max <= range1.max
}

fn range_overlaps(range1: &IntRange, range2: &IntRange) -> bool {
    (range2.min >= range1.min && range2.min <= range1.max) ||
    (range2.max >= range1.min && range2.max <= range1.max)
}

//84-93,30-85
fn day_4() {
    let input = include_str!("4.txt");

    let count = input
        .lines()
        .filter(|line| {
            let assignments = line.split(",").collect::<Vec<&str>>();
            let a = assignments[0].split("-").map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let b = assignments[1].split("-").map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let r1 = IntRange {min: a[0], max: a[1]};
            let r2 = IntRange {min: b[0], max: b[1]};
            range_contains(&r1, &r2) || range_contains(&r2, &r1)
        })
        .count();

    println!("count: {count}");

    let count = input
    .lines()
    .filter(|line| {
        let assignments = line.split(",").collect::<Vec<&str>>();
        let a = assignments[0].split("-").map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let b = assignments[1].split("-").map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let r1 = IntRange {min: a[0], max: a[1]};
        let r2 = IntRange {min: b[0], max: b[1]};
        println!("ranges: {} {} {} {} ", a[0], a[1], b[0], b[1]);
        range_overlaps(&r1, &r2) || range_overlaps(&r2, &r1)
    })
    .count();
    println!("count: {count}");

}

fn day_5() {
    let input = include_str!("5.txt");

    let mut split = input.split("\r\n\r\n");
    let (crates, moves) = (split.next().unwrap().lines().collect::<Vec<&str>>(), split.next().unwrap().lines());

    let max_height = crates.len();

    let num_stacks = crates
        .last().unwrap()
        .split_ascii_whitespace().count();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for i in 0..num_stacks {
        stacks.push(Vec::new());
    }

    for i in (0..max_height-1).rev() {
        let line = crates[i];
        println!("line {}: {}", i, line);
        line.chars().collect::<Vec<char>>().chunks(4)
            .map(|c| c[1])
            .enumerate()
            .for_each(|f| {
                if f.1 != ' ' {
                    stacks[f.0].push(f.1);
                }
            });
    }

    for stack in stacks.iter() {
        println!("{:?}", stack);
    }

    let m: Vec<Vec<usize>> = moves
        .map(|line| {
            let nums = line.split_ascii_whitespace()
                .skip(1)
                .step_by(2)
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>();
            nums
        }).collect();
    
        // m.iter().for_each(|m| {
        //     let count = m[0];
        //     let from  = m[1] - 1;
        //     let to    = m[2] - 1;
        //     println!("{count}, {from}, {to}");
        //     for i in 0..count {
        //         let tmp = stacks[from].pop().unwrap();
        //         stacks[to].push(tmp);
        //     }
        // });
        
        m.iter().for_each(|m| {
            let count = m[0];
            let from  = m[1] - 1;
            let to    = m[2] - 1;
            println!("{count}, {from}, {to}");
            let mut tmp: Vec<char> = Vec::new();
            for i in 0..count {
                let c = stacks[from].pop().unwrap();
                tmp.push(c);
            }
            for i in 0..count {
                stacks[to].push(tmp.pop().unwrap());
            }
        });
    
    for stack in stacks.iter() {
        println!("{:?}", stack);
    }

    

    println!("crates:");
    // println!("{}", crates);
    println!("num &mut : {num_stacks}")
}

fn main() {
    day_1();
    day_2();
    day_3();
    day_4();
    day_5();
}
