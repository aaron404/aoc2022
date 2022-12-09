#![feature(iter_array_chunks)]

use std::collections::{HashMap, HashSet};

fn day_1() {
    // split input at empty lines
    let input = include_str!("1.txt").split("\r\n\r\n");

    // split list of calories into lines, map strings to integers, sum each list, and collect into a vector
    let mut sums = input.map(|cal_list| 
        cal_list.lines()
            .map(|cal_count| cal_count.parse::<u32>().expect("invalid input"))
            .sum::<u32>()
        ).collect::<Vec<u32>>();

    // reverse sort the vector (highest first)
    sums.sort_unstable_by(|a, b| b.cmp(a));

    // extract highest and sum of top 3
    let highest = sums.first().expect("invalid input");
    let sum_of_top3 = sums.iter().take(3).sum::<u32>();

    println!("Day  1: {: >8} {: >8}", highest, sum_of_top3);
}

fn day_2() {
    let input = include_str!("2.txt");

    let result1: u32 = input.lines().map(|line| 
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

    // part two
    let result2: u32 = input.lines().map(|line| 
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

    println!("Day  3: {: >8} {: >8?}", result1, result2);
}

fn day_3() {
    let input = include_str!("3.txt");

    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let result1: i32 = input.lines().map(|line| {
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

    let result2: i32 = input.lines().collect::<Vec<&str>>()
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
    println!("Day  3: {: >8} {: >8?}", result1, result2);
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

    let count1 = input
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


    let count2 = input
    .lines()
    .filter(|line| {
        let assignments = line.split(",").collect::<Vec<&str>>();
        let a = assignments[0].split("-").map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let b = assignments[1].split("-").map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let r1 = IntRange {min: a[0], max: a[1]};
        let r2 = IntRange {min: b[0], max: b[1]};
        range_overlaps(&r1, &r2) || range_overlaps(&r2, &r1)
    })
    .count();

    println!("Day  4: {: >8} {: >8?}", count1, count2);
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
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }

    for i in (0..max_height-1).rev() {
        let line = crates[i];
        line.chars().collect::<Vec<char>>().chunks(4)
            .map(|c| c[1])
            .enumerate()
            .for_each(|f| {
                if f.1 != ' ' {
                    stacks[f.0].push(f.1);
                }
            });
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
            let mut tmp: Vec<char> = Vec::new();
            for _ in 0..count {
                let c = stacks[from].pop().unwrap();
                tmp.push(c);
            }
            for _ in 0..count {
                stacks[to].push(tmp.pop().unwrap());
            }
        });
    
    for stack in stacks.iter() {
        println!("{:?}", stack);
    }

    println!("Day  5: {: >8} {: >8?}", -1, -1);
}

// create a bitmask of ascii codes, then count the ones
fn msg_is_unique(msg: &[u8]) -> bool {
    let mask = msg.iter().fold(0u128, |acc, &x| acc | (1 << x));
    mask.count_ones() as usize == msg.len()
}

fn get_first_unique_n(data: &[u8], n: usize) -> usize {
    data.windows(n)
        .enumerate()
        .filter(|window| msg_is_unique(window.1))
        .nth(0)
        .expect("Error parsing input").0 + n
}

fn day_6() {
    const START_OF_PACKET_SIZE: usize = 4;
    const START_OF_MSG_SIZE: usize = 14;

    let input = include_str!("6.txt");

    let data = input.as_bytes();

    let first_packet_offset = get_first_unique_n(data, START_OF_PACKET_SIZE);
    let first_msg_offset = get_first_unique_n(data, START_OF_MSG_SIZE);

    println!("Day  6: {: >8} {: >8?}", first_packet_offset, first_msg_offset);
}

struct Dir {
    dirs: HashMap<String, Dir>,
    files: HashMap<String, usize>,
}

impl Dir {
    fn new() -> Self {
        Dir {
            dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn add_file(&mut self, path: &Vec<&str>, name: String, size: usize) {
        self.get_dir(path).files.insert(name, size);
    }

    fn get_dir(&mut self, path: &Vec<&str>) -> &mut Dir {
        let cwd = self;
        path.iter()
            .map(|name| name.to_string())
            .fold(cwd, |acc, f| {
                if !acc.dirs.contains_key(&f) {
                    acc.dirs.insert(f.clone(), Dir::new());
                }
                acc.dirs.get_mut(&f).unwrap()
            })
    }

    fn get_size(&self) -> usize {
        let mut sum: usize = 0;
        for file in self.files.iter() {
            sum += file.1;
        }
        for dir in self.dirs.iter() {
            sum += dir.1.get_size();
        }
        sum
    }

    fn get_dirs_smaller_than_n(&self, max_size: usize, sizes: &mut Vec<usize>) -> usize {
        let mut sum: usize = 0;
        for file in self.files.iter() {
            sum += file.1;
        }
        for dir in self.dirs.iter() {
            sum += dir.1.get_dirs_smaller_than_n(max_size, sizes);
        }
        if sum <= max_size {
            sizes.push(sum);
        }
        sum
    }

    fn find_smallest_dir_above_n(&self, min_size: usize, result: &mut usize) {
        for dir in self.dirs.iter() {
            dir.1.find_smallest_dir_above_n(min_size, result);
            let s = dir.1.get_size();
            if s > min_size && s < *result {
                *result = s;
            }
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("--- FS ---");
        self.print_recursive(0);
    }

    fn print_recursive(&self, depth: usize) {
        for dir in self.dirs.iter() {
            for _ in 0..depth * 2 {
                print!(" ");
            }
            println!("- dir {}", dir.0);
            dir.1.print_recursive(depth + 1);
        }
        for file in self.files.iter() {
            for _ in 0..depth * 2 {
                print!(" ");
            }
            println!("- {: <16} {: <8}", file.0, file.1); 
        }
    }
}

fn day_7() {
    let input = include_str!("7.txt");

    let mut fs = Dir::new();

    let mut cwd: Vec<&str> = Vec::new();

    fs.dirs.insert("/".to_string(), Dir::new());

    cwd.push("/");

    input.lines().for_each(|line| {
        if line.starts_with("$ cd") {
            let new_dir = line.split("cd ").last().expect("cd missing arg");
            match new_dir {
                "/" => { cwd.clear(); cwd.push("/") },
                ".." => _ = cwd.pop(),
                _ => cwd.push(new_dir),
            }
        } else if line.starts_with("$ ls") {
            // list beginning
        } else {
            let mut entry = line.split(" ");
            let first = entry.nth(0).unwrap().clone();
            let last = entry.last().unwrap().clone();
            if line.starts_with("dir") {

            } else {
                fs.add_file(&cwd, last.to_string(), first.parse::<usize>().unwrap());
            }
        }
    });

    let mut sizes = Vec::new();
    fs.get_dirs_smaller_than_n(100000, &mut sizes);
    let part1 = sizes.iter().sum::<usize>();

    // fs.print();

    sizes.clear();
    let size_to_clear = 30000000 - (70000000 - fs.get_size());

    // println!("space to clear: {size_to_clear}");
    let mut result: usize = usize::MAX;
    fs.find_smallest_dir_above_n(size_to_clear, &mut result);

    println!("Day  7: {: >8} {: >8?}", part1, result);
}

fn day_8() {
    let input = include_str!("8.txt");

    let mut trees = input.lines()
        .map(|line| line.bytes().map(|b| (b as i8 - 48, false)).collect::<Vec<(i8, bool)>>())
        .collect::<Vec<Vec<(i8, bool)>>>();
    
    let w = trees[0].len();
    let h = trees.len();

    // set borders to visible
    for i in 0..w {
        trees[0][i].1 = true;
        trees[h-1][i].1 = true;
    }

    for i in 0..h {
        trees[i][0].1 = true;
        trees[i][h-1].1 = true;
    }

    // from left side
    for y in 0..h {
        let mut max: i8 = -1;
        let mut x = 0;
        loop {
            if trees[y][x].0 > max {
                // println!("{x},{y} visible");
                trees[y][x].1 = true;
                max = trees[y][x].0;
            }
            x += 1;
            if x == w {
                break;
            }
        }
    }

    // from top side
    for x in 0..w {
        let mut max: i8 = -1;
        let mut y = 0;
        loop {
            if trees[y][x].0 > max {
                // println!("{x},{y} visible");
                trees[y][x].1 = true;
                max = trees[y][x].0;
            }
            y += 1;
            if y == h {
                break;
            }
        }
    }

    // from right side
    for y in (0..h).rev() {
        let mut max: i8 = -1;
        let mut x = w - 1;
        loop {
            if trees[y][x].0 > max {
                // println!("{x},{y} visible");
                trees[y][x].1 = true;
                max = trees[y][x].0;
            }
            if x == 0 {
                break;
            }
            x -= 1;
        }
    }

    // from bottom side
    for x in (0..w).rev() {
        let mut max: i8 = -1;
        let mut y = h - 1;
        loop {
            if trees[y][x].0 > max {
                // println!("{x},{y} visible");
                trees[y][x].1 = true;
                max = trees[y][x].0;
            }
            if y == 0 {
                break;
            }
            y -= 1;
        }
    }

    let mut count = 0;
    for y in 0..h {
        for x in 0..w {
            if trees[y][x].1 {
                count += 1;
            }
        }
    }
    
    let mut best_score = 0;
    for y in 0..h {
        for x in 0..w {
            let z = trees[y][x].0; // current tree height

            // look right
            let mut r_score = 0;
            for nx in x+1..w {
                let nz = trees[y][nx].0;
                // println!("  nx, y, nz: {} {} {}", nx, y, nz);
                r_score += 1;
                if nz >= z {
                    // println!("  breaking");
                    break;
                }
            }

            // look left
            let mut l_score = 0;
            if x > 0 {
                for nx in (0..x).rev() {
                    let nz = trees[y][nx].0;
                    // println!("  nx, y, nz: {} {} {}", nx, y, nz);
                    l_score += 1;
                    if nz >= z {
                        // println!("  breaking");
                        break;
                    }
                }
            }

            // look down
            let mut d_score = 0;
            for ny in y+1..h {
                let nz = trees[ny][x].0;
                d_score += 1;
                if nz >= z {
                    break;
                }
            }

            // look up
            let mut u_score = 0;
            if y > 0 {
                for ny in (0..y).rev() {
                    let nz = trees[ny][x].0;
                    u_score += 1;
                    if nz >= z {
                        break;
                    }
                }
            }

            let score = r_score * l_score * u_score * d_score;
            if score > best_score {
                best_score = score;
            }
        }
    }
    
    println!("Day  8: {: >8} {: >8?}", count, best_score);
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Point(i32, i32);

impl Point {
    fn move_dir(&mut self, dir: Direction) {
        match dir {
            Direction::Left => self.0 -= 1,
            Direction::Up => self.1 += 1,
            Direction::Right => self.0 += 1,
            Direction::Down => self.1 -= 1,
        }
    }

    fn move_offset(&mut self, x: i32, y: i32) {
        self.0 += x;
        self.1 += y;
    }
}

struct Rope {
    segments: Vec<Point>,
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Rope {
    fn new(n: usize) -> Self {
        assert!(n >= 2);
        Rope {
            segments: vec![Point(0, 0); n],
        }
    }

    fn move_head(&mut self, dir: Direction) -> Point {

        self.segments[0].move_dir(dir);

        for i in 0..self.segments.len() - 1 {
            // i is front knot, i+1 is next knot
            let dx = self.segments[i].0 - self.segments[i+1].0;
            let dy = self.segments[i].1 - self.segments[i+1].1;

            if dx.abs() > 1 && dy.abs() > 1 {
                self.segments[i+1].move_offset(dx / 2, dy / 2);
            } else {
                if dx.abs() > 1 {
                    self.segments[i+1].move_offset(dx / 2, dy);
                }
    
                if dy.abs() > 1 {
                    self.segments[i+1].move_offset(dx, dy / 2);
                }
            }
        }

        // there must be a last because we assert n >= 2 at creation
        *self.segments.last().unwrap()
    }
}

fn day_9() {
    let input = include_str!("9.txt");

    let mut ropes = vec![Rope::new(2), Rope::new(10)];

    let solutions = ropes.iter_mut()
        .map(|rope| {
        let mut visited = HashSet::new();
        input.lines().for_each(|line| {
            // first element is dir, second amount
            let mut split = line.split(" ");
            let dir = match split.next().expect("empty line") {
                "L" => Direction::Left,
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                _ => panic!("invalid input"),
            };
            let count = split.next().expect("direction with no amount").parse::<usize>().expect("couldn't parse number");
            for _ in 0..count {
                visited.insert(rope.move_head(dir));
            }
        });
        visited.len()
    }).collect::<Vec<usize>>();
    
    
    println!("Day  9: {: >8} {: >8?}", solutions[0], solutions[1]);

}

fn main() {
    day_1();
    day_2();
    day_3();
    day_4();
    day_5();
    day_6();
    day_7();
    day_8();
    day_9();
}
