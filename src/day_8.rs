pub fn solve() {
    let input = include_str!("input/8.txt");

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
    
    println!("Day  8: {: >10} {: >10}", count, best_score);
}