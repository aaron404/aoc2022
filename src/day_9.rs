use std::collections::HashSet;

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

pub fn solve() {
    let input = include_str!("input/9.txt");

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
    
    
    println!("Day  9: {: >10} {: >10}", solutions[0], solutions[1]);

}