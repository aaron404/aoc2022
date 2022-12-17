

// sand poured from here
const START_X: usize = 500;
const START_Y: usize = 0;

// candidate relative movements for a falling piece of sand
const MOVES: [(isize, isize); 3] = [(0, 1), (-1, 1), (1, 1)];

struct UPoint(usize, usize);

#[derive(Copy, Clone, PartialEq)]
enum CaveState {
    Air,
    Wall,
    Sand,
}

struct Cave {
    width: usize,
    height: usize,
    states: Vec<Vec<CaveState>>,
}

impl Cave {
    fn new(width: usize, height: usize) -> Self {
        Cave {
            width,
            height,
            states: vec![vec![CaveState::Air; width]; height],
        }
    }

    fn from_str(s: &str) -> Self {
        // parse input into a list of lists of coordinates, finding the bounding box as well
        let mut y_max = usize::MIN;
        let paths = s.lines()
            .map(|line| {
                line.split(" -> ")
                .map(|point| {
                    let mut p = point.split(',')
                        // .map_while(|num| num.parse::<usize>().ok());
                        .map(|num| num.parse::<usize>().expect("failed to parse num"));
                    let point: [usize; 2] = p.next_chunk().expect("coordinate with less than two values");
                    y_max = usize::max(y_max, point[1]);
                    UPoint(point[0], point[1])
                })
                .collect::<Vec<UPoint>>()
            })
            .collect::<Vec<Vec<UPoint>>>();
        
        // y=0 represents top of cave, from where sand falls
        let y_min = 0;
        let y_max = paths.iter()
            .flatten()
            .fold(0, |acc, x| usize::max(acc, x.1));
        let height = y_max - y_min + 3;

        // sand will pile up in a huge triangle, with width = 2 * height
        let x_min = START_X - height;
        let x_max = START_X + height;
        let width = x_max - x_min + 1;

        let mut cave = Cave {
            width: width,
            height: height,
            states: vec![vec![CaveState::Air; width]; height],
        };

        paths.iter().for_each(|path| {
            path.windows(2).for_each(|segment| {
                let dx = usize::abs_diff(segment[0].0, segment[1].0);
                let dy = usize::abs_diff(segment[0].1, segment[1].1);
                let x0 = usize::min(segment[0].0, segment[1].0) - x_min;
                let y0 = usize::min(segment[0].1, segment[1].1);
                if dx > 0 {
                    (x0..=x0+dx).for_each(|x| cave.states[y0][x] = CaveState::Wall);
                } else {
                    (y0..=y0+dy).for_each(|y| cave.states[y][x0] = CaveState::Wall);
    
                }
            })
        });

        cave
    }

    fn with_floor(mut self) -> Self {
        self.states.last_mut().expect("cave is empty")
            .iter_mut().for_each(|state| *state = CaveState::Wall);
        self
    }

    fn pour_sand(&mut self) -> usize {
        let start_x = self.height;
        
        let mut count = 0;

        let mut done = false;
        'pour: while !done {
            if self.states[0][start_x] == CaveState::Sand {
                break;
            }
            let mut sand = (start_x, 0usize);
            
            let mut settled = false;
            while !settled {
                let mut moved = false;
                for m in MOVES.iter() {
                    let nx = sand.0.wrapping_add_signed(m.0);
                    let ny = sand.1.wrapping_add_signed(m.1);
                    if ny >= self.height {
                        // fell into void
                        break 'pour;
                    }
                    if nx < self.width {
                        // sand was able to move successfuly
                        if self.states[ny][nx] == CaveState::Air {
                            sand = (nx, ny);
                            moved = true;
                            break;
                        }
                    } else {
                        // sand has gone out of bounds
                        println!("out of bounds: {:?}", sand);
                        done = true;
                        break;
                    }
                }
                if !moved {
                    self.states[sand.1][sand.0] = CaveState::Sand;
                    settled = true;
                    count += 1;
                }
            }
        }
        count
    }

    fn print(&self) {
        // print cave
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", match self.states[y][x] {
                    CaveState::Wall => '#',
                    CaveState::Sand => 'O',
                    CaveState::Air => ' ',
                });
            }
            println!("");
        }
    }
}

pub fn solve() {
    let part1 = Cave::from_str(include_str!("input/14.txt")).pour_sand();
    let part2 = Cave::from_str(include_str!("input/14.txt")).with_floor().pour_sand();

    println!("Day 14: {: >10} {: >10}", part1, part2);
}