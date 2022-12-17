const NEIGHBOR_DELTAS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Clone, Copy)]
struct UPoint(usize, usize);

pub fn solve() {
    const S_ASCII: u8 = 83;
    const E_ASCII: u8 = 69;
    let input = include_str!("input/12_test.txt");

    let mut start: Option<UPoint> = None;
    let mut end: Option<UPoint> = None;

    // map start as 255, end as 0, a-z as 0-25
    let terrain = input.lines().enumerate().map(|(y, line)| {
        line.bytes().enumerate().map(|(x, b)| {
            match b {
                S_ASCII => {
                    start = Some(UPoint(x, y));
                    0
                },
                E_ASCII => {
                    end = Some(UPoint(x, y));
                    25
                }
                _ => b as i8 - 97,
            }
        }).collect::<Vec<i8>>()
    }).collect::<Vec<Vec<i8>>>();

    let start = start.expect("Failed to parse start point");
    let end = end.expect("Failed to parse end point");

    let height = terrain.len();
    let width = terrain[0].len();

    // let mut visited = vec![vec![false; width]; height];
    let mut parents: Vec<Vec<Option<UPoint>>> = vec![vec![None; width]; height];
    let mut dists = vec![vec![i16::MAX; width]; height];
    dists[end.1 as usize][end.0 as usize] = 0;

    println!("w, h: {width}, {height}");
    println!("Start: {},{}, End: {},{}", start.0, start.1, end.0, end.1);

    let mut open_cells: Vec<UPoint> = Vec::new();
    open_cells.push(end);

    // parents[end.1 as usize][end.0 as usize] = Some(end);

    // let mut found_start = false;

    println!("asdf {}", (0 as usize).wrapping_add_signed(-1));

    while !open_cells.is_empty() {
        // necessarily Some(), due to above condition
        let cell = open_cells.pop().unwrap();
        let UPoint(cx, cy) = cell;
        let curr_height = terrain[cy][cx];
        let curr_dist = dists[cy][cx];

        // visited[cy][cx] = true;
        for offset in NEIGHBOR_DELTAS {
            let nx = cell.0.wrapping_add_signed(offset.0);
            let ny = cell.1.wrapping_add_signed(offset.1);
            // boundary check new cell
            if nx < width && ny < height {

                // check if current cell would have been accessible from new cell
                if curr_height - terrain[ny][nx] <= 1 {

                    // if new cell was already visited, check if its score can be improved
                    if dists[ny][nx] > curr_dist + 1 {
                        dists[ny][nx] = curr_dist + 1;
                        parents[ny][nx] = Some(cell);
                        open_cells.push(UPoint(nx, ny));
                    }
                }
 
            }


            // if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
            //     let dist_other = dists[ny as usize][nx as usize];
            //     if dist_other > dists[cy][cx] + 1 {
            //         // if terrain[ny as usize][nx as usize] == i8::MAX {
            //         //     parents[ny as usize][nx as usize] = cell;
            //         // }
            //         // can only have come from a cell at most 1 (it could be negative)
            //         let dz = curr_height - terrain[ny as usize][nx as usize];
            //         if dz <= 1 {
            //             // visited[ny as usize][nx as usize] = true;
            //             parents[ny as usize][nx as usize] = Some(cell);
            //             dists[ny as usize][nx as usize] = dists[cy][cx] + 1;
            //             open_cells.push(Point(nx, ny));
            //         }
            //     }
            // }
        }
    }

    // for y in 0..height {
    //     for x in 0..width {
    //         let c = match dists[y][x] {
    //             0..=1000 => (dists[y][x] % 26 + 97) as u8 as char,
    //             _ => '#',
    //         };
    //         print!("{}", c);
    //     }
    //     println!("");
    // }

    // let mut path = vec![vec![' '; width]; height];
    let mut count = 0;
    // let mut cx = start.0;
    // let mut cy = start.1;
    // println!("path start: {cx} {cy}");

    let mut cell = Some(start);
    while cell.is_some() {
        count += 1;
        cell = parents[cell.unwrap().1][cell.unwrap().0];
    }

    // while terrain[cy][cx] != 0  {
    //     path[cy][cx] = '#';
    //     if let Some(parent) = parents[cy][cx] {

    //         cx = parent.0 as usize;
    //         cy = parent.1 as usize;
    //         count += 1;

    //         if cx == end.0 as usize && cy == end.1 as usize {
    //             break;
    //         }
    //         print!("{},{}  ", cx, cy);
    //     } else {
    //         break;
    //     }
    // }

    println!("count: {count}");

    // let mut count = 0;
    // for y in 0..height {
    //     for x in 0..width {
    //         print!("{}", path[y][x]);
    //         if path[y][x] == '#' {
    //             count += 1;
    //         }
    //     }
    //     println!("");
    // }

    // println!("count: {count}");

    // for y in 0..height {
    //     for x in 0..width {
    //         print!("{}", if parents[y][x].is_none() { '#' } else { ' ' });
    //     }
    //     println!("");
    // }




}